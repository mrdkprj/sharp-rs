use crate::{
    common::{get_alpha_image, image_from_getpoint, is16_bit, remove_alpha, scalar_image_like, scale_image, stay_sequential},
    input::Raw,
    Colour,
};
use libvips::{
    error::Error::OperationError,
    ops::{BandFormat, Extend, Interpretation, Kernel, OperationBoolean, OperationMorphology, OperationRelational, Precision},
    v_value,
    voption::{VOption, V_Value},
    Result, VipsImage,
};
use num_derive::{FromPrimitive, ToPrimitive};
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

#[derive(Debug, Clone, Default)]
pub struct ResizeOptions {
    /** Alternative means of specifying width. If both are present self takes priority. */
    pub width: Option<i32>,
    /** Alternative means of specifying height. If both are present self takes priority. */
    pub height: Option<i32>,
    /** How the image should be resized to fit both provided dimensions, one of cover, contain, fill, inside or outside. (optional, default "cover") */
    pub fit: Option<Fit>,
    /** Position, gravity or strategy to use when fit is cover or contain. (optional, default "centre") */
    pub position: Option<Position>,
    /** Background colour when using a fit of contain, parsed by the color module, defaults to black without transparency. (optional, default {r:0,g:0,b:0,alpha:1}) */
    pub background: Option<Colour>,
    /** The kernel to use for image reduction. (optional, default "lanczos3") */
    pub kernel: Option<Kernel>,
    /** Do not enlarge if the width or height are already less than the specified dimensions, equivalent to GraphicsMagick"s > geometry option. (optional, default false) */
    pub without_enlargement: Option<bool>,
    /** Do not reduce if the width or height are already greater than the specified dimensions, equivalent to GraphicsMagick"s < geometry option. (optional, default false) */
    pub without_reduction: Option<bool>,
    /** Take greater advantage of the JPEG and WebP shrink-on-load feature, which can lead to a slight moiré pattern on some images. (optional, default true) */
    pub fast_shrink_on_load: Option<bool>,
}

#[derive(Default, Debug, Clone)]
pub struct ExtendOptions {
    /** single pixel count to top edge (optional, default 0) */
    pub top: Option<i32>,
    /** single pixel count to left edge (optional, default 0) */
    pub left: Option<i32>,
    /** single pixel count to bottom edge (optional, default 0) */
    pub bottom: Option<i32>,
    /** single pixel count to right edge (optional, default 0) */
    pub right: Option<i32>,
    /** background colour, parsed by the color module, defaults to black without transparency. (optional, default {r:0,g:0,b:0,alpha:1}) */
    pub background: Option<Colour>,
    /** how the extension is done, one of: "background", "copy", "repeat", "mirror" (optional, default `'background'`) */
    pub extend_with: Option<Extend>,
}

#[derive(Debug, Clone)]
pub enum Fit {
    Contain,
    Cover,
    Fill,
    Inside,
    Outside,
}

pub struct Region {
    /** zero-indexed offset from left edge */
    pub left: u32,
    /** zero-indexed offset from top edge */
    pub top: u32,
    /** dimension of extracted image */
    pub width: u32,
    /** dimension of extracted image */
    pub height: u32,
}

