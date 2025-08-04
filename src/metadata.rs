use crate::{
    common::{
        exif_orientation, get_density, has_density, has_profile, image_type_id, ImageType,
        InputDescriptor,
    },
    input::open_input,
};
use libvips::{
    bindings::{
        vips_array_double_get_type, vips_array_int_get_type, vips_blob_get_type,
        vips_image_get_string, vips_image_map, vips_isprefix, vips_ref_string_get_type, GValue,
        VIPS_META_BITS_PER_SAMPLE, VIPS_META_EXIF_NAME, VIPS_META_ICC_NAME, VIPS_META_IPTC_NAME,
        VIPS_META_N_PAGES, VIPS_META_N_SUBIFDS, VIPS_META_PAGE_HEIGHT, VIPS_META_PALETTE,
        VIPS_META_PHOTOSHOP_NAME, VIPS_META_RESOLUTION_UNIT, VIPS_META_XMP_NAME,
    },
    utils::{get_g_type, G_TYPE_INT},
    Result,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ffi::{c_char, c_void, CStr, CString},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub format: String,
    pub width: i32,
    pub height: i32,
    pub space: String,
    pub channels: i32,
    pub depth: String,
    pub density: i32,
    pub chroma_subsampling: String,
    pub is_progressive: bool,
    pub is_palette: bool,
    pub bits_per_sample: i32,
    pub pages: i32,
    pub page_height: i32,
    pub loop_: i32,
    pub delay: Vec<i32>,
    pub page_primary: i32,
    pub compression: String,
    pub resolution_unit: String,
    pub format_magick: String,
    pub levels: Vec<(i32, i32)>,
    pub subifds: i32,
    pub background: Vec<f64>,
    pub has_profile: bool,
    pub has_alpha: bool,
    pub orientation: i32,
    pub exif: Vec<f64>,
    pub icc: Vec<f64>,
    pub iptc: Vec<f64>,
    pub xmp: Vec<f64>,
    pub tifftag_photoshop: Vec<f64>,
    pub comments: HashMap<String, String>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            channels: 0,
            density: 0,
            is_progressive: false,
            is_palette: false,
            bits_per_sample: 0,
            pages: 0,
            page_height: 0,
            loop_: -1,
            page_primary: -1,
            subifds: 0,
            has_profile: false,
            has_alpha: false,
            orientation: 0,
            exif: Vec::new(),
            icc: Vec::new(),
            iptc: Vec::new(),
            xmp: Vec::new(),
            tifftag_photoshop: Vec::new(),
            format: String::new(),
            space: String::new(),
            depth: String::new(),
            chroma_subsampling: String::new(),
            delay: Vec::new(),
            compression: String::new(),
            resolution_unit: String::new(),
            format_magick: String::new(),
            levels: Vec::new(),
            background: Vec::new(),
            comments: HashMap::new(),
        }
    }
}

