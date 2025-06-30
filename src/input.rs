use crate::{
    common::{determine_image_type, determine_image_type_from_str, image_type_supports_page, image_type_supports_unlimited, set_density, ImageType, InputDescriptor},
    in_range, Colour, InvalidParameterError,
};
use libvips::{
    bindings::vips_band_format_is8bit,
    error::Error::{OperationError, OperationErrorExt},
    ops::{Align, BandFormat, FailOn, Interpretation, TextWrap},
    v_value,
    voption::{VOption, V_Value},
    Result, VipsImage,
};

#[derive(Debug, Clone)]
pub enum Input {
    Path(String),
    Buffer(Vec<u8>),
    None(),
}

#[derive(Debug, Clone, Default)]
pub struct SharpOptions {
    pub auto_orient: Option<bool>,
    pub fail_on: Option<FailOn>,
    pub limit_input_pixels: Option<usize>,
    pub unlimited: Option<bool>,
    pub sequential_read: Option<bool>,
    pub density: Option<f64>,
    pub ignore_icc: Option<bool>,
    pub pages: Option<i32>,
    pub page: Option<i32>,
    pub subifd: Option<i32>,
    pub level: Option<i32>,
    pub pdf_background: Option<Colour>,
    pub animated: Option<bool>,
    pub raw: Option<CreateRaw>,
    pub create: Option<Create>,
    pub text: Option<CreateText>,
    pub join: Option<Join>,
}

#[derive(Debug, Clone, Default)]
pub struct Raw {
    pub width: i32,
    pub height: i32,
    pub channels: u32,
}

