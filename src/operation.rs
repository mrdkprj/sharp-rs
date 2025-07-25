use crate::{
    common::{is16_bit, remove_alpha, stay_sequential},
    input::Raw,
    Colour,
};
use libvips::{
    error::Error::OperationError,
    operations::{BandFormat, Extend, Interpretation, OperationBoolean, OperationMorphology, OperationRelational, Precision},
    operator::{Ge, Lt, MyIndex},
    v_value,
    voption::VOption,
    Result, VipsImage,
};
use std::slice;
use strum_macros::Display;

#[derive(Debug, Clone)]
pub struct AffineOptions {
    /** Parsed by the color module to extract values for red, green, blue and alpha. (optional, default "#000000") */
    pub background: Option<Colour>,
    /** Input horizontal offset (optional, default 0) */
    pub idx: Option<f64>,
    /** Input vertical offset (optional, default 0) */
    pub idy: Option<f64>,
    /** Output horizontal offset (optional, default 0) */
    pub odx: Option<f64>,
    /** Output horizontal offset (optional, default 0) */
    pub ody: Option<f64>,
    /** Interpolator (optional, default sharp.interpolators.bicubic) */
    pub interpolator: Option<Interpolators>,
}

#[derive(Display, Debug, Clone)]
pub enum Interpolators {
    /** [Nearest neighbour interpolation](http://en.wikipedia.org/wiki/Nearest-neighbor_interpolation). Suitable for image enlargement only. */
    #[strum(to_string = "nearest")]
    Nearest,
    /** [Bilinear interpolation](http://en.wikipedia.org/wiki/Bilinear_interpolation). Faster than bicubic but with less smooth results. */
    #[strum(to_string = "bilinear")]
    Bilinear,
    /** [Bicubic interpolation](http://en.wikipedia.org/wiki/Bicubic_interpolation) (the default). */
    #[strum(to_string = "bicubic")]
    Bicubic,
    /**
     * [LBB interpolation](https://github.com/libvips/libvips/blob/master/libvips/resample/lbb.cpp#L100).
     * Prevents some "[acutance](http://en.wikipedia.org/wiki/Acutance)" but typically reduces performance by a factor of 2.
     */
    #[strum(to_string = "lbb")]
    LocallyBoundedBicubic,
    /** [Nohalo interpolation](http://eprints.soton.ac.uk/268086/). Prevents acutance but typically reduces performance by a factor of 3. */
    #[strum(to_string = "nohalo")]
    Nohalo,
    /** [VSQBS interpolation](https://github.com/libvips/libvips/blob/master/libvips/resample/vsqbs.cpp#L48). Prevents "staircasing" when enlarging. */
    #[strum(to_string = "vsqbs")]
    VertexSplitQuadraticBasisSpline,
}