pub(crate) fn get_metadata(input: &InputDescriptor) -> Result<Metadata> {
    let _guard = crate::util::VipsGuard;

    let mut baton = Metadata::default();
    let (image, image_type) = open_input(input)?;
    if image_type != ImageType::UNKNOWN {
        // Image type
        baton.format = image_type_id(image_type);
        // VipsImage attributes
        baton.width = image.get_width();
        baton.height = image.get_height();
        baton.space = (image.get_interpretation()? as u32).to_string();
        baton.channels = image.get_bands();
        baton.depth = (image.get_format()? as u32).to_string();

        if has_density(&image) {
            baton.density = get_density(&image);
        }
        if image.get_typeof("jpeg-chroma-subsample") == unsafe { vips_ref_string_get_type() } {
            baton.chroma_subsampling =
                image.get_string("jpeg-chroma-subsample").unwrap_or_default();
        }
        if image.get_typeof("interlaced") == get_g_type(G_TYPE_INT) {
            baton.is_progressive = image.get_int("interlaced").unwrap_or_default() == 1;
        }
        if image.get_typeof(VIPS_META_PALETTE) == get_g_type(G_TYPE_INT) {
            baton.is_palette = image.get_int(VIPS_META_PALETTE).unwrap_or_default() == 1;
        }
        if image.get_typeof(VIPS_META_BITS_PER_SAMPLE) == get_g_type(G_TYPE_INT) {
            baton.bits_per_sample = image.get_int(VIPS_META_BITS_PER_SAMPLE).unwrap_or_default();
        }
        if image.get_typeof(VIPS_META_N_PAGES) == get_g_type(G_TYPE_INT) {
            baton.pages = image.get_int(VIPS_META_N_PAGES).unwrap_or_default();
        }
        if image.get_typeof(VIPS_META_PAGE_HEIGHT) == get_g_type(G_TYPE_INT) {
            baton.page_height = image.get_int(VIPS_META_PAGE_HEIGHT).unwrap_or_default();
        }
        if image.get_typeof("loop") == get_g_type(G_TYPE_INT) {
            baton.loop_ = image.get_int("loop").unwrap_or_default();
        }
        if image.get_typeof("delay") == unsafe { vips_array_int_get_type() } {
            baton.delay = image.get_array_int("delay").unwrap_or_default();
        }
        if image.get_typeof("heif-primary") == get_g_type(G_TYPE_INT) {
            baton.page_primary = image.get_int("heif-primary").unwrap_or_default();
        }
        if image.get_typeof("heif-compression") == unsafe { vips_ref_string_get_type() } {
            baton.compression = image.get_string("heif-compression").unwrap_or_default();
        }
        if image.get_typeof(VIPS_META_RESOLUTION_UNIT) == unsafe { vips_ref_string_get_type() } {
            baton.resolution_unit = image.get_string(VIPS_META_RESOLUTION_UNIT).unwrap_or_default();
        }
        if image.get_typeof("magick-format") == unsafe { vips_ref_string_get_type() } {
            baton.format_magick = image.get_string("magick-format").unwrap_or_default();
        }
        if image.get_typeof("openslide.level-count") == unsafe { vips_ref_string_get_type() } {
            let levels: i32 = image
                .get_string("openslide.level-count")
                .unwrap_or(String::from("0"))
                .parse()
                .unwrap();
            for l in 0..levels {
                let prefix = format!(r#"openslide.level["{:?}"]."#, l);
                let width: i32 = image
                    .get_string(format!("{}width", prefix))
                    .unwrap_or(String::from("0"))
                    .parse()
                    .unwrap();
                let height: i32 = image
                    .get_string(format!("{}height", prefix))
                    .unwrap_or(String::from("0"))
                    .parse()
                    .unwrap();
                baton.levels.push((width, height));
            }
        }
        if image.get_typeof(VIPS_META_N_SUBIFDS) == get_g_type(G_TYPE_INT) {
            baton.subifds = image.get_int(VIPS_META_N_SUBIFDS).unwrap_or_default();
        }
        baton.has_profile = has_profile(&image);
        if image.get_typeof("background") == unsafe { vips_array_double_get_type() } {
            baton.background = image.get_array_double("background").unwrap_or_default();
        }
        // Derived attributes
        baton.has_alpha = image.image_hasalpha();
        baton.orientation = exif_orientation(&image);
        // EXIF
        if image.get_typeof(VIPS_META_EXIF_NAME) == unsafe { vips_blob_get_type() } {
            let exif = image.get_blob(VIPS_META_EXIF_NAME).unwrap_or_default();
            baton.exif = exif.iter().map(|e| *e as f64).collect();
        }
        // ICC profile
        if image.get_typeof(VIPS_META_ICC_NAME) == unsafe { vips_blob_get_type() } {
            let icc = image.get_blob(VIPS_META_ICC_NAME).unwrap_or_default();
            baton.icc = icc.iter().map(|e| *e as f64).collect();
        }
        // IPTC
        if image.get_typeof(VIPS_META_IPTC_NAME) == unsafe { vips_blob_get_type() } {
            let iptc = image.get_blob(VIPS_META_IPTC_NAME).unwrap_or_default();
            baton.iptc = iptc.iter().map(|e| *e as f64).collect();
        }
        // XMP
        if image.get_typeof(VIPS_META_XMP_NAME) == unsafe { vips_blob_get_type() } {
            let xmp = image.get_blob(VIPS_META_XMP_NAME).unwrap_or_default();
            baton.xmp = xmp.iter().map(|e| *e as f64).collect();
        }
        // TIFFTAG_PHOTOSHOP
        if image.get_typeof(VIPS_META_PHOTOSHOP_NAME) == unsafe { vips_blob_get_type() } {
            let tifftag_photoshop = image.get_blob(VIPS_META_PHOTOSHOP_NAME).unwrap_or_default();
            baton.tifftag_photoshop = tifftag_photoshop.iter().map(|e| *e as f64).collect();
        }
        // PNG comments
        let mut comments = Box::new(baton.comments.clone());
        let comments_ptr: *mut c_void = &mut *comments as *mut _ as *mut c_void;
        unsafe { vips_image_map(image.as_mut_ptr(), Some(read_pngcomment), comments_ptr) };
    }

    Ok(baton)
}

unsafe extern "C" fn read_pngcomment(
    image: *mut libvips::bindings::_VipsImage,
    field: *const c_char,
    _value: *mut GValue,
    data: *mut c_void,
) -> *mut c_void {
    let comments: &mut HashMap<String, String> =
        unsafe { &mut *(data as *mut HashMap<String, String>) };

    let png_comment_start = CString::new("png-comment-").unwrap();
    let png_comment_start_len = png_comment_start.to_string_lossy().len();
    let raw: *const c_char = png_comment_start.as_ptr();

    if vips_isprefix(raw, field) == 1 {
        let field_str = CStr::from_ptr(field).to_string_lossy().into_owned();
        let rest = &field_str[png_comment_start_len..];
        let keyword = rest.find('-').map(|idx| &rest[idx..]);
        let mut str: *const c_char = std::ptr::null();
        if keyword.is_some() && vips_image_get_string(image, field, &mut str) != 0 {
            // Skip the hyphen
            let keyword = &keyword.unwrap()[1..];
            let value = CStr::from_ptr(str).to_string_lossy().into_owned();
            comments.insert(keyword.to_string(), value);
        }
    }

    std::ptr::null_mut()
}