#[derive(Debug, Clone, Default)]
pub struct CreateRaw {
    pub width: i32,
    pub height: i32,
    pub channels: u32,
    /* Specifies that the raw input has already been premultiplied, set to true to avoid sharp premultiplying the image. (optional, default false) */
    pub premultiplied: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Create {
    /** i32 of pixels wide. */
    pub width: i32,
    /** i32 of pixels high. */
    pub height: i32,
    /** i32 of bands, 3 for RGB, 4 for RGBA */
    pub channels: u32,
    /** Parsed by the [color](https://www.npmjs.org/package/color) module to extract values for red, green, blue and alpha. */
    pub background: Colour,
    /** Describes a noise to be created. */
    pub noise: Option<Noise>,
}

#[derive(Debug, Clone, Default)]
pub struct Noise {
    /** type of generated noise, currently only gaussian is supported. */
    pub gaussian: Option<bool>,
    /** mean of pixels in generated noise. */
    pub mean: Option<f64>,
    /** standard deviation of pixels in generated noise. */
    pub sigma: Option<f64>,
}

#[derive(Debug, Clone, Default)]
pub struct CreateText {
    /** Text to render as a UTF-8 string. It can contain Pango markup, for example `<i>Le</i>Monde`. */
    pub text: String,
    /** Font name to render with. */
    pub font: Option<String>,
    /** Absolute filesystem path to a font file that can be used by `font`. */
    pub fontfile: Option<String>,
    /** Integral i32 of pixels to word-wrap at. Lines of text wider than this will be broken at word boundaries. (optional, default `0`) */
    pub width: Option<i32>,
    /**
     * Integral i32 of pixels high. When defined, `dpi` will be ignored and the text will automatically fit the pixel resolution
     * defined by `width` and `height`. Will be ignored if `width` is not specified or set to 0. (optional, default `0`)
     */
    pub height: Option<i32>,
    /** Text alignment ('left', 'centre', 'center', 'right'). (optional, default 'left') */
    pub align: Option<TextAlign>,
    /** Set this to true to apply justification to the text. (optional, default `false`) */
    pub justify: Option<bool>,
    /** The resolution (size) at which to render the text. Does not take effect if `height` is specified. (optional, default `72`) */
    pub dpi: Option<i32>,
    /**
     * Set this to true to enable RGBA output. This is useful for colour emoji rendering,
     * or support for pango markup features like `<span foreground="red">Red!</span>`. (optional, default `false`)
     */
    pub rgba: Option<bool>,
    /** Text line height in points. Will use the font line height if none is specified. (optional, default `0`) */
    pub spacing: Option<i32>,
    /** Word wrapping style when width is provided, one of: 'word', 'char', 'word-char' (prefer word, fallback to char) or 'none' */
    pub wrap: Option<TextWrap>,
}

#[derive(Debug, Clone)]
pub enum TextAlign {
    Left,
    Centre,
    Right,
}

#[derive(Debug, Clone, Default)]
pub struct Join {
    /** Number of images per row. */
    pub across: Option<i32>,
    /** Treat input as frames of an animated image. */
    pub animated: Option<bool>,
    /** Space between images, in pixels. */
    pub shim: Option<i32>,
    /** Background colour. */
    pub background: Option<Colour>,
    /** Horizontal alignment. */
    pub halign: Option<HorizontalAlignment>,
    /* Vertical alignment. */
    pub valign: Option<VerticalAlignment>,
}

#[derive(Debug, Clone)]
pub enum HorizontalAlignment {
    Left,
    Centre,
    Right,
}

#[derive(Debug, Clone)]
pub enum VerticalAlignment {
    Top,
    Centre,
    Bottom,
}

#[derive(Debug, Clone)]
pub struct RotateOptions {
    pub background: Colour,
}

pub(crate) fn create_input_descriptor(input: Input, input_options: Option<SharpOptions>) -> core::result::Result<InputDescriptor, String> {
    let mut input_descriptor = InputDescriptor {
        auto_orient: false,
        fail_on: FailOn::Warning,
        limit_input_pixels: 0x3FFF_u32.pow(2) as _,
        ignore_icc: false,
        unlimited: false,
        ..Default::default()
    };

    match input {
        Input::Path(file) => {
            input_descriptor.file = file;
        }
        Input::Buffer(buffer) => {
            if buffer.is_empty() {
                return Err("Input Buffer is empty".to_string());
            }
            input_descriptor.buffer = buffer;
            input_descriptor.is_buffer = true;
        }
        Input::None() => {}
    };

    if let Some(input_options) = input_options {
        // failOn
        if let Some(fail_on) = input_options.fail_on {
            input_descriptor.fail_on = fail_on;
        }
        // autoOrient
        if let Some(auto_orient) = input_options.auto_orient {
            input_descriptor.auto_orient = auto_orient;
        }
        // Density
        if let Some(density) = input_options.density {
            input_descriptor.density = density;
        }
        // Ignore embeddded ICC profile
        if let Some(ignore_icc) = input_options.ignore_icc {
            input_descriptor.ignore_icc = ignore_icc;
        }
        // limitInputPixels
        if let Some(limit_input_pixels) = input_options.limit_input_pixels {
            input_descriptor.limit_input_pixels = limit_input_pixels;
        }
        // unlimited
        if let Some(unlimited) = input_options.unlimited {
            input_descriptor.unlimited = unlimited;
        }

        // Raw pixel input
        if let Some(raw) = input_options.raw {
            if !in_range(raw.channels as _, 1.0, 4.0) {
                return Err(InvalidParameterError!("raw.channels", "number between 1 and 4", raw.channels));
            }

            input_descriptor.raw_width = raw.width;
            input_descriptor.raw_height = raw.height;
            input_descriptor.raw_channels = raw.channels as _;
            input_descriptor.raw_premultiplied = raw.premultiplied;

            input_descriptor.raw_depth = BandFormat::Uchar;
        }
        // Multi-page input (GIF, TIFF, PDF)
        if let Some(animated) = input_options.animated {
            input_descriptor.pages = if animated {
                -1
            } else {
                1
            };
        }
        if let Some(pages) = input_options.pages {
            input_descriptor.pages = pages;
        }
        if let Some(page) = input_options.page {
            input_descriptor.page = page;
        }
        // Multi-level input (OpenSlide)
        if let Some(level) = input_options.level {
            input_descriptor.level = level;
        }
        // Sub Image File Directory (TIFF)
        if let Some(subifd) = input_options.subifd {
            input_descriptor.subifd = subifd;
        }
        // PDF background colour
        if let Some(pdf_background) = input_options.pdf_background {
            input_descriptor.pdf_background = pdf_background.rgba;
        }
        // Create new image
        if let Some(create) = input_options.create {
            input_descriptor.create_width = create.width;
            input_descriptor.create_height = create.height;
            input_descriptor.create_channels = create.channels as _;
            // Noise
            if let Some(noise) = create.noise {
                if !in_range(create.channels as _, 1.0, 4.0) {
                    return Err(InvalidParameterError!("create.channels", "number between 1 and 4", create.channels));
                }
                if let Some(gaussian) = noise.gaussian {
                    if gaussian {
                        input_descriptor.create_noise_type = "gaussian".to_string();
                    }
                }
                if let Some(mean) = noise.mean {
                    input_descriptor.create_noise_mean = mean;
                }
                if let Some(sigma) = noise.sigma {
                    input_descriptor.create_noise_sigma = sigma;
                }
            } else {
                if !in_range(create.channels as _, 3.0, 4.0) {
                    return Err(InvalidParameterError!("create.channels", "number between 3 and 4", create.channels));
                }
                input_descriptor.create_background = create.background.rgba;
            }
            input_descriptor.buffer.clear();
        }
        // Create a new image with text
        if let Some(text) = input_options.text {
            input_descriptor.text_value = text.text;

            if let Some(font) = text.font {
                input_descriptor.text_font = font;
            }
            if let Some(fontfile) = text.fontfile {
                input_descriptor.text_fontfile = fontfile;
            }
            if let Some(width) = text.width {
                input_descriptor.text_width = width;
            }
            if let Some(height) = text.height {
                input_descriptor.text_height = height;
            }
            if let Some(align) = text.align {
                input_descriptor.text_align = match align {
                    TextAlign::Centre => Align::Centre,
                    TextAlign::Left => Align::Low,
                    TextAlign::Right => Align::High,
                }
            }
            if let Some(justify) = text.justify {
                input_descriptor.text_justify = justify;
            }
            if let Some(dpi) = text.dpi {
                input_descriptor.text_dpi = dpi;
            }
            if let Some(rgba) = text.rgba {
                input_descriptor.text_rgba = rgba;
            }
            if let Some(spacing) = text.spacing {
                input_descriptor.text_spacing = spacing;
            }
            if let Some(wrap) = text.wrap {
                input_descriptor.text_wrap = wrap;
            }
            input_descriptor.buffer.clear();
        }
        // Join images together
        if let Some(join) = input_options.join {
            if let Some(animated) = join.animated {
                input_descriptor.join_animated = animated;
            }
            if let Some(across) = join.across {
                input_descriptor.join_across = across;
            }
            if let Some(shim) = join.shim {
                input_descriptor.join_shim = shim;
            }
            if let Some(background) = join.background {
                input_descriptor.join_background = background.rgba;
            }
            if let Some(halign) = join.halign {
                input_descriptor.join_halign = match halign {
                    HorizontalAlignment::Centre => Align::Centre,
                    HorizontalAlignment::Left => Align::Low,
                    HorizontalAlignment::Right => Align::High,
                };
            }
            if let Some(valign) = join.valign {
                input_descriptor.join_valign = match valign {
                    VerticalAlignment::Bottom => Align::High,
                    VerticalAlignment::Centre => Align::Centre,
                    VerticalAlignment::Top => Align::Low,
                };
            }
        }
    }

    Ok(input_descriptor)
}

/*
    Open an image from the given InputDescriptor (filesystem, compressed buffer, raw pixel data)
*/
pub(crate) fn open_input(descriptor: &InputDescriptor) -> Result<(VipsImage, ImageType)> {
    if descriptor.is_buffer {
        open_input_from_buffer(descriptor)
    } else {
        open_input_from(descriptor)
    }
}

pub(crate) fn open_input_from(descriptor: &InputDescriptor) -> Result<(VipsImage, ImageType)> {
    let channels = descriptor.create_channels;

    let (image, image_type) = if channels > 0 {
        // Create new image
        let image = if descriptor.create_noise_type == "gaussian" {
            let mut bands: Vec<VipsImage> = Vec::with_capacity(channels as _);

            for _band in 0..channels {
                bands.push(VipsImage::gaussnoise_with_opts(
                    descriptor.create_width,
                    descriptor.create_height,
                    VOption::new().with("mean", v_value!(descriptor.create_noise_mean)).with("sigma", v_value!(descriptor.create_noise_sigma)),
                )?);
            }
            let image = VipsImage::bandjoin(bands.as_mut_slice())?;
            let interpretation = if channels < 3 {
                Interpretation::BW
            } else {
                Interpretation::Srgb
            };
            image.copy_with_opts(VOption::new().with("interpretation", v_value!(interpretation as i32)))?
        } else {
            let mut background = vec![descriptor.create_background[0], descriptor.create_background[1], descriptor.create_background[2]];
            if channels == 4 {
                background.push(descriptor.create_background[3]);
            }

            let image = VipsImage::image_new_matrix(descriptor.create_width, descriptor.create_height)?;
            let interpretation = if channels < 3 {
                Interpretation::BW
            } else {
                Interpretation::Srgb
            };
            let image = image.copy_with_opts(VOption::new().with("interpretation", v_value!(interpretation as i32)))?;
            VipsImage::new_from_image(&image, &background)?
        };

        let image = image.cast(BandFormat::Uchar)?;

        (image, ImageType::RAW)
    } else if !descriptor.text_value.is_empty() {
        // Create a new image with text
        let mut text_options = VOption::new()
            .with("align", v_value!(descriptor.text_align as i32))
            .with("justify", v_value!(descriptor.text_justify))
            .with("rgba", v_value!(descriptor.text_rgba))
            .with("spacing", v_value!(descriptor.text_spacing))
            .with("wrap", v_value!(descriptor.text_wrap as i32))
            .with("autofit_dpi", v_value!(descriptor.text_autofit_dpi));

        if descriptor.text_width > 0 {
            text_options.set("width", v_value!(descriptor.text_width));
        }
        // Ignore dpi if height is set
        if descriptor.text_width > 0 && descriptor.text_height > 0 {
            text_options.set("height", v_value!(descriptor.text_height));
        } else if descriptor.text_dpi > 0 {
            text_options.set("dpi", v_value!(descriptor.text_dpi));
        }
        if !descriptor.text_font.is_empty() {
            text_options.set("font", v_value!(descriptor.text_font.as_str()));
        }
        if !descriptor.text_fontfile.is_empty() {
            text_options.set("fontfile", v_value!(descriptor.text_fontfile.as_str()));
        }
        let image = VipsImage::text_with_opts(&descriptor.text_value, text_options)?;

        if descriptor.text_rgba {
            (image, ImageType::RAW)
        } else {
            (image.copy_with_opts(VOption::new().with("interpretation", v_value!(Interpretation::BW as i32)))?, ImageType::RAW)
        }
    } else {
        // From filesystem
        let image_type = determine_image_type_from_str(&descriptor.file);

        if image_type == ImageType::MISSING {
            if descriptor.file.contains("<svg") {
                let msg = format!("Input file is missing, did you mean Buffer.from('{:?}')", descriptor.file[0..8].to_string());
                return Err(OperationErrorExt(msg));
            }
            return Err(OperationErrorExt(format!("Input file is missing: {}", descriptor.file)));
        }
        if image_type != ImageType::UNKNOWN {
            let mut option = VOption::new().with("access", v_value!(descriptor.access as i32)).with("fail_on", v_value!(descriptor.fail_on as i32));

            if descriptor.unlimited && image_type_supports_unlimited(&image_type) {
                option.set("unlimited", v_value!(true));
            }

            if image_type_supports_page(&image_type) {
                option.set("n", v_value!(descriptor.pages));
                option.set("page", v_value!(descriptor.page));
            }

            let density = descriptor.density.to_string();
            match image_type {
                ImageType::SVG => {
                    option.set("dpi", v_value!(descriptor.density));
                    option.set("stylesheet", v_value!(&descriptor.svg_stylesheet));
                    option.set("high_bitdepth", v_value!(descriptor.svg_high_bitdepth))
                }
                ImageType::Tiff => option.set("tiffSubifd", v_value!(descriptor.tiff_subifd)),
                ImageType::PDF => {
                    option.set("dpi", v_value!(descriptor.density));
                    option.set("background", v_value!(descriptor.pdf_background.as_slice()))
                }
                ImageType::OPENSLIDE => option.set("openSlideLevel", v_value!(descriptor.open_slide_level)),
                ImageType::JP2 => option.set("oneshot", v_value!(descriptor.jp2_oneshot)),
                ImageType::MAGICK => option.set("density", v_value!(&density)),
                _ => {}
            };

            let image = VipsImage::new_from_file_with_opts(&descriptor.file, option)?;
            if image_type == ImageType::SVG || image_type == ImageType::PDF || image_type == ImageType::MAGICK {
                (set_density(image, descriptor.density)?, image_type)
            } else {
                (image, image_type)
            }
        } else {
            return Err(OperationError("Input file contains unsupported image format"));
        }
    };

    // Limit input images to a given number of pixels, where pixels = width * height
    if descriptor.limit_input_pixels > 0 && image.get_width() * image.get_height() > descriptor.limit_input_pixels as i32 {
        return Err(OperationError("Input image exceeds pixel limit"));
    }

    Ok((image, image_type))
}

pub(crate) fn open_input_from_buffer(descriptor: &InputDescriptor) -> Result<(VipsImage, ImageType)> {
    let (image, image_type) = if descriptor.raw_channels > 0 {
        // Raw, uncompressed pixel data
        let is8bit = unsafe { vips_band_format_is8bit(descriptor.raw_depth as _) } == 1;
        let image = VipsImage::new_from_memory(descriptor.buffer.as_slice(), descriptor.raw_width, descriptor.raw_height, descriptor.raw_channels, descriptor.raw_depth)?;
        let image = if descriptor.raw_channels < 3 {
            image.colourspace(if is8bit {
                Interpretation::BW
            } else {
                Interpretation::Grey16
            })?
        } else {
            image.colourspace(if is8bit {
                Interpretation::Srgb
            } else {
                Interpretation::Rgb16
            })?
        };
        let image = if descriptor.raw_premultiplied {
            image.unpremultiply()?
        } else {
            image
        };
        (image, ImageType::RAW)
    } else {
        // Compressed data
        let image_type = determine_image_type(&descriptor.buffer);
        if image_type != ImageType::UNKNOWN {
            let mut option = VOption::new().with("access", v_value!(descriptor.access as i32)).with("fail_on", v_value!(descriptor.fail_on as i32));

            if descriptor.unlimited && image_type_supports_unlimited(&image_type) {
                option.set("unlimited", v_value!(true));
            }

            if image_type_supports_page(&image_type) {
                option.set("n", v_value!(descriptor.pages));
                option.set("page", v_value!(descriptor.page));
            }

            let density = descriptor.density.to_string();
            match image_type {
                ImageType::SVG => {
                    option.set("dpi", v_value!(descriptor.density));
                    option.set("stylesheet", v_value!(&descriptor.svg_stylesheet));
                    option.set("high_bitdepth", v_value!(descriptor.svg_high_bitdepth))
                }
                ImageType::Tiff => option.set("tiffSubifd", v_value!(descriptor.tiff_subifd)),
                ImageType::PDF => {
                    option.set("dpi", v_value!(descriptor.density));
                    option.set("background", v_value!(descriptor.pdf_background.as_slice()))
                }
                ImageType::OPENSLIDE => option.set("openSlideLevel", v_value!(descriptor.open_slide_level)),
                ImageType::JP2 => option.set("oneshot", v_value!(descriptor.jp2_oneshot)),
                ImageType::MAGICK => option.set("density", v_value!(&density)),
                _ => {}
            };
            let image = VipsImage::new_from_buffer_with_opts(descriptor.buffer.as_slice(), option)?;
            if image_type == ImageType::SVG || image_type == ImageType::PDF || image_type == ImageType::MAGICK {
                (set_density(image, descriptor.density)?, image_type)
            } else {
                (image, image_type)
            }
        } else {
            return Err(OperationError("Input buffer contains unsupported image format"));
        }
    };

    // Limit input images to a given number of pixels, where pixels = width * height
    if descriptor.limit_input_pixels > 0 && image.get_width() * image.get_height() > descriptor.limit_input_pixels as i32 {
        return Err(OperationError("Input image exceeds pixel limit"));
    }

    Ok((image, image_type))
}