#[derive(Debug, Clone)]
pub struct SharpenOptions {
    /** The sigma of the Gaussian mask, where sigma = 1 + radius / 2, between 0.000001 and 10000 */
    pub sigma: f64,
    /** The level of sharpening to apply to "flat" areas, between 0 and 1000000 (optional, default 1.0) */
    pub m1: Option<f64>,
    /** The level of sharpening to apply to "jagged" areas, between 0 and 1000000 (optional, default 2.0) */
    pub m2: Option<f64>,
    /** Threshold between "flat" and "jagged", between 0 and 1000000 (optional, default 2.0) */
    pub x1: Option<f64>,
    /** Maximum amount of brightening, between 0 and 1000000 (optional, default 10.0) */
    pub y2: Option<f64>,
    /** Maximum amount of darkening, between 0 and 1000000 (optional, default 20.0) */
    pub y3: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct BlurOptions {
    /** A value between 0.3 and 1000 representing the sigma of the Gaussian mask, where `sigma = 1 + radius / 2` */
    pub sigma: f64,
    /** A value between 0.001 and 1. A smaller value will generate a larger, more accurate mask. */
    pub min_amplitude: Option<f64>,
    /** How accurate the operation should be, one of: integer, float, approximate. (optional, default "integer") */
    pub precision: Option<Precision>,
}

#[derive(Debug, Clone)]
pub struct FlattenOptions {
    /** background colour, parsed by the color module, defaults to black. (optional, default {r:0,g:0,b:0}) */
    pub background: Option<Colour>,
}

#[derive(Debug, Clone)]
pub struct NegateOptions {
    /** whether or not to negate any alpha channel. (optional, default true) */
    pub alpha: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct NormaliseOptions {
    /** Percentile below which luminance values will be underexposed. */
    pub lower: Option<i32>,
    /** Percentile above which luminance values will be overexposed. */
    pub upper: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct ClaheOptions {
    /** width of the region */
    pub width: i32,
    /** height of the region */
    pub height: i32,
    /** max slope of the cumulative contrast. A value of 0 disables contrast limiting. Valid values are integers in the range 0-100 (inclusive) (optional, default 3) */
    pub max_slope: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct KernelOptions {
    /** width of the kernel in pixels. */
    pub width: i32,
    /** height of the kernel in pixels. */
    pub height: i32,
    /** Array of length width*height containing the kernel values. */
    pub kernel: Vec<f64>,
    /** the scale of the kernel in pixels. (optional, default sum) */
    pub scale: Option<f64>,
    /** the offset of the kernel in pixels. (optional, default 0) */
    pub offset: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct ThresholdOptions {
    /** alternative spelling for greyscale. (optional, default true) */
    pub grayscale: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct BooleanOptions {
    pub raw: Raw,
}

#[derive(Debug, Clone)]
pub struct ModulateOptions {
    pub brightness: Option<f64>,
    pub saturation: Option<f64>,
    pub hue: Option<i32>,
    pub lightness: Option<f64>,
}

/*
 * Tint an image using the provided RGB.
 */
pub(crate) fn tint(image: VipsImage, tint: &[f64]) -> Result<VipsImage> {
    let tint_lab = (VipsImage::black(1, 1)? + tint).colourspace_with_opts(Interpretation::Lab, VOption::new().with("source_space", v_value!(Interpretation::Srgb as i32)))?.getpoint(0, 0)?;

    // LAB identity function
    let identity_lab = VipsImage::identity_with_opts(VOption::new().with("bands", v_value!(3)))?;
    let identity_lab = identity_lab.colourspace_with_opts(Interpretation::Lab, VOption::new().with("source_space", v_value!(Interpretation::Srgb as i32)))?;

    // Scale luminance range, 0.0 to 1.0
    let l = identity_lab.at(0) / 100.0;
    // Weighting functions
    let weight_l = 1.0 - 4.0 * ((&l - 0.5) * (&l - 0.5));
    let weight_ab = (weight_l * tint_lab.as_slice()).extract_band_with_opts(1, VOption::new().with("n", v_value!(2)))?;
    let identity_lab = identity_lab.at(0).bandjoin_with(&[weight_ab])?;

    // Convert lookup table to sRGB
    let lut = identity_lab.colourspace_with_opts(Interpretation::Srgb, VOption::new().with("source_space", v_value!(Interpretation::Lab as i32)))?;

    // Original colourspace
    let mut type_before_tint = image.get_interpretation()?;
    if type_before_tint == Interpretation::Rgb {
        type_before_tint = Interpretation::Srgb
    };

    // Apply lookup table
    let image = if image.image_hasalpha() {
        let alpha = image.at(image.get_bands() - 1);
        remove_alpha(image)?.colourspace(Interpretation::BW)?.maplut(&lut)?.colourspace(type_before_tint)?.bandjoin_with(&[alpha])?
    } else {
        image.colourspace(Interpretation::BW)?.maplut(&lut)?.colourspace(type_before_tint)?
    };

    Ok(image)
}

/*
 * Stretch luminance to cover full dynamic range.
 */
pub(crate) fn normalise(image: VipsImage, lower: f64, upper: f64) -> Result<VipsImage> {
    // Get original colourspace
    let mut type_before_normalize = image.get_interpretation()?;
    if type_before_normalize == Interpretation::Rgb {
        type_before_normalize = Interpretation::Srgb;
    }
    // Convert to LAB colourspace
    let lab = image.colourspace(Interpretation::Lab)?;
    // Extract luminance
    let luminance = lab.at(0);

    // Find luminance range
    let min: f64 = if lower == 0.0 {
        luminance.min()?
    } else {
        luminance.percent(lower)? as _
    };
    let max: f64 = if upper == 100.0 {
        luminance.max()?
    } else {
        luminance.percent(upper)? as _
    };

    if (max - min).abs() > 1.0 {
        // Extract chroma
        let chroma = lab.extract_band_with_opts(1, VOption::new().with("n", v_value!(2)))?;
        // Calculate multiplication factor and addition
        let f = 100.0 / (max - min);
        let a = -(min * f);
        // Scale luminance, join to chroma, convert back to original colourspace
        let normalized = luminance.linear(&[f], &[a])?.bandjoin_with(&[chroma])?.colourspace(type_before_normalize)?;
        // Attach original alpha channel, if any
        if image.image_hasalpha() {
            // Extract original alpha channel
            let alpha = image.at(image.get_bands() - 1);
            // Join alpha channel to normalised image
            return normalized.bandjoin_with(&[alpha]);
        } else {
            return Ok(normalized);
        }
    }

    Ok(image)
}

/*
 * Contrast limiting adapative histogram equalization (CLAHE)
 */
pub(crate) fn clahe(image: VipsImage, width: i32, height: i32, max_slope: i32) -> Result<VipsImage> {
    image.hist_local_with_opts(width, height, VOption::new().with("max_slope", v_value!(max_slope)))
}

/*
 * Gamma encoding/decoding
 */
pub(crate) fn gamma(image: VipsImage, exponent: f64) -> Result<VipsImage> {
    if image.image_hasalpha() {
        // Separate alpha channel
        let alpha = image.at(image.get_bands() - 1);
        remove_alpha(image)?.gamma_with_opts(VOption::new().with("exponent", v_value!(exponent)))?.bandjoin_with(&[alpha])
    } else {
        image.gamma_with_opts(VOption::new().with("exponent", v_value!(exponent)))
    }
}

/*
 * Flatten image to remove alpha channel
 */
pub(crate) fn flatten(image: VipsImage, flatten_background: &[f64]) -> Result<VipsImage> {
    let multiplier = if is16_bit(image.get_interpretation()?) {
        256.0
    } else {
        1.0
    };
    let background = [flatten_background[0] * multiplier, flatten_background[1] * multiplier, flatten_background[2] * multiplier];

    image.flatten_with_opts(VOption::new().with("background", v_value!(background.as_slice())))
}

/**
 * Produce the "negative" of the image.
 */
pub(crate) fn negate(image: VipsImage, negate_alpha: bool) -> Result<VipsImage> {
    if image.image_hasalpha() && !negate_alpha {
        // Separate alpha channel
        let alpha = image.at(image.get_bands() - 1);
        remove_alpha(image)?.invert()?.bandjoin_with(&[alpha])
    } else {
        image.invert()
    }
}

/*
 * Gaussian blur. Use sigma of -1.0 for fast blur.
 */
pub(crate) fn blur(image: VipsImage, sigma: f64, precision: Precision, min_ampl: f64) -> Result<VipsImage> {
    if sigma == -1.0 {
        // Fast, mild blur - averages neighbouring pixels
        let blur = VipsImage::new_matrixv(3, 3, &[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0])?;
        blur.set_double("scale", 9.0);
        image.conv(&blur)
    } else {
        // Slower, accurate Gaussian blur
        stay_sequential(image, true)?.gaussblur_with_opts(sigma, VOption::new().with("precision", v_value!(precision as i32)).with("min_ampl", v_value!(min_ampl)))
    }
}

/*
 * Convolution with a kernel.
 */
pub(crate) fn convolve(image: VipsImage, width: i32, height: i32, scale: f64, offset: f64, kernel_v: &[f64]) -> Result<VipsImage> {
    let bytes: &[u8] = unsafe { slice::from_raw_parts(kernel_v.as_ptr() as *const u8, std::mem::size_of_val(kernel_v)) };
    let kernel = VipsImage::new_from_memory(bytes, width, height, 1, BandFormat::Double)?;
    kernel.set_double("scale", scale);
    kernel.set_double("offset", offset);
    image.conv(&kernel)
}

/*
 * Recomb with a Matrix of the given bands/channel size.
 * Eg. RGB will be a 3x3 matrix.
 */
pub(crate) fn recomb(image: VipsImage, matrix: &[f64]) -> Result<VipsImage> {
    let image = image.colourspace(Interpretation::Srgb)?;
    if matrix.len() == 9 {
        let m = if image.get_bands() == 3 {
            VipsImage::new_matrix_from_array(3, 3, matrix)?
        } else {
            VipsImage::new_matrix_from_array(4, 4, &[matrix[0], matrix[1], matrix[2], 0.0, matrix[3], matrix[4], matrix[5], 0.0, matrix[6], matrix[7], matrix[8], 0.0, 0.0, 0.0, 0.0, 1.0])?
        };
        image.recomb(&m)
    } else {
        image.recomb(&VipsImage::new_matrix_from_array(4, 4, matrix)?)
    }
}

pub(crate) fn modulate(image: VipsImage, brightness: f64, saturation: f64, hue: i32, lightness: f64) -> Result<VipsImage> {
    let colourspace_before_modulate = image.get_interpretation()?;
    if image.image_hasalpha() {
        // Separate alpha channel
        let alpha = image.at(image.get_bands() - 1);
        remove_alpha(image)?.colourspace(Interpretation::Lch)?.linear(&[brightness, saturation, 1.0], &[lightness, 0.0, hue as _])?.colourspace(colourspace_before_modulate)?.bandjoin_with(&[alpha])
    } else {
        image.colourspace(Interpretation::Lch)?.linear(&[brightness, saturation, 1.0], &[lightness, 0.0, hue as _])?.colourspace(colourspace_before_modulate)
    }
}

/*
 * Sharpen flat and jagged areas. Use sigma of -1.0 for fast sharpen.
 */
pub(crate) fn sharpen(image: VipsImage, sigma: f64, m1: f64, m2: f64, x1: f64, y2: f64, y3: f64) -> Result<VipsImage> {
    if sigma == -1.0 {
        // Fast, mild sharpen
        let sharpen = VipsImage::new_matrix_from_array(3, 3, &[-1.0, -1.0, -1.0, -1.0, 32.0, -1.0, -1.0, -1.0, -1.0])?;
        sharpen.set_double("scale", 24.0);
        image.conv(&sharpen)
    } else {
        // Slow, accurate sharpen in LAB colour space, with control over flat vs jagged areas
        let mut colourspace_before_sharpen = image.get_interpretation()?;
        if colourspace_before_sharpen == Interpretation::Rgb {
            colourspace_before_sharpen = Interpretation::Srgb;
        }
        image
            .sharpen_with_opts(
                VOption::new().with("sigma", v_value!(sigma)).with("m1", v_value!(m1)).with("m2", v_value!(m2)).with("x1", v_value!(x1)).with("y2", v_value!(y2)).with("y3", v_value!(y3)),
            )?
            .colourspace(colourspace_before_sharpen)
    }
}

pub(crate) fn threshold(image: VipsImage, threshold: f64, threshold_grayscale: bool) -> Result<VipsImage> {
    if !threshold_grayscale {
        Ok(image.ge(threshold))
    } else {
        image.colourspace(Interpretation::BW)?.relational_const(OperationRelational::Moreeq, &[threshold])
    }
}

/*
  Perform boolean/bitwise operation on image color channels - results in one channel image
*/
pub(crate) fn bandbool(mut image: VipsImage, boolean: OperationBoolean) -> Result<VipsImage> {
    image = image.bandbool(boolean)?;
    image.copy_with_opts(VOption::new().with("interpretation", v_value!(Interpretation::BW as i32)))
}

/*
  Perform bitwise boolean operation between images
*/
pub(crate) fn boolean(image: VipsImage, image_r: &VipsImage, opration_boolean: OperationBoolean) -> Result<VipsImage> {
    image.boolean(image_r, opration_boolean)
}

/*
  Trim an image
*/
pub(crate) fn trim(image: VipsImage, background: &[f64], threshold: f64, line_art: bool) -> Result<VipsImage> {
    if image.get_width() < 3 && image.get_height() < 3 {
        return Err(OperationError("Image to trim must be at least 3x3 pixels"));
    }

    let mut background = background.to_vec();
    let mut threshold = threshold;
    if background.is_empty() {
        // Top-left pixel provides the default background colour if none is given
        background = image.extract_area(0, 0, 1, 1)?.getpoint(0, 0)?;
    } else if is16_bit(image.get_interpretation()?) {
        for color in &mut background {
            *color *= 256.0;
        }
        threshold *= 256.0;
    }

    let background_alpha = *background.last().unwrap();

    if image.image_hasalpha() {
        background.pop();
    } else {
        background.resize(image.get_bands() as _, 0.0);
    }

    let (left, top, width, height) =
        image.find_trim_with_opts(VOption::new().with("background", v_value!(background.as_slice())).with("line_art", v_value!(line_art)).with("threshold", v_value!(threshold)))?;

    if image.image_hasalpha() {
        // Search alpha channel (A)
        let alpha = image.at(image.get_bands() - 1);
        let (left_a, top_a, width_a, height_a) =
            alpha.find_trim_with_opts(VOption::new().with("background", v_value!(vec![background_alpha].as_slice())).with("line_art", v_value!(line_art)).with("threshold", v_value!(threshold)))?;

        if width_a > 0 && height_a > 0 {
            if width > 0 && height > 0 {
                // Combined bounding box (B)
                let left_b = std::cmp::min(left, left_a);
                let top_b = std::cmp::min(top, top_a);
                let width_b = std::cmp::max(left + width, left_a + width_a) - left_b;
                let height_b = std::cmp::max(top + height, top_a + height_a) - top_b;
                return image.extract_area(left_b, top_b, width_b, height_b);
            } else {
                // Use alpha only
                return image.extract_area(left_a, top_a, width_a, height_a);
            }
        }
    }

    if width > 0 && height > 0 {
        return image.extract_area(left, top, width, height);
    }

    Ok(image)
}

/*
 * Calculate (a * in + b)
 */
pub(crate) fn linear(image: VipsImage, a: &[f64], b: &[f64]) -> Result<VipsImage> {
    let bands = image.get_bands() as usize;
    if a.len() > bands {
        return Err(OperationError("Band expansion using linear is unsupported"));
    }

    let uchar = !is16_bit(image.get_interpretation()?);
    if image.image_hasalpha() && a.len() != bands && (a.len() == 1 || a.len() == bands - 1 || bands - 1 == 1) {
        // Separate alpha channel
        let alpha = image.at(image.get_bands() - 1);
        remove_alpha(image)?.linear_with_opts(a, b, VOption::new().with("uchar", v_value!(uchar)))?.bandjoin_with(&[alpha])
    } else {
        image.linear_with_opts(a, b, VOption::new().with("uchar", v_value!(uchar)))
    }
}

/*
 * Unflatten
 */
pub(crate) fn unflatten(image: VipsImage) -> Result<VipsImage> {
    if image.image_hasalpha() {
        let alpha = image.at(image.get_bands() - 1);
        let no_alpha = remove_alpha(image)?;
        no_alpha.bandjoin_with(&[alpha & (no_alpha.colourspace(Interpretation::BW)?.lt(255.0))])
    } else {
        image.colourspace(Interpretation::BW)?.bandjoin_with(&[image.colourspace(Interpretation::BW)?.lt(255.0)])
    }
}

/*
 * Ensure the image is in a given colourspace
 */
pub(crate) fn ensure_colourspace(image: VipsImage, colourspace: Interpretation) -> Result<VipsImage> {
    if colourspace != Interpretation::Last && image.get_interpretation()? != colourspace {
        return image.colourspace_with_opts(colourspace, VOption::new().with("source_space", v_value!(image.get_interpretation()? as i32)));
    }
    Ok(image)
}

/*
 * Split and crop each frame, reassemble, and update pageHeight.
 */
pub(crate) fn crop_multi_page(image: VipsImage, left: i32, top: i32, width: i32, height: i32, n_pages: i32, page_height: &mut i32) -> Result<VipsImage> {
    if top == 0 && height == *page_height {
        // Fast path; no need to adjust the height of the multi-page image
        image.extract_area(left, 0, width, image.get_height())
    } else {
        let mut pages: Vec<VipsImage> = Vec::new();

        // Split the image into cropped frames
        let image = stay_sequential(image, true)?;
        for i in 0..n_pages {
            pages.push(image.extract_area(left, *page_height * i + top, width, height)?);
        }

        // Reassemble the frames into a tall, thin image
        let assembled = VipsImage::arrayjoin_with_opts(pages.as_mut_slice(), VOption::new().with("across", v_value!(1)))?;

        // Update the page height
        *page_height = height;

        Ok(assembled)
    }
}

#[allow(clippy::too_many_arguments)]
/*
 * Split into frames, embed each frame, reassemble, and update pageHeight.
 */
pub(crate) fn embed_multi_page(image: VipsImage, left: i32, top: i32, width: i32, height: i32, extend_with: Extend, background: &[f64], n_pages: i32, page_height: &mut i32) -> Result<VipsImage> {
    if top == 0 && height == *page_height {
        // Fast path; no need to adjust the height of the multi-page image
        image.embed_with_opts(left, 0, width, image.get_height(), VOption::new().with("extend", v_value!(extend_with as i32)).with("background", v_value!(background)))
    } else if left == 0 && width == image.get_width() {
        // Fast path; no need to adjust the width of the multi-page image
        let mut pages: Vec<VipsImage> = Vec::new();

        // Rearrange the tall image into a vertical grid
        let image = image.grid(*page_height, n_pages, 1)?;

        // Do the embed on the wide image
        let image = image.embed_with_opts(0, top, image.get_width(), height, VOption::new().with("extend", v_value!(extend_with as i32)).with("background", v_value!(background)))?;

        // Split the wide image into frames
        for i in 0..n_pages {
            pages.push(image.extract_area(width * i, 0, width, height)?);
        }

        // Reassemble the frames into a tall, thin image
        let assembled = VipsImage::arrayjoin_with_opts(pages.as_slice(), VOption::new().with("across", v_value!(1)))?;

        // Update the page height
        *page_height = height;

        Ok(assembled)
    } else {
        let mut pages: Vec<VipsImage> = Vec::new();

        // Split the image into frames
        for i in 0..n_pages {
            pages.push(image.extract_area(0, *page_height * i, image.get_width(), *page_height)?);
        }

        // Embed each frame in the target size
        for page in pages.iter_mut().take(n_pages as usize) {
            *page = page.embed_with_opts(left, top, width, height, VOption::new().with("extend", v_value!(extend_with as i32)).with("background", v_value!(background)))?;
        }

        // Reassemble the frames into a tall, thin image
        let assembled = VipsImage::arrayjoin_with_opts(pages.as_mut_slice(), VOption::new().with("across", v_value!(1)))?;

        // Update the page height
        *page_height = height;

        Ok(assembled)
    }
}

/*
 * Dilate an image
 */
pub(crate) fn dilate(image: VipsImage, width: i32) -> Result<VipsImage> {
    let mask_width = 2 * width + 1;
    let mask = VipsImage::new_matrix(mask_width, mask_width)?;
    image.morph(&mask, OperationMorphology::Dilate)?.invert()
}

/*
 * Erode an image
 */
pub(crate) fn erode(image: VipsImage, width: i32) -> Result<VipsImage> {
    let mask_width = 2 * width + 1;
    let mask = VipsImage::new_matrix(mask_width, mask_width)?;
    image.morph(&mask, OperationMorphology::Erode)?.invert()
}