pub struct TrimOptions {
    /** Background colour, parsed by the color module, defaults to that of the top-left pixel. (optional) */
    pub background: Option<Colour>,
    /** Allowed difference from the above colour, a positive number. (optional, default 10) */
    pub threshold: Option<f64>,
    /** Does the input more closely resemble line art (e.g. vector) rather than being photographic? (optional, default false) */
    pub line_art: Option<bool>,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum Position {
    Top = 1,
    Right = 2,
    Bottom = 3,
    Left = 4,
    RightTop = 5,
    RightBottom = 6,
    LeftBottom = 7,
    LeftTop = 8,
}

/*
 * Tint an image using the provided RGB.
 */
pub(crate) fn tint(image: VipsImage, tint: &[f64]) -> Result<VipsImage> {
    let black_img = VipsImage::black(1, 1)?;
    let tint_img = image_from_getpoint(tint)?;
    let tint_lab = black_img.add(&tint_img)?;
    let tint_lab = tint_lab.colourspace_with_opts(Interpretation::Lab, VOption::new().with("source_space", v_value!(Interpretation::Srgb as i32)))?;
    let point = tint_lab.getpoint(0, 0)?;
    let tint_lab = image_from_getpoint(&point)?;

    // LAB identity function
    let identity_lab = VipsImage::identity_with_opts(VOption::new().with("bands", v_value!(3)))?;
    let identity_lab = identity_lab.colourspace_with_opts(Interpretation::Lab, VOption::new().with("source_space", v_value!(Interpretation::Srgb as i32)))?;

    // Scale luminance range, 0.0 to 1.0
    let l = identity_lab.divide(&scalar_image_like(&identity_lab, 100.0)?)?;
    let weight_l = l.subtract(&scalar_image_like(&l, 0.5)?)?;
    let weight_l = weight_l.multiply(&weight_l)?;
    let weight_l = weight_l.multiply(&scalar_image_like(&weight_l, 4.0)?)?;
    let weight_l = scalar_image_like(&weight_l, 1.0)?.subtract(&weight_l)?;

    // Weighting functions
    let weight_ab = weight_l.multiply(&tint_lab)?;
    let weight_ab = weight_ab.extract_band_with_opts(1, VOption::new().with("n", v_value!(2)))?;
    let identity_lab = VipsImage::bandjoin(&[identity_lab, weight_ab])?;

    // Convert lookup table to sRGB
    let lut = identity_lab.colourspace_with_opts(Interpretation::Srgb, VOption::new().with("source_space", v_value!(Interpretation::Lab as i32)))?;

    // Original colourspace
    let mut type_before_tint = image.get_interpretation()?;
    if type_before_tint == Interpretation::Rgb {
        type_before_tint = Interpretation::Srgb
    };

    // Apply lookup table
    let image = if image.image_hasalpha() {
        let alpha = get_alpha_image(&image)?;
        let image = remove_alpha(image)?;
        let image = image.colourspace(Interpretation::BW)?.maplut(&lut)?.colourspace(type_before_tint)?;
        VipsImage::bandjoin(&[image, alpha])?
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
    let luminance = lab.extract_band(0)?;

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
        let normalized = linear(luminance, &[f], &[a])?;
        let normalized = VipsImage::bandjoin(&[normalized, chroma])?;
        let normalized = normalized.colourspace(type_before_normalize)?;
        // Attach original alpha channel, if any
        if image.image_hasalpha() {
            // Extract original alpha channel
            let alpha = get_alpha_image(&image)?;
            // Join alpha channel to normalised image
            return VipsImage::bandjoin(&[normalized, alpha]);
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
        let alpha = get_alpha_image(&image)?;
        let image = remove_alpha(image)?;
        let image = image.gamma_with_opts(VOption::new().with("exponent", v_value!(exponent)))?;
        VipsImage::bandjoin(&[image, alpha])
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
        let alpha = get_alpha_image(&image)?;
        let image = remove_alpha(image)?;
        let image = image.invert()?;
        VipsImage::bandjoin(&[image, alpha])
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
        let blur = VipsImage::image_new_matrix_from_array(3, 3, &[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0])?;
        let blur = blur.scale_with_opts(VOption::new().with("exp", v_value!(9.0)))?;
        image.conv(&blur)
    } else {
        // Slower, accurate Gaussian blur
        let blur = stay_sequential(image, true)?;
        blur.gaussblur_with_opts(sigma, VOption::new().with("precision", v_value!(precision as i32)).with("min_ampl", v_value!(min_ampl)))
    }
}

/*
 * Convolution with a kernel.
 */
pub(crate) fn convolve(image: VipsImage, width: i32, height: i32, scale: f64, offset: f64, kernel_v: &[f64]) -> Result<VipsImage> {
    let bytes: &[u8] = unsafe { slice::from_raw_parts(kernel_v.as_ptr() as *const u8, std::mem::size_of_val(kernel_v)) };
    let kernel = VipsImage::new_from_memory(bytes, width, height, 1, BandFormat::Double)?;
    let kernel = scale_image(kernel, scale)?;

    // Convolve image with scaled kernel
    let image = image.conv(&kernel)?;

    // Add offset to the result
    image.add(&scalar_image_like(&image, offset)?)
}

/*
 * Recomb with a Matrix of the given bands/channel size.
 * Eg. RGB will be a 3x3 matrix.
 */
pub(crate) fn recomb(image: VipsImage, matrix: &[f64]) -> Result<VipsImage> {
    let m = matrix;
    let image = image.colourspace(Interpretation::Srgb)?;
    if matrix.len() == 9 {
        let m = if image.get_bands() == 3 {
            VipsImage::image_new_matrix_from_array(3, 3, matrix)?
        } else {
            VipsImage::image_new_matrix_from_array(4, 4, &[matrix[0], matrix[1], matrix[2], 0.0, matrix[3], matrix[4], matrix[5], 0.0, matrix[6], matrix[7], matrix[8], 0.0, 0.0, 0.0, 0.0, 1.0])?
        };
        image.recomb(&m)
    } else {
        image.recomb(&VipsImage::image_new_matrix_from_array(4, 4, m)?)
    }
}

pub(crate) fn modulate(image: VipsImage, brightness: f64, saturation: f64, hue: i32, lightness: f64) -> Result<VipsImage> {
    let colourspace_before_modulate = image.get_interpretation()?;
    if image.image_hasalpha() {
        // Separate alpha channel
        let alpha = get_alpha_image(&image)?;
        let image = remove_alpha(image)?;
        let image = image.colourspace(Interpretation::Lch)?.linear(&[brightness, saturation, 1.0], &[lightness, 0.0, hue as _])?.colourspace(colourspace_before_modulate)?;
        VipsImage::bandjoin(&[image, alpha])
    } else {
        image.colourspace(Interpretation::Lch)?.linear(&[brightness, saturation, 1.0], &[lightness, 0.0, hue as _])?.colourspace(colourspace_before_modulate)
    }
}

/*
 * Sharpen flat and jagged areas. Use sigma of -1.0 for fast sharpen.
 */
pub(crate) fn sharpen(image: VipsImage, sigma: f64, m_1: f64, m_2: f64, x_1: f64, y_2: f64, y_3: f64) -> Result<VipsImage> {
    if sigma == -1.0 {
        // Fast, mild sharpen
        let sharpen = VipsImage::image_new_matrix_from_array(3, 3, &[-1.0, -1.0, -1.0, -1.0, 32.0, -1.0, -1.0, -1.0, -1.0])?;
        let sharpen = scale_image(sharpen, 24.0)?;
        image.conv(&sharpen)
    } else {
        // Slow, accurate sharpen in LAB colour space, with control over flat vs jagged areas
        let mut colourspace_before_sharpen = image.get_interpretation()?;
        if colourspace_before_sharpen == Interpretation::Rgb {
            colourspace_before_sharpen = Interpretation::Srgb;
        }

        image
            .sharpen_with_opts(
                VOption::new().with("sigma", v_value!(sigma)).with("m_1", v_value!(m_1)).with("m_2", v_value!(m_2)).with("x_1", v_value!(x_1)).with("y_2", v_value!(y_2)).with("y_3", v_value!(y_3)),
            )?
            .colourspace(colourspace_before_sharpen)
    }
}

pub(crate) fn threshold(image: VipsImage, threshold: f64, threshold_grayscale: bool) -> Result<VipsImage> {
    if !threshold_grayscale {
        image.relational_const(OperationRelational::Moreeq, &[threshold])
    } else {
        image.colourspace(Interpretation::BW)?.relational_const(OperationRelational::Moreeq, &[threshold])
    }
}

/*
  Perform boolean/bitwise operation on image color channels - results in one channel image
*/
pub(crate) fn bandbool(image: VipsImage, boolean: OperationBoolean) -> Result<VipsImage> {
    image.bandbool(boolean)?.copy_with_opts(VOption::new().with("interpretation", v_value!(Interpretation::BW as i32)))
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
        let one_pixel = image.extract_area(0, 0, 1, 1)?;
        background = one_pixel.getpoint(0, 0)?
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
        let alpha = get_alpha_image(&image)?;
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
        let alpha = get_alpha_image(&image)?;
        let image = remove_alpha(image)?.linear_with_opts(a, b, VOption::new().with("uchar", v_value!(uchar)))?;
        VipsImage::bandjoin(&[image, alpha])
    } else {
        image.linear_with_opts(a, b, VOption::new().with("uchar", v_value!(uchar)))
    }
}

/*
 * Unflatten
 */
pub(crate) fn unflatten(image: VipsImage) -> Result<VipsImage> {
    if image.image_hasalpha() {
        let alpha = get_alpha_image(&image)?;
        let no_alpha = remove_alpha(image)?;
        let gray = no_alpha.colourspace(Interpretation::BW)?;
        let mask = gray.relational_const(OperationRelational::Less, &[255.0])?;
        let new_alpha = alpha.boolean(&mask, OperationBoolean::And)?;
        VipsImage::bandjoin(&[no_alpha, new_alpha])
    } else {
        let gray = image.colourspace(Interpretation::BW)?;
        VipsImage::bandjoin(&[image, gray])
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
    let mask = VipsImage::image_new_matrix(mask_width, mask_width)?;
    image.morph(&mask, OperationMorphology::Dilate)?.invert()
}

/*
 * Erode an image
 */
pub(crate) fn erode(image: VipsImage, width: i32) -> Result<VipsImage> {
    let mask_width = 2 * width + 1;
    let mask = VipsImage::image_new_matrix(mask_width, mask_width)?;
    image.morph(&mask, OperationMorphology::Erode)?.invert()
}
