use libvips::{
    bindings::{
        g_signal_connect_data, g_type_from_name, vips_blob_get_type, vips_error, vips_foreign_find_load, vips_foreign_find_load_buffer, vips_image_is_sequential, vips_image_map, vips_image_set_kill,
        vips_image_set_progress, vips_interpretation_max_alpha, vips_malloc, GValue, VIPS_META_ICC_NAME, VIPS_META_ORIENTATION, VIPS_META_PAGE_HEIGHT, VIPS_META_SEQUENTIAL,
    },
    error::Error::{OperationError, OperationErrorExt},
    ops::{Access, Align, BandFormat, FailOn, Interpretation, TextWrap},
    voption::{VOption, VipsValue},
    Result, VipsImage,
};
use std::{
    collections::HashMap,
    ffi::{c_char, c_int, c_void, CStr, CString},
    slice,
    sync::OnceLock,
};

#[derive(Debug, Clone)]
pub(crate) struct InputDescriptor {
    pub(crate) file: String,
    pub(crate) auto_orient: bool,
    pub(crate) buffer: Vec<u8>,
    pub(crate) fail_on: FailOn,
    pub(crate) limit_input_pixels: usize,
    pub(crate) unlimited: bool,
    pub(crate) access: Access,
    pub(crate) is_buffer: bool,
    pub(crate) density: f64,
    pub(crate) ignore_icc: bool,
    pub(crate) raw_depth: BandFormat,
    pub(crate) raw_channels: i32,
    pub(crate) raw_width: i32,
    pub(crate) raw_height: i32,
    pub(crate) raw_premultiplied: bool,
    pub(crate) pages: i32,
    pub(crate) page: i32,
    pub(crate) level: i32,
    pub(crate) subifd: i32,
    pub(crate) create_channels: i32,
    pub(crate) create_width: i32,
    pub(crate) create_height: i32,
    pub(crate) create_background: Vec<f64>,
    pub(crate) create_noise_type: String,
    pub(crate) create_noise_mean: f64,
    pub(crate) create_noise_sigma: f64,
    pub(crate) text_value: String,
    pub(crate) text_font: String,
    pub(crate) text_fontfile: String,
    pub(crate) text_width: i32,
    pub(crate) text_height: i32,
    pub(crate) text_align: Align,
    pub(crate) text_justify: bool,
    pub(crate) text_dpi: i32,
    pub(crate) text_rgba: bool,
    pub(crate) text_spacing: i32,
    pub(crate) text_wrap: TextWrap,
    pub(crate) text_autofit_dpi: i32,
    pub(crate) join_animated: bool,
    pub(crate) join_across: i32,
    pub(crate) join_shim: i32,
    pub(crate) join_background: Vec<f64>,
    pub(crate) join_halign: Align,
    pub(crate) join_valign: Align,
    pub(crate) pdf_background: Vec<f64>,
    pub(crate) svg_stylesheet: String,
    pub(crate) svg_high_bitdepth: bool,
    pub(crate) tiff_subifd: i32,
    pub(crate) open_slide_level: i32,
    pub(crate) jp2_oneshot: bool,
}

impl Default for InputDescriptor {
    fn default() -> Self {
        Self {
            auto_orient: false,
            buffer: Vec::new(),
            fail_on: FailOn::Warning,
            limit_input_pixels: 0x3FFF * 0x3FFF,
            unlimited: false,
            access: Access::Sequential,
            is_buffer: false,
            density: 72.0,
            ignore_icc: false,
            raw_depth: BandFormat::Uchar,
            raw_channels: 0,
            raw_width: 0,
            raw_height: 0,
            raw_premultiplied: false,
            pages: 1,
            page: 0,
            level: 0,
            subifd: -1,
            create_channels: 0,
            create_width: 0,
            create_height: 0,
            create_background: vec![0.0, 0.0, 0.0, 255.0],
            create_noise_mean: 0.0,
            create_noise_sigma: 0.0,
            text_width: 0,
            text_height: 0,
            text_align: Align::Low,
            text_justify: false,
            text_dpi: 72,
            text_rgba: false,
            text_spacing: 0,
            text_wrap: TextWrap::Word,
            text_autofit_dpi: 0,
            join_animated: false,
            join_across: 1,
            join_shim: 0,
            join_background: vec![0.0, 0.0, 0.0, 255.0],
            join_halign: Align::Low,
            join_valign: Align::Low,
            pdf_background: vec![255.0, 255.0, 255.0, 255.0],
            create_noise_type: String::new(),
            file: String::new(),
            text_font: String::new(),
            text_fontfile: String::new(),
            text_value: String::new(),
            svg_stylesheet: String::new(),
            jp2_oneshot: false,
            svg_high_bitdepth: false,
            tiff_subifd: 0,
            open_slide_level: 0,
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum ImageType {
    Jpeg,
    Png,
    Webp,
    JP2,
    Tiff,
    GIF,
    SVG,
    HEIF,
    PDF,
    MAGICK,
    OPENSLIDE,
    PPM,
    FITS,
    EXR,
    JXL,
    RAD,
    VIPS,
    RAW,
    UNKNOWN,
    MISSING,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Canvas {
    Crop,
    Embed,
    Max,
    Min,
    IgnoreAspect,
}

pub(crate) fn rgba_from_hex(color: u32) -> Vec<f64> {
    if has_alpha(color) {
        let r = (color >> 24) & 0xFF; /* Shift right by 24 bits, mask the last 8 bits */
        let g = (color >> 16) & 0xFF; /* Shift right by 16 bits, mask the last 8 bits */
        let b = (color >> 8) & 0xFF; /* Shift right by 8 bits, mask the last 8 bits */
        let a = (color & 0xFF) as f32 / 255.0; /* Extract alpha and normalize to [0.0, 1.0] */
        vec![r as f64, g as f64, b as f64, a as f64]
    } else {
        let r = (color >> 16) & 0xFF; /* Shift right by 16 bits, mask the last 8 bits */
        let g = (color >> 8) & 0xFF; /* Shift right by 8 bits, mask the last 8 bits */
        let b = color & 0xFF;
        vec![r as f64, g as f64, b as f64, 1.0]
    }
}

fn has_alpha(value: u32) -> bool {
    /* If the value is larger than 24 bits, it contains alpha */
    value > 0xFFFFFF
}

pub(crate) fn ends_with(str: &str, end: &str) -> bool {
    str.ends_with(end)
}

pub(crate) fn is_jpeg(str: &str) -> bool {
    ends_with(str, ".jpg") || ends_with(str, ".jpeg") || ends_with(str, ".JPG") || ends_with(str, ".JPEG")
}

pub(crate) fn is_png(str: &str) -> bool {
    ends_with(str, ".png") || ends_with(str, ".PNG")
}

pub(crate) fn is_webp(str: &str) -> bool {
    ends_with(str, ".webp") || ends_with(str, ".WEBP")
}

pub(crate) fn is_gif(str: &str) -> bool {
    ends_with(str, ".gif") || ends_with(str, ".GIF")
}

pub(crate) fn is_jp2(str: &str) -> bool {
    ends_with(str, ".jp2")
        || ends_with(str, ".jpx")
        || ends_with(str, ".j2k")
        || ends_with(str, ".j2c")
        || ends_with(str, ".JP2")
        || ends_with(str, ".JPX")
        || ends_with(str, ".J2K")
        || ends_with(str, ".J2C")
}

pub(crate) fn is_tiff(str: &str) -> bool {
    ends_with(str, ".tif") || ends_with(str, ".tiff") || ends_with(str, ".TIF") || ends_with(str, ".TIFF")
}

pub(crate) fn is_heic(str: &str) -> bool {
    ends_with(str, ".heic") || ends_with(str, ".HEIC")
}

pub(crate) fn is_heif(str: &str) -> bool {
    ends_with(str, ".heif") || ends_with(str, ".HEIF") || is_heic(str) || is_avif(str)
}

pub(crate) fn is_avif(str: &str) -> bool {
    ends_with(str, ".avif") || ends_with(str, ".AVIF")
}

pub(crate) fn is_jxl(str: &str) -> bool {
    ends_with(str, ".jxl") || ends_with(str, ".JXL")
}

pub(crate) fn is_dz(str: &str) -> bool {
    ends_with(str, ".dzi") || ends_with(str, ".DZI")
}

pub(crate) fn is_dz_zip(str: &str) -> bool {
    ends_with(str, ".zip") || ends_with(str, ".ZIP") || ends_with(str, ".szi") || ends_with(str, ".SZI")
}

pub(crate) fn is_v(str: &str) -> bool {
    ends_with(str, ".v") || ends_with(str, ".V") || ends_with(str, ".vips") || ends_with(str, ".VIPS")
}

pub(crate) fn image_type_id(image_type: ImageType) -> String {
    let id = match image_type {
        ImageType::Jpeg => "jpeg",
        ImageType::Png => "png",
        ImageType::Webp => "webp",
        ImageType::Tiff => "tiff",
        ImageType::GIF => "gif",
        ImageType::JP2 => "jp2",
        ImageType::SVG => "svg",
        ImageType::HEIF => "heif",
        ImageType::PDF => "pdf",
        ImageType::MAGICK => "magick",
        ImageType::OPENSLIDE => "openslide",
        ImageType::PPM => "ppm",
        ImageType::FITS => "fits",
        ImageType::EXR => "exr",
        ImageType::JXL => "jxl",
        ImageType::RAD => "rad",
        ImageType::VIPS => "vips",
        ImageType::RAW => "raw",
        ImageType::UNKNOWN => "unknown",
        ImageType::MISSING => "missing",
    };

    id.to_string()
}

// Static loader-to-type map, initialized once
fn loader_to_type() -> &'static HashMap<&'static str, ImageType> {
    static MAP: OnceLock<HashMap<&'static str, ImageType>> = OnceLock::new();
    MAP.get_or_init(|| {
        use ImageType::*;
        HashMap::from([
            ("VipsForeignLoadJpegFile", Jpeg),
            ("VipsForeignLoadJpegBuffer", Jpeg),
            ("VipsForeignLoadPngFile", Png),
            ("VipsForeignLoadPngBuffer", Png),
            ("VipsForeignLoadWebpFile", Webp),
            ("VipsForeignLoadWebpBuffer", Webp),
            ("VipsForeignLoadTiffFile", Tiff),
            ("VipsForeignLoadTiffBuffer", Tiff),
            ("VipsForeignLoadGifFile", GIF),
            ("VipsForeignLoadGifBuffer", GIF),
            ("VipsForeignLoadNsgifFile", GIF),
            ("VipsForeignLoadNsgifBuffer", GIF),
            ("VipsForeignLoadJp2kFile", JP2),
            ("VipsForeignLoadJp2kBuffer", JP2),
            ("VipsForeignLoadSvgFile", SVG),
            ("VipsForeignLoadSvgBuffer", SVG),
            ("VipsForeignLoadHeifFile", HEIF),
            ("VipsForeignLoadHeifBuffer", HEIF),
            ("VipsForeignLoadPdfFile", PDF),
            ("VipsForeignLoadPdfBuffer", PDF),
            ("VipsForeignLoadMagickFile", MAGICK),
            ("VipsForeignLoadMagickBuffer", MAGICK),
            ("VipsForeignLoadMagick7File", MAGICK),
            ("VipsForeignLoadMagick7Buffer", MAGICK),
            ("VipsForeignLoadOpenslideFile", OPENSLIDE),
            ("VipsForeignLoadPpmFile", PPM),
            ("VipsForeignLoadFitsFile", FITS),
            ("VipsForeignLoadOpenexr", EXR),
            ("VipsForeignLoadJxlFile", JXL),
            ("VipsForeignLoadJxlBuffer", JXL),
            ("VipsForeignLoadRadFile", RAD),
            ("VipsForeignLoadRadBuffer", RAD),
            ("VipsForeignLoadVips", VIPS),
            ("VipsForeignLoadVipsFile", VIPS),
            ("VipsForeignLoadRaw", RAW),
        ])
    })
}

/*
  Determine image format of a buffer.
*/
pub(crate) fn determine_image_type(buffer: &[u8]) -> ImageType {
    unsafe {
        let load = vips_foreign_find_load_buffer(buffer.as_ptr() as _, buffer.len() as _);
        if load.is_null() {
            return ImageType::UNKNOWN;
        }

        let c_str = CStr::from_ptr(load);
        if let Ok(loader_name) = c_str.to_str() {
            loader_to_type().get(loader_name).cloned().unwrap_or(ImageType::UNKNOWN)
        } else {
            ImageType::UNKNOWN
        }
    }
}

/*
  Determine image format, reads the first few bytes of the file
*/
pub(crate) fn determine_image_type_from_str(file: &str) -> ImageType {
    let filename = new_c_string(file).unwrap();
    let load = unsafe { vips_foreign_find_load(filename.as_ptr() as _) };

    if load.is_null() {
        return ImageType::UNKNOWN;
    }

    let c_str = unsafe { CStr::from_ptr(load) };
    if let Ok(loader_name) = c_str.to_str() {
        loader_to_type().get(loader_name).cloned().unwrap_or(ImageType::UNKNOWN)
    } else {
        ImageType::UNKNOWN
    }
}

/*
  Does this image type support multiple pages?
*/
pub(crate) fn image_type_supports_page(image_type: &ImageType) -> bool {
    image_type == &ImageType::Webp
        || image_type == &ImageType::MAGICK
        || image_type == &ImageType::GIF
        || image_type == &ImageType::JP2
        || image_type == &ImageType::Tiff
        || image_type == &ImageType::HEIF
        || image_type == &ImageType::PDF
}

/*
  Does this image type support removal of safety limits?
*/
pub(crate) fn image_type_supports_unlimited(image_type: &ImageType) -> bool {
    image_type == &ImageType::Jpeg || image_type == &ImageType::Png || image_type == &ImageType::SVG || image_type == &ImageType::HEIF
}

/*
  Does this image have an embedded profile?
*/
pub(crate) fn has_profile(image: &VipsImage) -> bool {
    unsafe { image.get_typeof(VIPS_META_ICC_NAME) == vips_blob_get_type() }
}

/*
  Get copy of embedded profile.
*/
pub(crate) fn get_profile(image: &VipsImage) -> Option<Vec<u8>> {
    if has_profile(image) {
        let data = image.get_blob(VIPS_META_ICC_NAME);
        if data.is_err() {
            return None;
        }

        Some(data.unwrap())
    } else {
        None
    }
}

/*
  Set embedded profile.
*/
pub(crate) fn set_profile(image: VipsImage, icc: Option<Vec<u8>>) -> Result<VipsImage> {
    if let Some(icc) = icc {
        let image = image.copy()?;
        image.set_blob(VIPS_META_ICC_NAME, &icc);
        return Ok(image);
    }
    Ok(image)
}

unsafe extern "C" fn remove_exif_callback(_image: *mut libvips::bindings::_VipsImage, name: *const c_char, _value: *mut GValue, data: *mut c_void) -> *mut c_void {
    if data.is_null() || name.is_null() {
        return data;
    }

    let field_name = match unsafe { CStr::from_ptr(name).to_str() } {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    // Recover the Vec<String> passed as Box<Vec<String>>
    let fields: &mut Vec<String> = unsafe { &mut *(data as *mut Vec<String>) };

    if field_name.starts_with("exif-") {
        fields.push(field_name.to_owned());
    }

    std::ptr::null_mut()
}

/*
  Remove all EXIF-related image fields.
*/
pub(crate) fn remove_exif(image: VipsImage) -> VipsImage {
    let mut field_names = Box::new(Vec::<String>::new());
    let field_names_ptr: *mut c_void = &mut *field_names as *mut _ as *mut c_void;

    unsafe {
        vips_image_map(image.as_mut_ptr(), Some(remove_exif_callback), field_names_ptr);
    }

    for name in field_names.iter() {
        image.remove(name.as_bytes());
    }
    image
}

/*
  Get EXIF Orientation of image, if any.
*/
pub(crate) fn exif_orientation(image: &VipsImage) -> i32 {
    let mut orientation = 0;
    if image.get_typeof(VIPS_META_ORIENTATION) != 0 {
        orientation = image.get_int(VIPS_META_ORIENTATION).unwrap();
    }
    orientation
}

/*
  Set EXIF Orientation of image.
*/
pub(crate) fn set_exif_orientation(image: VipsImage, orientation: i32) -> Result<VipsImage> {
    let image = image.copy()?;
    image.set_int(VIPS_META_ORIENTATION, orientation);
    Ok(image)
}

/*
  Remove EXIF Orientation from image.
*/
pub(crate) fn remove_exif_orientation(image: VipsImage) -> Result<VipsImage> {
    let image = image.copy()?;
    image.remove(VIPS_META_ORIENTATION);
    image.remove(b"exif-ifd0-Orientation");
    Ok(image)
}

/*
  Set animation properties if necessary.
*/
pub(crate) fn set_animation_properties(image: VipsImage, n_pages: i32, page_height: i32, delay: &[i32], loop_: i32) -> Result<VipsImage> {
    let has_delay = !delay.is_empty();
    let copied_image = image.copy()?;

    // Only set page-height if we have more than one page, or this could
    // accidentally turn into an animated image later.
    if n_pages > 1 {
        copied_image.set_int(VIPS_META_PAGE_HEIGHT, page_height);
    }
    if has_delay {
        let mut delay = delay.to_vec();
        if delay.len() == 1 {
            // We have just one delay, repeat that value for all frames.
            delay.extend(std::iter::repeat(delay[0]).take((n_pages - 1) as usize));
        }
        copied_image.set_array_int(b"delay", delay.as_slice());
    }
    let loop_value = if n_pages == 1 && !has_delay && loop_ == -1 {
        1
    } else {
        loop_
    };
    if loop_value != -1 {
        copied_image.set_int(b"loop", loop_value);
    }

    Ok(copied_image)
}

/*
  Remove animation properties from image.
*/
pub(crate) fn remove_animation_properties(image: VipsImage) -> Result<VipsImage> {
    let image = image.copy()?;
    image.remove(VIPS_META_PAGE_HEIGHT);
    image.remove(b"delay");
    image.remove(b"loop");

    Ok(image)
}

/*
  Remove GIF palette from image.
*/
pub(crate) fn remove_gif_palette(image: VipsImage) -> Result<VipsImage> {
    let image = image.copy()?;
    image.remove(b"gif-palette");
    Ok(image)
}

/*
  Does this image have a non-default density?
*/
pub(crate) fn has_density(image: &VipsImage) -> bool {
    image.get_xres() > 1.0
}

/*
  Get pixels/mm resolution as pixels/inch density.
*/
pub(crate) fn get_density(image: &VipsImage) -> i32 {
    let density = image.get_xres() * 25.4;
    density.round() as _
}

/*
  Set pixels/mm resolution based on a pixels/inch density.
*/
pub(crate) fn set_density(image: VipsImage, density: f64) -> Result<VipsImage> {
    let pixels_per_mm = density / 25.4;
    image.copy_with_opts(VOption::new().with("xres", VipsValue::Double(pixels_per_mm)).with("name", VipsValue::Double(pixels_per_mm)))
}

/*
  Check the proposed format supports the current dimensions.
*/
pub(crate) fn assert_image_type_dimensions(image: &VipsImage, image_type: ImageType) -> Result<()> {
    let height = if image.get_typeof(VIPS_META_PAGE_HEIGHT) == get_g_type("gint") {
        image.get_int(VIPS_META_PAGE_HEIGHT)?
    } else {
        image.get_height()
    };

    if image_type == ImageType::Jpeg {
        if image.get_width() > 65535 || height > 65535 {
            return Err(OperationError("Processed image is too large for the JPEG format"));
        }
    } else if image_type == ImageType::Webp {
        if image.get_width() > 16383 || height > 16383 {
            return Err(OperationError("Processed image is too large for the WebP format"));
        }
    } else if image_type == ImageType::GIF {
        if image.get_width() > 65535 || height > 65535 {
            return Err(OperationError("Processed image is too large for the GIF format"));
        }
    } else if image_type == ImageType::HEIF && image.get_width() > 16384 || height > 16384 {
        return Err(OperationError("Processed image is too large for the HEIF format"));
    }

    Ok(())
}

/*
  Attach an event listener for progress updates, used to detect timeout
*/
pub(crate) fn set_timeout(image: &VipsImage, seconds: i32) {
    unsafe {
        if seconds > 0 {
            let im = image.as_mut_ptr();

            if (*im).progress_signal.is_null() {
                let timeout = vips_malloc(im as *mut _, std::mem::size_of::<c_int>() as _) as *mut c_int;
                if timeout.is_null() {
                    panic!("Failed to allocate timeout");
                }

                *timeout = seconds;
                g_signal_connect_data(
                    im as *mut _,
                    b"eval\0".as_ptr() as *const _,
                    Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(vips_progress_call_back as *const ())),
                    timeout as *mut _,
                    None,
                    0,
                );

                vips_image_set_progress(im, 1);
            }
        }
    }
}

/*
  Event listener for progress updates, used to detect timeout
*/
unsafe extern "C" fn vips_progress_call_back(im: *mut libvips::bindings::VipsImage, progress: *mut libvips::bindings::VipsProgress, timeout: *mut c_int) {
    if *timeout > 0 && (*progress).run >= *timeout {
        vips_image_set_kill(im, 1);
        let c_str_domain = CString::new("timeout").unwrap();
        let c_str_fmt = CString::new("%d%% complete").unwrap();
        vips_error(c_str_domain.as_ptr(), c_str_fmt.as_ptr(), (*progress).percent);
        *timeout = 0;
    }
}

/*
  Calculate the (left, top) coordinates of the output image
  within the input image, applying the given gravity during an embed.

  @Azurebyte: We are basically swapping the inWidth and outWidth, inHeight and outHeight from the CalculateCrop function.
*/
pub(crate) fn calculate_embed_position(in_width: i32, in_height: i32, out_width: i32, out_height: i32, gravity: i32) -> (i32, i32) {
    let mut left = 0;
    let mut top = 0;
    match gravity {
        1 => {
            // North
            left = (out_width - in_width) / 2;
        }
        2 => {
            // East
            left = out_width - in_width;
            top = (out_height - in_height) / 2;
        }
        3 => {
            // South
            left = (out_width - in_width) / 2;
            top = out_height - in_height;
        }
        4 => {
            // West
            top = (out_height - in_height) / 2;
        }
        5 => {
            // Northeast
            left = out_width - in_width;
        }
        6 => {
            // Southeast
            left = out_width - in_width;
            top = out_height - in_height;
        }
        7 => {
            // Southwest
            top = out_height - in_height;
        }
        8 => {}
        // Northwest
        // Which is the default is 0,0 so we do not assign anything here.
        _ => {
            // Centre
            left = (out_width - in_width) / 2;
            top = (out_height - in_height) / 2;
        }
    }
    (left, top)
}

/*
  Calculate the (left, top) coordinates of the output image
  within the input image, applying the given gravity during a crop.
*/
pub(crate) fn calculate_crop(in_width: i32, in_height: i32, out_width: i32, out_height: i32, gravity: i32) -> (i32, i32) {
    let mut left = 0;
    let mut top = 0;
    match gravity {
        1 => {
            // North
            left = (in_width - out_width + 1) / 2;
        }
        2 => {
            // East
            left = in_width - out_width;
            top = (in_height - out_height + 1) / 2;
        }
        3 => {
            // South
            left = (in_width - out_width + 1) / 2;
            top = in_height - out_height;
        }
        4 => {
            // West
            top = (in_height - out_height + 1) / 2;
        }
        5 => {
            // Northeast
            left = in_width - out_width;
        }
        6 => {
            // Southeast
            left = in_width - out_width;
            top = in_height - out_height;
        }
        7 => {
            // Southwest
            top = in_height - out_height;
        }
        8 => {
            // Northwest
        }
        _ => {
            // Centre
            left = (in_width - out_width + 1) / 2;
            top = (in_height - out_height + 1) / 2;
        }
    }
    (left, top)
}

/*
  Calculate the (left, top) coordinates of the output image
  within the input image, applying the given x and y offsets.
*/
pub(crate) fn calculate_crop2(in_width: i32, in_height: i32, out_width: i32, out_height: i32, x: i32, y: i32) -> (i32, i32) {
    // default values
    let mut left = 0;
    let mut top = 0;

    // assign only if valid
    if x < (in_width - out_width) {
        left = x;
    } else if x >= (in_width - out_width) {
        left = in_width - out_width;
    }

    if y < (in_height - out_height) {
        top = y;
    } else if y >= (in_height - out_height) {
        top = in_height - out_height;
    }

    (left, top)
}

/*
  Are pixel values in this image 16-bit integer?
*/
pub(crate) fn is16_bit(interpretation: Interpretation) -> bool {
    interpretation as i32 == Interpretation::Rgb16 as i32 || interpretation as i32 == Interpretation::Grey16 as i32
}

/*
  Convert RGBA value to another colourspace
*/
pub(crate) fn get_rgba_as_colourspace(rgba: Vec<f64>, interpretation: Interpretation, should_premultiply: bool) -> Result<Vec<f64>> {
    let bands = rgba.len();
    if bands < 3 {
        return Ok(rgba);
    }
    let pixel = VipsImage::image_new_matrix(1, 1)?;
    pixel.set_int(b"bands", bands as _);
    let pixel = VipsImage::new_from_image(&pixel, &rgba)?.colourspace_with_opts(interpretation, VOption::new().with("source_space", VipsValue::Int(Interpretation::Srgb as _)))?;

    if should_premultiply {
        let pixel = pixel.premultiply()?;
        pixel.getpoint(0, 0)
    } else {
        pixel.getpoint(0, 0)
    }
}

/*
  Apply the alpha channel to a given colour
*/
pub(crate) fn apply_alpha(image: VipsImage, colour: &[f64], should_premultiply: bool) -> Result<(VipsImage, Vec<f64>)> {
    // Scale up 8-bit values to match 16-bit input image
    let interpretation = image.get_interpretation()?;
    let multiplier = if is16_bit(interpretation) {
        256.0
    } else {
        1.0
    };
    // Create alphaColour colour
    let mut alpha_colour = if image.get_bands() > 2 {
        vec![multiplier * colour[0], multiplier * colour[1], multiplier * colour[2]]
    } else {
        // Convert sRGB to greyscale
        vec![multiplier * (0.2126 * colour[0] + 0.7152 * colour[1] + 0.0722 * colour[2])]
    };
    // Add alpha channel(s) to alphaColour colour
    if colour[3] < 255.0 || image.image_hasalpha() {
        let extra_bands = if image.get_bands() > 4 {
            image.get_bands() - 3
        } else {
            1
        };
        alpha_colour.extend(std::iter::repeat(colour[3] * multiplier).take(extra_bands as usize));
    }
    // Ensure alphaColour colour uses correct colourspace
    alpha_colour = get_rgba_as_colourspace(alpha_colour, image.get_interpretation()?, should_premultiply)?;

    // Add non-transparent alpha channel, if required
    if colour[3] < 255.0 && !image.image_hasalpha() {
        let image = image.bandjoin_const(&[255.0 * multiplier])?;
        Ok((image, alpha_colour))
    } else {
        Ok((image, alpha_colour))
    }
}

/*
  Removes alpha channels, if any.
*/
pub(crate) fn remove_alpha(image: VipsImage) -> Result<VipsImage> {
    let mut image = image.copy()?;
    while image.get_bands() > 1 && image.image_hasalpha() {
        image = image.extract_band_with_opts(0, VOption::new().with("n", VipsValue::Int(image.get_bands() - 1)))?;
    }
    Ok(image)
}

/*
  Ensures alpha channel, if missing.
*/
pub(crate) fn ensure_alpha(image: VipsImage, value: f64) -> Result<VipsImage> {
    if !image.image_hasalpha() {
        let interpretation = image.get_interpretation()?;
        let max_alpha = unsafe { vips_interpretation_max_alpha(interpretation as _) };
        image.bandjoin_const(&[value * max_alpha])
    } else {
        Ok(image)
    }
}

pub(crate) fn get_alpha_image(image: &VipsImage) -> Result<VipsImage> {
    image.extract_band(image.get_bands() - 1)
}

pub(crate) fn resolve_shrink(width: i32, height: i32, target_width: i32, target_height: i32, canvas: Canvas, without_enlargement: bool, without_reduction: bool) -> (f64, f64) {
    let mut hshrink: f64 = 1.0;
    let mut vshrink: f64 = 1.0;

    if target_width > 0 && target_height > 0 {
        // Fixed width and height
        hshrink = width as f64 / target_width as f64;
        vshrink = height as f64 / target_height as f64;

        match canvas {
            Canvas::Crop | Canvas::Min => {
                if hshrink < vshrink {
                    vshrink = hshrink;
                } else {
                    hshrink = vshrink;
                }
            }
            Canvas::Embed | Canvas::Max => {
                if hshrink > vshrink {
                    vshrink = hshrink;
                } else {
                    hshrink = vshrink;
                }
            }
            Canvas::IgnoreAspect => {}
        }
    } else if target_width > 0 {
        // Fixed width
        hshrink = width as f64 / target_width as f64;

        if canvas != Canvas::IgnoreAspect {
            // Auto height
            vshrink = hshrink;
        }
    } else if target_height > 0 {
        // Fixed height
        vshrink = height as f64 / target_height as f64;

        if canvas != Canvas::IgnoreAspect {
            // Auto width
            hshrink = vshrink;
        }
    }

    // We should not reduce or enlarge the output image, if
    // withoutReduction or withoutEnlargement is specified.
    if without_reduction {
        // Equivalent of VIPS_SIZE_UP
        hshrink = hshrink.min(1.0);
        vshrink = vshrink.min(1.0);
    } else if without_enlargement {
        // Equivalent of VIPS_SIZE_DOWN
        hshrink = hshrink.max(1.0);
        vshrink = vshrink.max(1.0);
    }

    // We don't want to shrink so much that we send an axis to 0
    hshrink = hshrink.min(width as f64);
    vshrink = vshrink.min(height as f64);

    (hshrink, vshrink)
}

/*
  Ensure decoding remains sequential.
*/
pub(crate) fn stay_sequential(image: VipsImage, condition: bool) -> Result<VipsImage> {
    if unsafe { vips_image_is_sequential(image.as_mut_ptr()) > 0 } && condition {
        let copied_image = VipsImage::image_copy_memory(image)?.copy()?;
        copied_image.remove(VIPS_META_SEQUENTIAL);
        Ok(copied_image)
    } else {
        Ok(image)
    }
}

pub(crate) fn scale_image(image: VipsImage, scale: f64) -> Result<VipsImage> {
    image.multiply(&scalar_image_like(&image, scale)?)
}

pub(crate) fn scalar_image_like(image: &VipsImage, value: f64) -> Result<VipsImage> {
    let w = image.get_width();
    let h = image.get_height();
    let bands = image.get_bands();
    // match source format
    let format = image.get_format()?;

    match format {
        BandFormat::Double => {
            let data: Vec<f64> = vec![value; (w * h * bands) as usize];
            let bytes = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * std::mem::size_of::<f64>()) };
            VipsImage::new_from_memory(bytes, w, h, bands, BandFormat::Double)
        }
        BandFormat::Float => {
            let data: Vec<f32> = vec![value as f32; (w * h * bands) as usize];
            let bytes = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * std::mem::size_of::<f32>()) };
            VipsImage::new_from_memory(bytes, w, h, bands, BandFormat::Float)
        }
        BandFormat::Uchar => {
            let data: Vec<u8> = vec![value.clamp(0.0, 255.0) as u8; (w * h * bands) as usize];
            VipsImage::new_from_memory(&data, w, h, bands, BandFormat::Uchar)
        }
        _ => Err(OperationErrorExt(format!("Unsupported format: {:?}", format))),
    }
}

pub(crate) fn image_from_getpoint(vec: &[f64]) -> Result<VipsImage> {
    let bands = vec.len() as i32;

    let bytes: &[u8] = unsafe { slice::from_raw_parts(vec.as_ptr() as *const u8, std::mem::size_of_val(vec)) };

    VipsImage::new_from_memory(bytes, 1, 1, bands, BandFormat::Double)
}

#[inline]
pub(crate) fn new_c_string(string: &str) -> Result<CString> {
    CString::new(string).map_err(|_| libvips::error::Error::InitializationError("Error initializing C string."))
}

pub(crate) fn get_g_type(name: &str) -> u64 {
    let type_name = new_c_string(name).unwrap();
    unsafe { g_type_from_name(type_name.as_ptr()) }
}
