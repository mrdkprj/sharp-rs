use crate::{
    common::{
        apply_alpha, assert_image_type_dimensions, calculate_crop, calculate_crop2,
        calculate_embed_position, ensure_alpha, exif_orientation, get_profile, has_profile,
        image_type_id, is16_bit, is_dz, is_dz_zip, is_gif, is_heif, is_jp2, is_jpeg, is_jxl,
        is_png, is_tiff, is_v, is_webp, remove_alpha, remove_animation_properties, remove_exif,
        remove_exif_orientation, remove_gif_palette, resolve_shrink, set_animation_properties,
        set_density, set_exif_orientation, set_profile, set_timeout, stay_sequential, Canvas,
        ImageType, InputDescriptor,
    },
    input::open_input,
    operation::{
        bandbool, blur, boolean, clahe, convolve, crop_multi_page, dilate, embed_multi_page,
        ensure_colourspace, erode, flatten, foreign_webp_preset_string, gamma, linear, modulate,
        negate, normalise, recomb, sharpen, threshold, tint, trim, unflatten,
    },
    util::{get_g_type, VipsGuard, G_TYPE_INT},
};
use rs_vips::{
    bindings::{
        VipsForeignKeep_VIPS_FOREIGN_KEEP_EXIF, VipsForeignKeep_VIPS_FOREIGN_KEEP_ICC,
        VipsForeignKeep_VIPS_FOREIGN_KEEP_XMP, VIPS_META_N_PAGES, VIPS_META_PAGE_HEIGHT,
        VIPS_META_XMP_NAME,
    },
    error::Error::OperationError,
    ops::{
        Angle, BandFormat, BlendMode, Direction, Extend, ForeignDzContainer, ForeignDzDepth,
        ForeignDzLayout, ForeignHeifCompression, ForeignPngFilter, ForeignSubsample,
        ForeignTiffCompression, ForeignTiffPredictor, ForeignTiffResunit, ForeignWebpPreset,
        Intent, Interesting, Interpretation, Kernel, OperationBoolean, Precision,
    },
    voption::{Setter, VOption},
    Result, VipsImage, VipsInterpolate,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(crate) struct Composite {
    pub(crate) input: InputDescriptor,
    pub(crate) mode: BlendMode,
    pub(crate) gravity: i32,
    pub(crate) left: i32,
    pub(crate) top: i32,
    pub(crate) has_offset: bool,
    pub(crate) tile: bool,
    pub(crate) premultiplied: bool,
}

impl Default for Composite {
    fn default() -> Self {
        Self {
            input: InputDescriptor::default(),
            mode: BlendMode::Over,
            gravity: 0,
            left: 0,
            top: 0,
            has_offset: false,
            tile: false,
            premultiplied: false,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PipelineBaton {
    pub(crate) input: InputDescriptor,
    pub(crate) join: Vec<InputDescriptor>,
    pub(crate) format_out: String,
    pub(crate) file_out: String,
    pub(crate) buffer_out: Vec<u8>,
    pub(crate) page_height_out: i32,
    pub(crate) pages_out: i32,
    pub(crate) composite: Vec<Composite>,
    pub(crate) join_channel_in: Vec<InputDescriptor>,
    pub(crate) top_offset_pre: i32,
    pub(crate) left_offset_pre: i32,
    pub(crate) width_pre: i32,
    pub(crate) height_pre: i32,
    pub(crate) top_offset_post: i32,
    pub(crate) left_offset_post: i32,
    pub(crate) width_post: i32,
    pub(crate) height_post: i32,
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) channels: i32,
    pub(crate) kernel: Kernel,
    pub(crate) canvas: Canvas,
    pub(crate) position: i32,
    pub(crate) resize_background: Vec<f64>,
    pub(crate) has_crop_offset: bool,
    pub(crate) crop_offset_left: i32,
    pub(crate) crop_offset_top: i32,
    pub(crate) has_attention_center: bool,
    pub(crate) attention_x: i32,
    pub(crate) attention_y: i32,
    pub(crate) premultiplied: bool,
    pub(crate) tile_centre: bool,
    pub(crate) fast_shrink_on_load: bool,
    pub(crate) tint: Vec<f64>,
    pub(crate) flatten: bool,
    pub(crate) flatten_background: Vec<f64>,
    pub(crate) unflatten: bool,
    pub(crate) negate: bool,
    pub(crate) negate_alpha: bool,
    pub(crate) blur_sigma: f64,
    pub(crate) precision: Precision,
    pub(crate) min_ampl: f64,
    pub(crate) brightness: f64,
    pub(crate) saturation: f64,
    pub(crate) hue: i32,
    pub(crate) lightness: f64,
    pub(crate) median_size: i32,
    pub(crate) sharpen_sigma: f64,
    pub(crate) sharpen_m1: f64,
    pub(crate) sharpen_m2: f64,
    pub(crate) sharpen_x1: f64,
    pub(crate) sharpen_y2: f64,
    pub(crate) sharpen_y3: f64,
    pub(crate) threshold: i32,
    pub(crate) threshold_grayscale: bool,
    pub(crate) trim_background: Vec<f64>,
    pub(crate) trim_threshold: f64,
    pub(crate) trim_line_art: bool,
    pub(crate) trim_offset_left: i32,
    pub(crate) trim_offset_top: i32,
    pub(crate) linear_a: Vec<f64>,
    pub(crate) linear_b: Vec<f64>,
    pub(crate) dilate_width: i32,
    pub(crate) erode_width: i32,
    pub(crate) gamma: f64,
    pub(crate) gamma_out: f64,
    pub(crate) greyscale: bool,
    pub(crate) normalise: bool,
    pub(crate) normalise_lower: i32,
    pub(crate) normalise_upper: i32,
    pub(crate) clahe_width: i32,
    pub(crate) clahe_height: i32,
    pub(crate) clahe_max_slope: i32,
    pub(crate) angle: i32,
    pub(crate) rotation_angle: f64,
    pub(crate) rotation_background: Vec<f64>,
    pub(crate) rotate_before: bool,
    pub(crate) orient_before: bool,
    pub(crate) flip: bool,
    pub(crate) flop: bool,
    pub(crate) extend_top: i32,
    pub(crate) extend_bottom: i32,
    pub(crate) extend_left: i32,
    pub(crate) extend_right: i32,
    pub(crate) extend_background: Vec<f64>,
    pub(crate) extend_with: Extend,
    pub(crate) without_enlargement: bool,
    pub(crate) without_reduction: bool,
    pub(crate) affine_matrix: Vec<f64>,
    pub(crate) affine_background: Vec<f64>,
    pub(crate) affine_idx: f64,
    pub(crate) affine_idy: f64,
    pub(crate) affine_odx: f64,
    pub(crate) affine_ody: f64,
    pub(crate) affine_interpolator: String,
    pub(crate) jpeg_quality: i32,
    pub(crate) jpeg_progressive: bool,
    pub(crate) jpeg_chroma_subsampling: String,
    pub(crate) jpeg_trellis_quantisation: bool,
    pub(crate) jpeg_quantisation_table: i32,
    pub(crate) jpeg_overshoot_deringing: bool,
    pub(crate) jpeg_optimise_scans: bool,
    pub(crate) jpeg_optimise_coding: bool,
    pub(crate) png_progressive: bool,
    pub(crate) png_compression_level: i32,
    pub(crate) png_adaptive_filtering: bool,
    pub(crate) png_palette: bool,
    pub(crate) png_quality: i32,
    pub(crate) png_effort: i32,
    pub(crate) png_bitdepth: i32,
    pub(crate) png_dither: f64,
    pub(crate) jp2_quality: i32,
    pub(crate) jp2_lossless: bool,
    pub(crate) jp2_tile_height: i32,
    pub(crate) jp2_tile_width: i32,
    pub(crate) jp2_chroma_subsampling: String,
    pub(crate) webp_quality: i32,
    pub(crate) webp_alpha_quality: i32,
    pub(crate) webp_near_lossless: bool,
    pub(crate) webp_lossless: bool,
    pub(crate) webp_smart_subsample: bool,
    pub(crate) webp_smart_deblock: bool,
    pub(crate) webp_preset: ForeignWebpPreset,
    pub(crate) webp_effort: i32,
    pub(crate) webp_min_size: bool,
    pub(crate) webp_mixed: bool,
    pub(crate) gif_bitdepth: i32,
    pub(crate) gif_effort: i32,
    pub(crate) gif_dither: f64,
    pub(crate) gif_inter_frame_max_error: f64,
    pub(crate) gif_inter_palette_max_error: f64,
    pub(crate) gif_reuse: bool,
    pub(crate) gif_progressive: bool,
    pub(crate) tiff_quality: i32,
    pub(crate) tiff_compression: ForeignTiffCompression,
    pub(crate) tiff_predictor: ForeignTiffPredictor,
    pub(crate) tiff_pyramid: bool,
    pub(crate) tiff_bitdepth: i32,
    pub(crate) tiff_miniswhite: bool,
    pub(crate) tiff_tile: bool,
    pub(crate) tiff_tile_height: i32,
    pub(crate) tiff_tile_width: i32,
    pub(crate) tiff_xres: f64,
    pub(crate) tiff_yres: f64,
    pub(crate) tiff_resolution_unit: ForeignTiffResunit,
    pub(crate) heif_quality: i32,
    pub(crate) heif_compression: ForeignHeifCompression,
    pub(crate) heif_effort: i32,
    pub(crate) heif_chroma_subsampling: String,
    pub(crate) heif_lossless: bool,
    pub(crate) heif_bitdepth: i32,
    pub(crate) jxl_distance: f64,
    pub(crate) jxl_decoding_tier: i32,
    pub(crate) jxl_effort: i32,
    pub(crate) jxl_lossless: bool,
    pub(crate) raw_depth: BandFormat,
    pub(crate) err: String,
    pub(crate) keep_metadata: i32,
    pub(crate) with_metadata_orientation: i32,
    pub(crate) with_metadata_density: f64,
    pub(crate) with_icc_profile: String,
    pub(crate) with_exif: HashMap<String, String>,
    pub(crate) with_exif_merge: bool,
    pub(crate) with_xmp: String,
    pub(crate) timeout_seconds: u32,
    pub(crate) conv_kernel: Vec<f64>,
    pub(crate) conv_kernel_width: i32,
    pub(crate) conv_kernel_height: i32,
    pub(crate) conv_kernel_scale: f64,
    pub(crate) conv_kernel_offset: f64,
    pub(crate) boolean_descriptor: Option<InputDescriptor>,
    pub(crate) boolean_op: OperationBoolean,
    pub(crate) band_bool_op: OperationBoolean,
    pub(crate) extract_channel: i32,
    pub(crate) remove_alpha: bool,
    pub(crate) ensure_alpha: f64,
    pub(crate) colourspace_pipeline: Interpretation,
    pub(crate) colourspace: Interpretation,
    pub(crate) delay: Vec<i32>,
    pub(crate) loop_: i32,
    pub(crate) tile_size: i32,
    pub(crate) tile_overlap: i32,
    pub(crate) tile_container: ForeignDzContainer,
    pub(crate) tile_layout: ForeignDzLayout,
    pub(crate) tile_format: String,
    pub(crate) tile_angle: i32,
    pub(crate) tile_background: Vec<f64>,
    pub(crate) tile_skip_blanks: i32,
    pub(crate) tile_depth: ForeignDzDepth,
    pub(crate) tile_id: String,
    pub(crate) tile_basename: String,
    pub(crate) recomb_matrix: Vec<f64>,
}

impl Default for PipelineBaton {
    fn default() -> Self {
        Self {
            input: InputDescriptor::default(),
            page_height_out: 0,
            pages_out: 0,
            top_offset_pre: -1,
            top_offset_post: -1,
            channels: 0,
            kernel: Kernel::Lanczos3,
            canvas: Canvas::Crop,
            position: 0,
            resize_background: vec![0.0, 0.0, 0.0, 255.0],
            has_crop_offset: false,
            crop_offset_left: 0,
            crop_offset_top: 0,
            has_attention_center: false,
            attention_x: 0,
            attention_y: 0,
            premultiplied: false,
            tint: vec![-1.0, 0.0, 0.0, 0.0],
            flatten: false,
            flatten_background: vec![0.0, 0.0, 0.0],
            unflatten: false,
            negate: false,
            negate_alpha: true,
            blur_sigma: 0.0,
            brightness: 1.0,
            saturation: 1.0,
            hue: 0,
            lightness: 0.0,
            median_size: 0,
            sharpen_sigma: 0.0,
            sharpen_m1: 1.0,
            sharpen_m2: 2.0,
            sharpen_x1: 2.0,
            sharpen_y2: 10.0,
            sharpen_y3: 20.0,
            threshold: 0,
            threshold_grayscale: true,
            trim_background: Vec::new(),
            trim_threshold: -1.0,
            trim_line_art: false,
            trim_offset_left: 0,
            trim_offset_top: 0,
            linear_a: Vec::new(),
            linear_b: Vec::new(),
            dilate_width: 0,
            erode_width: 0,
            gamma: 0.0,
            greyscale: false,
            normalise: false,
            normalise_lower: 1,
            normalise_upper: 99,
            clahe_width: 0,
            clahe_height: 0,
            clahe_max_slope: 3,
            angle: 0,
            rotation_angle: 0.0,
            rotation_background: vec![0.0, 0.0, 0.0, 255.0],
            flip: false,
            flop: false,
            extend_top: 0,
            extend_bottom: 0,
            extend_left: 0,
            extend_right: 0,
            extend_background: vec![0.0, 0.0, 0.0, 255.0],
            extend_with: Extend::Background,
            without_enlargement: false,
            without_reduction: false,
            affine_matrix: vec![1.0, 0.0, 0.0, 1.0],
            affine_background: vec![0.0, 0.0, 0.0, 255.0],
            affine_idx: 0.0,
            affine_idy: 0.0,
            affine_odx: 0.0,
            affine_ody: 0.0,
            affine_interpolator: "bicubic".to_string(),
            jpeg_quality: 80,
            jpeg_progressive: false,
            jpeg_chroma_subsampling: "4:2:0".to_string(),
            jpeg_trellis_quantisation: false,
            jpeg_quantisation_table: 0,
            jpeg_overshoot_deringing: false,
            jpeg_optimise_scans: false,
            jpeg_optimise_coding: true,
            png_progressive: false,
            png_compression_level: 6,
            png_adaptive_filtering: false,
            png_palette: false,
            png_quality: 100,
            png_effort: 7,
            png_bitdepth: 8,
            png_dither: 1.0,
            jp2_quality: 80,
            jp2_lossless: false,
            jp2_tile_height: 512,
            jp2_tile_width: 512,
            jp2_chroma_subsampling: "4:4:4".to_string(),
            webp_quality: 80,
            webp_alpha_quality: 100,
            webp_near_lossless: false,
            webp_lossless: false,
            webp_smart_subsample: false,
            webp_smart_deblock: false,
            webp_preset: ForeignWebpPreset::Default,
            webp_effort: 4,
            webp_min_size: false,
            webp_mixed: false,
            gif_bitdepth: 8,
            gif_effort: 7,
            gif_dither: 1.0,
            gif_inter_frame_max_error: 0.0,
            gif_inter_palette_max_error: 3.0,
            gif_reuse: true,
            gif_progressive: false,
            tiff_quality: 80,
            tiff_compression: ForeignTiffCompression::Jpeg,
            tiff_predictor: ForeignTiffPredictor::Horizontal,
            tiff_pyramid: false,
            tiff_bitdepth: 8,
            tiff_miniswhite: false,
            tiff_tile: false,
            tiff_tile_height: 256,
            tiff_tile_width: 256,
            tiff_xres: 1.0,
            tiff_yres: 1.0,
            tiff_resolution_unit: ForeignTiffResunit::Inch,
            heif_quality: 50,
            heif_compression: ForeignHeifCompression::Av1,
            heif_effort: 4,
            heif_chroma_subsampling: "4:4:4".to_string(),
            heif_lossless: false,
            heif_bitdepth: 8,
            jxl_distance: 1.0,
            jxl_decoding_tier: 0,
            jxl_effort: 7,
            jxl_lossless: false,
            raw_depth: BandFormat::Uchar,
            keep_metadata: 0,
            with_metadata_orientation: -1,
            with_metadata_density: 0.0,
            with_exif_merge: true,
            with_xmp: String::new(),
            timeout_seconds: 0,
            conv_kernel_width: 0,
            conv_kernel_height: 0,
            conv_kernel_scale: 0.0,
            conv_kernel_offset: 0.0,
            boolean_descriptor: None,
            boolean_op: OperationBoolean::Last,
            band_bool_op: OperationBoolean::Last,
            extract_channel: -1,
            remove_alpha: false,
            ensure_alpha: -1.0,
            colourspace_pipeline: Interpretation::Last,
            colourspace: Interpretation::Last,
            loop_: -1,
            tile_size: 256,
            tile_overlap: 0,
            tile_container: ForeignDzContainer::Fs,
            tile_layout: ForeignDzLayout::Dz,
            tile_angle: 0,
            tile_background: vec![255.0, 255.0, 255.0, 255.0],
            tile_skip_blanks: -1,
            tile_depth: ForeignDzDepth::Last,
            join: Vec::new(),
            format_out: String::new(),
            file_out: String::new(),
            buffer_out: Vec::new(),
            composite: Vec::new(),
            join_channel_in: Vec::new(),
            left_offset_post: 0,
            left_offset_pre: 0,
            width_pre: 0,
            height_pre: 0,
            width_post: 0,
            height_post: 0,
            width: 0,
            height: 0,
            tile_centre: false,
            fast_shrink_on_load: false,
            precision: Precision::Approximate,
            min_ampl: 0.0,
            gamma_out: 0.0,
            rotate_before: false,
            orient_before: false,
            err: String::new(),
            with_icc_profile: String::new(),
            with_exif: HashMap::new(),
            conv_kernel: Vec::new(),
            delay: Vec::new(),
            tile_format: String::new(),
            tile_id: String::new(),
            tile_basename: String::new(),
            recomb_matrix: Vec::new(),
        }
    }
}

pub(crate) fn init_options() -> PipelineBaton {
    PipelineBaton {
        // resize options
        top_offset_pre: -1,
        left_offset_pre: -1,
        width_pre: -1,
        height_pre: -1,
        top_offset_post: -1,
        left_offset_post: -1,
        width_post: -1,
        height_post: -1,
        width: -1,
        height: -1,
        canvas: Canvas::Crop,
        position: 0,
        resize_background: vec![0.0, 0.0, 0.0, 255.0],
        angle: 0,
        rotation_angle: 0.0,
        rotation_background: vec![0.0, 0.0, 0.0, 255.0],
        rotate_before: false,
        orient_before: false,
        flip: false,
        flop: false,
        extend_top: 0,
        extend_bottom: 0,
        extend_left: 0,
        extend_right: 0,
        extend_background: vec![0.0, 0.0, 0.0, 255.0],
        extend_with: Extend::Background,
        without_enlargement: false,
        without_reduction: false,
        affine_matrix: Vec::new(),
        affine_background: vec![0.0, 0.0, 0.0, 255.0],
        affine_idx: 0.0,
        affine_idy: 0.0,
        affine_odx: 0.0,
        affine_ody: 0.0,
        affine_interpolator: "bilinear".to_string(),
        kernel: Kernel::Lanczos3,
        fast_shrink_on_load: true,
        // operations
        tint: vec![-1.0, 0.0, 0.0, 0.0],
        flatten: false,
        flatten_background: vec![0.0, 0.0, 0.0],
        unflatten: false,
        negate: false,
        negate_alpha: true,
        median_size: 0,
        blur_sigma: 0.0,
        precision: Precision::Integer,
        min_ampl: 0.2,
        sharpen_sigma: 0.0,
        sharpen_m1: 1.0,
        sharpen_m2: 2.0,
        sharpen_x1: 2.0,
        sharpen_y2: 10.0,
        sharpen_y3: 20.0,
        threshold: 0,
        threshold_grayscale: true,
        trim_background: Vec::new(),
        trim_threshold: -1.0,
        trim_line_art: false,
        dilate_width: 0,
        erode_width: 0,
        gamma: 0.0,
        gamma_out: 0.0,
        greyscale: false,
        normalise: false,
        normalise_lower: 1,
        normalise_upper: 99,
        clahe_width: 0,
        clahe_height: 0,
        clahe_max_slope: 3,
        brightness: 1.0,
        saturation: 1.0,
        hue: 0,
        lightness: 0.0,
        join_channel_in: vec![],
        extract_channel: -1,
        remove_alpha: false,
        ensure_alpha: -1.0,
        colourspace: Interpretation::Srgb,
        colourspace_pipeline: Interpretation::Last,
        composite: Vec::new(),
        // output
        file_out: String::new(),
        format_out: "input".to_string(),
        keep_metadata: 0,
        with_metadata_orientation: -1,
        with_metadata_density: 0.0,
        with_icc_profile: String::new(),
        with_exif: HashMap::new(),
        with_exif_merge: true,
        loop_: -1,
        delay: Vec::new(),
        // output format
        jpeg_quality: 80,
        jpeg_progressive: false,
        jpeg_chroma_subsampling: "4:2:0".to_string(),
        jpeg_trellis_quantisation: false,
        jpeg_overshoot_deringing: false,
        jpeg_optimise_scans: false,
        jpeg_optimise_coding: true,
        jpeg_quantisation_table: 0,
        png_progressive: false,
        png_compression_level: 6,
        png_adaptive_filtering: false,
        png_palette: false,
        png_quality: 100,
        png_effort: 7,
        png_bitdepth: 8,
        png_dither: 1.0,
        jp2_quality: 80,
        jp2_tile_height: 512,
        jp2_tile_width: 512,
        jp2_lossless: false,
        jp2_chroma_subsampling: "4:4:4".to_string(),
        webp_quality: 80,
        webp_alpha_quality: 100,
        webp_lossless: false,
        webp_near_lossless: false,
        webp_smart_subsample: false,
        webp_smart_deblock: false,
        webp_preset: ForeignWebpPreset::Default,
        webp_effort: 4,
        webp_min_size: false,
        webp_mixed: false,
        gif_bitdepth: 8,
        gif_effort: 7,
        gif_dither: 1.0,
        gif_inter_frame_max_error: 0.0,
        gif_inter_palette_max_error: 3.0,
        gif_reuse: true,
        gif_progressive: false,
        tiff_quality: 80,
        tiff_compression: ForeignTiffCompression::Jpeg,
        tiff_predictor: ForeignTiffPredictor::Horizontal,
        tiff_pyramid: false,
        tiff_miniswhite: false,
        tiff_bitdepth: 8,
        tiff_tile: false,
        tiff_tile_height: 256,
        tiff_tile_width: 256,
        tiff_xres: 1.0,
        tiff_yres: 1.0,
        tiff_resolution_unit: ForeignTiffResunit::Inch,
        heif_quality: 50,
        heif_lossless: false,
        heif_compression: ForeignHeifCompression::Av1,
        heif_effort: 4,
        heif_chroma_subsampling: "4:4:4".to_string(),
        heif_bitdepth: 8,
        jxl_distance: 1.0,
        jxl_decoding_tier: 0,
        jxl_effort: 7,
        jxl_lossless: false,
        raw_depth: BandFormat::Uchar,
        tile_size: 256,
        tile_overlap: 0,
        tile_container: ForeignDzContainer::Fs,
        tile_layout: ForeignDzLayout::Dz,
        tile_format: "last".to_string(),
        tile_depth: ForeignDzDepth::Last,
        tile_angle: 0,
        tile_skip_blanks: -1,
        tile_background: vec![255.0, 255.0, 255.0, 255.0],
        tile_centre: false,
        tile_id: "https://example.com/iiif".to_string(),
        tile_basename: String::new(),
        timeout_seconds: 0,

        ..Default::default()
    }
}

pub(crate) fn pipline(mut baton: PipelineBaton) -> Result<PipelineBaton> {
    let _guard = VipsGuard;

    // Open input
    let (image, input_image_type) = if baton.join.is_empty() {
        open_input(&baton.input)?
    } else {
        let mut images = Vec::new();
        let mut has_alpha = false;
        for join in &baton.join {
            let (image, _) = open_input(join)?;
            let image = ensure_colourspace(image, baton.colourspace_pipeline)?;
            has_alpha |= image.hasalpha();
            images.push(image);
        }
        if has_alpha {
            images = images
                .into_iter()
                .map(|img| {
                    if !img.hasalpha() {
                        ensure_alpha(img, 1.0)
                    } else {
                        Ok(img)
                    }
                })
                .collect::<Result<Vec<VipsImage>>>()?;
        } else {
            baton.input.join_background.pop();
        }

        let input_image_type = ImageType::Png;
        let image = VipsImage::arrayjoin_with_opts(
            images.as_slice(),
            VOption::new()
                .set("across", baton.input.join_across)
                .set("shim", baton.input.join_shim)
                .set("background", baton.input.join_background.as_slice())
                .set("halign", baton.input.join_halign as i32)
                .set("valign", baton.input.join_valign as i32),
        )?;

        if baton.input.join_animated {
            let image = image.copy()?;
            image.set_int(VIPS_META_N_PAGES, images.len() as _)?;
            image.set_int(VIPS_META_PAGE_HEIGHT, image.get_height() / images.len() as i32)?;
            (image, input_image_type)
        } else {
            (image, input_image_type)
        }
    };

    let access = baton.input.access;
    let mut image = ensure_colourspace(image, baton.colourspace_pipeline)?;

    let mut n_pages = baton.input.pages;
    if n_pages == -1 {
        // Resolve the number of pages if we need to render until the end of the document
        n_pages = if image.get_typeof(VIPS_META_N_PAGES)? != 0 {
            image.get_int(VIPS_META_N_PAGES)? - baton.input.page
        } else {
            1
        };
    }

    // Get pre-resize page height
    let mut page_height = image.get_page_height();

    // Calculate angle of rotation
    let (mut auto_rotation, mut auto_flop) = if baton.input.auto_orient {
        // Rotate and flip image according to Exif orientation
        calculate_exif_rotation_and_flop(exif_orientation(&image))
    } else {
        (Angle::D0, false)
    };

    let mut rotation = calculate_angle_rotation(baton.angle);

    // Rotate pre-extract
    let should_rotate_before = baton.rotate_before
        && (rotation != Angle::D0 || baton.flip || baton.flop || baton.rotation_angle != 0.0);
    let should_orient_before =
        (should_rotate_before || baton.orient_before) && (auto_rotation != Angle::D0 || auto_flop);

    if should_orient_before {
        image = stay_sequential(image, auto_rotation != Angle::D0)?;
        if auto_rotation != Angle::D0 {
            if auto_rotation != Angle::D180 {
                multi_page_unsupported(n_pages, "Rotate")?;
            }
            image = image.rot(auto_rotation)?;
            auto_rotation = Angle::D0;
        }
        if auto_flop {
            image = image.flip(Direction::Horizontal)?;
            auto_flop = false;
        }
    }

    if should_rotate_before {
        image = stay_sequential(
            image,
            rotation != Angle::D0 || baton.flip || baton.rotation_angle != 0.0,
        )?;

        if baton.flip {
            image = image.flip(Direction::Vertical)?;
            baton.flip = false;
        }
        if baton.flop {
            image = image.flip(Direction::Horizontal)?;
            baton.flop = false;
        }
        if rotation != Angle::D0 {
            if rotation != Angle::D180 {
                multi_page_unsupported(n_pages, "Rotate")?;
            }
            image = image.rot(rotation)?;
            rotation = Angle::D0;
        }
        if baton.rotation_angle != 0.0 {
            multi_page_unsupported(n_pages, "Rotate")?;
            let (alpha_image, background) = apply_alpha(image, &baton.rotation_background, false)?;
            image = alpha_image;
            image = image.rotate_with_opts(
                baton.rotation_angle,
                VOption::new().set("background", background.as_slice()),
            )?;

            image = VipsImage::copy_memory(image)?;
            baton.rotation_angle = 0.0;
        }
    }

    // Trim
    if baton.trim_threshold >= 0.0 {
        multi_page_unsupported(n_pages, "Trim")?;
        image = stay_sequential(image, true)?;
        image = trim(image, &baton.trim_background, baton.trim_threshold, baton.trim_line_art)?;
        baton.trim_offset_left = image.get_xoffset();
        baton.trim_offset_top = image.get_yoffset();
    }

    // Pre extraction
    if baton.top_offset_pre != -1 {
        image = if n_pages > 1 {
            crop_multi_page(
                image,
                baton.left_offset_pre,
                baton.top_offset_pre,
                baton.width_pre,
                baton.height_pre,
                n_pages,
                &mut page_height,
            )?
        } else {
            image.extract_area(
                baton.left_offset_pre,
                baton.top_offset_pre,
                baton.width_pre,
                baton.height_pre,
            )?
        };
    }

    // Get pre-resize image width and height
    let mut input_width = image.get_width();
    let mut input_height = image.get_height();

    // Is there just one page? Shrink to inputHeight instead
    if n_pages == 1 {
        page_height = input_height;
    }

    // Scaling calculations
    let mut target_resize_width = baton.width;
    let mut target_resize_height = baton.height;

    // When auto-rotating by 90 or 270 degrees, swap the target width and
    // height to ensure the behavior aligns with how it would have been if
    // the rotation had taken place *before* resizing.
    if auto_rotation == Angle::D90 || auto_rotation == Angle::D270 {
        std::mem::swap(&mut target_resize_width, &mut target_resize_height);
    }

    // Shrink to pageHeight, so we work for multi-page images
    let (hshrink, vshrink) = resolve_shrink(
        input_width,
        page_height,
        target_resize_width,
        target_resize_height,
        baton.canvas,
        baton.without_enlargement,
        baton.without_reduction,
    );

    // The jpeg preload shrink.
    let mut jpeg_shrink_on_load = 1;

    // WebP, PDF, SVG scale
    let mut scale = 1.0;

    // Try to reload input using shrink-on-load for JPEG, WebP, SVG and PDF, when:
    //  - the width or height parameters are specified;
    //  - gamma correction doesn't need to be applied;
    //  - trimming or pre-resize extract isn't required;
    //  - input colourspace is not specified;
    let should_pre_shrink = (target_resize_width > 0 || target_resize_height > 0)
        && baton.gamma == 0.0
        && baton.top_offset_pre == -1
        && baton.trim_threshold < 0.0
        && baton.colourspace_pipeline == Interpretation::Last
        && !(should_orient_before || should_rotate_before);

    if should_pre_shrink {
        // The common part of the shrink: the bit by which both axes must be shrunk
        let shrink = hshrink.min(vshrink);

        if input_image_type == ImageType::Jpeg {
            // Leave at least a factor of two for the final resize step, when fastShrinkOnLoad: false
            // for more consistent results and to avoid extra sharpness to the image
            let factor = if baton.fast_shrink_on_load {
                1.0
            } else {
                2.0
            };
            if shrink >= 8.0 * factor {
                jpeg_shrink_on_load = 8;
            } else if shrink >= 4.0 * factor {
                jpeg_shrink_on_load = 4;
            } else if shrink >= 2.0 * factor {
                jpeg_shrink_on_load = 2;
            }
            // Lower shrink-on-load for known libjpeg rounding errors
            if jpeg_shrink_on_load > 1 && shrink as i32 == jpeg_shrink_on_load {
                jpeg_shrink_on_load /= 2;
            }
        } else if input_image_type == ImageType::Webp && baton.fast_shrink_on_load && shrink > 1.0 {
            // Avoid upscaling via webp
            scale = 1.0 / shrink;
        } else if input_image_type == ImageType::SVG || input_image_type == ImageType::PDF {
            scale = 1.0 / shrink;
        }
    }

    // Reload input using shrink-on-load, it'll be an integer shrink
    // factor for jpegload*, a double scale factor for webpload*,
    // pdfload* and svgload*
    let mut image = if jpeg_shrink_on_load > 1 {
        let option = VOption::new()
            .set("access", access as i32)
            .set("shrink", jpeg_shrink_on_load)
            .set("unlimited", baton.input.unlimited)
            .set("fail_on", baton.input.fail_on as i32);

        if !baton.input.buffer.is_empty() {
            // Reload JPEG buffer
            VipsImage::jpegload_buffer_with_opts(&baton.input.buffer, option)?
        } else {
            // Reload JPEG file
            VipsImage::jpegload_with_opts(&baton.input.file, option)?
        }
    } else if scale != 1.0 {
        if input_image_type == ImageType::Webp {
            let option = VOption::new()
                .set("access", access as i32)
                .set("scale", scale)
                .set("fail_on", baton.input.fail_on as i32)
                .set("n", baton.input.pages)
                .set("page", baton.input.page);

            if !baton.input.buffer.is_empty() {
                // Reload WebP buffer
                VipsImage::webpload_buffer_with_opts(&baton.input.buffer, option)?
            } else {
                // Reload WebP file
                VipsImage::webpload_with_opts(&baton.input.file, option)?
            }
        } else if input_image_type == ImageType::SVG {
            let option = VOption::new()
                .set("access", access as i32)
                .set("scale", scale)
                .set("fail_on", baton.input.fail_on as i32)
                .set("unlimited", baton.input.unlimited)
                .set("dpi", baton.input.density);

            let image = if !baton.input.buffer.is_empty() {
                // Reload SVG buffer
                VipsImage::svgload_buffer_with_opts(&baton.input.buffer, option)?
            } else {
                // Reload SVG file
                VipsImage::svgload_with_opts(&baton.input.file, option)?
            };
            let image = set_density(image, baton.input.density)?;
            if image.get_width() > 32767 || image.get_height() > 32767 {
                return Err(OperationError(
                    "Input SVG image will exceed 32767x32767 pixel limit when scaled".to_string(),
                ));
            }
            image
        } else if input_image_type == ImageType::PDF {
            let option = VOption::new()
                .set("n", baton.input.pages)
                .set("page", baton.input.page)
                .set("dpi", baton.input.density)
                .set("background", baton.input.pdf_background.as_slice());

            let image = if !baton.input.buffer.is_empty() {
                // Reload PDF buffer
                VipsImage::pdfload_buffer_with_opts(&baton.input.buffer, option)?
            } else {
                // Reload PDF file
                VipsImage::pdfload_with_opts(&baton.input.file, option)?
            };

            set_density(image, baton.input.density)?
        } else {
            image
        }
    } else {
        if input_image_type == ImageType::SVG
            && (image.get_width() > 32767 || image.get_height() > 32767)
        {
            return Err(OperationError(
                "Input SVG image exceeds 32767x32767 pixel limit".to_string(),
            ));
        }
        image
    };

    if baton.input.auto_orient {
        image = remove_exif_orientation(image)?;
    }

    // Any pre-shrinking may already have been done
    input_width = image.get_width();
    input_height = image.get_height();

    // After pre-shrink, but before the main shrink stage
    // Reuse the initial pageHeight if we didn't pre-shrink
    if should_pre_shrink {
        page_height = image.get_page_height();
    }

    // Shrink to pageHeight, so we work for multi-page images
    let (hshrink, mut vshrink) = resolve_shrink(
        input_width,
        page_height,
        target_resize_width,
        target_resize_height,
        baton.canvas,
        baton.without_enlargement,
        baton.without_reduction,
    );

    let mut target_height = (page_height as f64 / vshrink).round() as i32;
    let mut target_page_height = target_height;

    // In toilet-roll mode, we must adjust vshrink so that we exactly hit
    // pageHeight or we'll have pixels straddling pixel boundaries
    if input_height > page_height {
        target_height *= n_pages;
        vshrink = input_height as f64 / target_height as f64;
    }

    // Ensure we're using a device-independent colour space
    let mut input_profile = None;
    if (baton.keep_metadata as u32 & VipsForeignKeep_VIPS_FOREIGN_KEEP_ICC != 0)
        && baton.with_icc_profile.is_empty()
    {
        // Cache input profile for use with output
        input_profile = get_profile(&image);
        baton.input.ignore_icc = true;
    }

    let processing_profile = if image.get_interpretation()? == Interpretation::Rgb16 {
        "p3"
    } else {
        "srgb"
    };

    if has_profile(&image)
        && image.get_interpretation()? != Interpretation::Labs
        && image.get_interpretation()? != Interpretation::Grey16
        && baton.colourspace_pipeline != Interpretation::Cmyk
        && !baton.input.ignore_icc
    {
        // Convert to sRGB/P3 using embedded profile
        image = image
            .icc_transform_with_opts(
                processing_profile,
                VOption::new()
                    .set("embedded", true)
                    .set(
                        "depth",
                        if is16_bit(image.get_interpretation()?) {
                            16
                        } else {
                            8
                        },
                    )
                    .set("intent", Intent::Perceptual as i32),
            )
            .unwrap_or_else(|_| {
                println!("Invalid embedded profile");
                image
            });
    } else if image.get_interpretation()? == Interpretation::Cmyk
        && baton.colourspace_pipeline != Interpretation::Cmyk
    {
        image = image.icc_transform_with_opts(
            processing_profile,
            VOption::new().set("input_profile", "cmyk").set("intent", Intent::Perceptual as i32),
        )?;
    }

    // Flatten image to remove alpha channel
    if baton.flatten && image.hasalpha() {
        image = flatten(image, &baton.flatten_background)?;
    }

    // Gamma encoding (darken)
    if baton.gamma >= 1.0 && baton.gamma <= 3.0 {
        image = gamma(image, 1.0 / baton.gamma)?;
    }

    // Convert to greyscale (linear, therefore after gamma encoding, if any)
    if baton.greyscale {
        image = image.colourspace(Interpretation::BW)?;
    }

    let should_resize = hshrink != 1.0 || vshrink != 1.0;
    let should_blur = baton.blur_sigma != 0.0;
    let should_conv = baton.conv_kernel_width * baton.conv_kernel_height > 0;
    let should_sharpen = baton.sharpen_sigma != 0.0;
    let should_composite = !baton.composite.is_empty();

    if should_composite && !image.hasalpha() {
        image = ensure_alpha(image, 1.0)?;
    }

    let premultiply_format = image.get_format()?;
    let should_premultiply_alpha =
        image.hasalpha() && (should_resize || should_blur || should_conv || should_sharpen);

    if should_premultiply_alpha {
        image = image.premultiply()?.cast(premultiply_format)?;
    }

    // Resize
    if should_resize {
        image = image.resize_with_opts(
            1.0 / hshrink,
            VOption::new().set("vscale", 1.0 / vshrink).set("kernel", baton.kernel as i32),
        )?;
    }

    image =
        stay_sequential(image, auto_rotation != Angle::D0 || baton.flip || rotation != Angle::D0)?;
    // Auto-rotate post-extract
    if auto_rotation != Angle::D0 {
        if auto_rotation != Angle::D180 {
            multi_page_unsupported(n_pages, "Rotate")?;
        }
        image = image.rot(auto_rotation)?;
    }
    // Mirror vertically (up-down) about the x-axis
    if baton.flip {
        image = image.flip(Direction::Vertical)?;
    }
    // Mirror horizontally (left-right) about the y-axis
    if baton.flop != auto_flop {
        image = image.flip(Direction::Horizontal)?;
    }
    // Rotate post-extract 90-angle
    if rotation != Angle::D0 {
        if rotation != Angle::D180 {
            multi_page_unsupported(n_pages, "Rotate")?;
        }
        image = image.rot(rotation)?;
    }

    // Join additional color channels to the image
    if !baton.join_channel_in.is_empty() {
        for i in 0..baton.join_channel_in.len() {
            baton.join_channel_in[i].access = access;
            let (mut join_image, _) = open_input(&baton.join_channel_in[i])?;
            join_image = ensure_colourspace(join_image, baton.colourspace_pipeline)?;
            image = VipsImage::bandjoin(&[image, join_image])?;
        }
        image =
            image.copy_with_opts(VOption::new().set("interpretation", baton.colourspace as i32))?;
        image = remove_gif_palette(image)?;
    }

    input_width = image.get_width();
    input_height = if n_pages > 1 {
        target_page_height
    } else {
        image.get_height()
    };

    // Resolve dimensions
    if baton.width <= 0 {
        baton.width = input_width;
    }
    if baton.height <= 0 {
        baton.height = input_height;
    }

    // Crop/embed
    if input_width != baton.width || input_height != baton.height {
        if baton.canvas == Canvas::Embed {
            let (alpha_image, background) =
                apply_alpha(image, &baton.resize_background, should_premultiply_alpha)?;
            image = alpha_image;
            // Embed
            let (left, top) = calculate_embed_position(
                input_width,
                input_height,
                baton.width,
                baton.height,
                baton.position,
            );
            let width = std::cmp::max(input_width, baton.width);
            let height = std::cmp::max(input_height, baton.height);

            image = if n_pages > 1 {
                embed_multi_page(
                    image,
                    left,
                    top,
                    width,
                    height,
                    Extend::Background,
                    &background,
                    n_pages,
                    &mut target_page_height,
                )?
            } else {
                image.embed_with_opts(
                    left,
                    top,
                    width,
                    height,
                    VOption::new()
                        .set("extend", Extend::Background as i32)
                        .set("background", background.as_slice()),
                )?
            };
        } else if baton.canvas == Canvas::Crop {
            if baton.width > input_width {
                baton.width = input_width;
            }
            if baton.height > input_height {
                baton.height = input_height;
            }

            // Crop
            if baton.position < 9 {
                // Gravity-based crop
                let (left, top) = calculate_crop(
                    input_width,
                    input_height,
                    baton.width,
                    baton.height,
                    baton.position,
                );
                let width = std::cmp::min(input_width, baton.width);
                let height = std::cmp::min(input_height, baton.height);

                image = if n_pages > 1 {
                    crop_multi_page(
                        image,
                        left,
                        top,
                        width,
                        height,
                        n_pages,
                        &mut target_page_height,
                    )?
                } else {
                    image.extract_area(left, top, width, height)?
                }
            } else {
                // Attention-based or Entropy-based crop
                multi_page_unsupported(n_pages, "Resize strategy")?;
                image = stay_sequential(image, true)?;
                let mut attention_x = 0;
                let mut attention_y = 0;
                image = image.smartcrop_with_opts(
                    baton.width,
                    baton.height,
                    VOption::new()
                        .set(
                            "interesting",
                            if baton.position == 16 {
                                Interesting::Entropy
                            } else {
                                Interesting::Attention
                            } as i32,
                        )
                        .set("premultiplied", should_premultiply_alpha)
                        .set("attention_x", &mut attention_x)
                        .set("attention_y", &mut attention_y),
                )?;
                baton.has_crop_offset = true;
                baton.crop_offset_left = image.get_xoffset();
                baton.crop_offset_top = image.get_yoffset();
                baton.has_attention_center = true;
                baton.attention_x =
                    (attention_x as f64 * jpeg_shrink_on_load as f64 / scale) as i32;
                baton.attention_y =
                    (attention_y as f64 * jpeg_shrink_on_load as f64 / scale) as i32;
            }
        }
    }

    // Rotate post-extract non-90 angle
    if !baton.rotate_before && baton.rotation_angle != 0.0 {
        multi_page_unsupported(n_pages, "Rotate")?;
        image = stay_sequential(image, true)?;

        let (alpha_image, background) =
            apply_alpha(image, &baton.rotation_background, should_premultiply_alpha)?;
        image = alpha_image.rotate_with_opts(
            baton.rotation_angle,
            VOption::new().set("background", background.as_slice()),
        )?;
    }

    // Post extraction
    if baton.top_offset_post != -1 {
        if n_pages > 1 {
            image = crop_multi_page(
                image,
                baton.left_offset_post,
                baton.top_offset_post,
                baton.width_post,
                baton.height_post,
                n_pages,
                &mut target_page_height,
            )?;

            // heightPost is used in the info object, so update to reflect the number of pages
            baton.height_post *= n_pages;
        } else {
            image = image.extract_area(
                baton.left_offset_post,
                baton.top_offset_post,
                baton.width_post,
                baton.height_post,
            )?;
        }
    }

    // Affine transform
    if !baton.affine_matrix.is_empty() {
        multi_page_unsupported(n_pages, "Affine")?;
        image = stay_sequential(image, true)?;
        let (alpha_image, background) =
            apply_alpha(image, &baton.affine_background, should_premultiply_alpha)?;
        let interp = VipsInterpolate::new_from_name(&baton.affine_interpolator)?;
        image = alpha_image.affine_with_opts(
            baton.affine_matrix.as_slice(),
            VOption::new()
                .set("background", background.as_slice())
                .set("idx", baton.affine_idx)
                .set("idy", baton.affine_idy)
                .set("odx", baton.affine_odx)
                .set("ody", baton.affine_ody)
                .set("interpolate", &interp),
        )?;
    }

    // Extend edges
    if baton.extend_top > 0
        || baton.extend_bottom > 0
        || baton.extend_left > 0
        || baton.extend_right > 0
    {
        // Embed
        baton.width = image.get_width() + baton.extend_left + baton.extend_right;
        baton.height = (if n_pages > 1 {
            target_page_height
        } else {
            image.get_height()
        }) + baton.extend_top
            + baton.extend_bottom;

        if baton.extend_with == Extend::Background {
            let (alpha_image, background) =
                apply_alpha(image, &baton.extend_background, should_premultiply_alpha)?;

            image = stay_sequential(alpha_image, n_pages > 1)?;
            image = if n_pages > 1 {
                embed_multi_page(
                    image,
                    baton.extend_left,
                    baton.extend_top,
                    baton.width,
                    baton.height,
                    baton.extend_with,
                    &background,
                    n_pages,
                    &mut target_page_height,
                )?
            } else {
                image.embed_with_opts(
                    baton.extend_left,
                    baton.extend_top,
                    baton.width,
                    baton.height,
                    VOption::new()
                        .set("extend", baton.extend_with as i32)
                        .set("background", background.as_slice()),
                )?
            }
        } else {
            let ignored_background = Vec::with_capacity(1);
            image = stay_sequential(image, true)?;
            image = if n_pages > 1 {
                embed_multi_page(
                    image,
                    baton.extend_left,
                    baton.extend_top,
                    baton.width,
                    baton.height,
                    baton.extend_with,
                    &ignored_background,
                    n_pages,
                    &mut target_page_height,
                )?
            } else {
                image.embed_with_opts(
                    baton.extend_left,
                    baton.extend_top,
                    baton.width,
                    baton.height,
                    VOption::new().set("extend", baton.extend_with as i32),
                )?
            };
        }
    }

    // Median - must happen before blurring, due to the utility of blurring after thresholding
    if baton.median_size > 0 {
        image = image.median(baton.median_size)?;
    }

    // Threshold - must happen before blurring, due to the utility of blurring after thresholding
    // Threshold - must happen before unflatten to enable non-white unflattening
    if baton.threshold != 0 {
        image = threshold(image, baton.threshold as _, baton.threshold_grayscale)?;
    }

    // Dilate - must happen before blurring, due to the utility of dilating after thresholding
    if baton.dilate_width != 0 {
        image = dilate(image, baton.dilate_width)?;
    }

    // Erode - must happen before blurring, due to the utility of eroding after thresholding
    if baton.erode_width != 0 {
        image = erode(image, baton.erode_width)?;
    }

    // Blur
    if should_blur {
        image = blur(image, baton.blur_sigma, baton.precision, baton.min_ampl)?;
    }

    // Unflatten the image
    if baton.unflatten {
        image = unflatten(image)?;
    }

    // Convolve
    if should_conv {
        image = convolve(
            image,
            baton.conv_kernel_width,
            baton.conv_kernel_height,
            baton.conv_kernel_scale,
            baton.conv_kernel_offset,
            &baton.conv_kernel,
        )?;
    }

    // Recomb
    if !baton.recomb_matrix.is_empty() {
        image = recomb(image, &baton.recomb_matrix)?;
    }

    // Modulate
    if baton.brightness != 1.0
        || baton.saturation != 1.0
        || baton.hue != 0
        || baton.lightness != 0.0
    {
        image = modulate(image, baton.brightness, baton.saturation, baton.hue, baton.lightness)?;
    }

    // Sharpen
    if should_sharpen {
        image = sharpen(
            image,
            baton.sharpen_sigma,
            baton.sharpen_m1,
            baton.sharpen_m2,
            baton.sharpen_x1,
            baton.sharpen_y2,
            baton.sharpen_y3,
        )?;
    }

    // Reverse premultiplication after all transformations
    if should_premultiply_alpha {
        image = image.unpremultiply()?.cast(premultiply_format)?;
    }
    baton.premultiplied = should_premultiply_alpha;

    // Composite
    if should_composite {
        let mut images = Vec::new();
        let mut modes = Vec::new();
        let mut xs = Vec::new();
        let mut ys = Vec::new();

        for composite in baton.composite.as_mut_slice() {
            composite.input.access = access;
            let (mut composite_image, _) = open_input(&composite.input)?;

            if composite.input.auto_orient {
                // Respect EXIF Orientation
                let (composite_auto_rotation, composite_auto_flop) =
                    calculate_exif_rotation_and_flop(exif_orientation(&composite_image));

                composite_image = remove_exif_orientation(composite_image)?;
                composite_image =
                    stay_sequential(composite_image, composite_auto_rotation != Angle::D0)?;

                if composite_auto_rotation != Angle::D0 {
                    composite_image = composite_image.rot(composite_auto_rotation)?;
                }
                if composite_auto_flop {
                    composite_image = composite_image.flip(Direction::Horizontal)?;
                }
            }

            // Verify within current dimensions
            if composite_image.get_width() > image.get_width()
                || composite_image.get_height() > image.get_height()
            {
                return Err(OperationError(
                    "Image to composite must have same dimensions or smaller".to_string(),
                ));
            }

            // Check if overlay is tiled
            if composite.tile {
                let mut across = 0;
                let mut down = 0;
                // Use gravity in overlay
                if composite_image.get_width() <= image.get_width() {
                    across =
                        (image.get_width() as f32 / composite_image.get_width() as f32).ceil() as _;
                    // Ensure odd number of tiles across when gravity is centre, north or south
                    if composite.gravity == 0 || composite.gravity == 1 || composite.gravity == 3 {
                        across |= 1;
                    }
                }
                if composite_image.get_height() <= image.get_height() {
                    down = (image.get_height() as f32 / composite_image.get_height() as f32).ceil()
                        as _;
                    // Ensure odd number of tiles down when gravity is centre, east or west
                    if composite.gravity == 0 || composite.gravity == 2 || composite.gravity == 4 {
                        down |= 1;
                    }
                }
                if across != 0 || down != 0 {
                    composite_image = stay_sequential(composite_image, true)?;
                    composite_image = composite_image.replicate(across, down)?;
                    let (left, top) = if composite.has_offset {
                        calculate_crop2(
                            composite_image.get_width(),
                            composite_image.get_height(),
                            image.get_width(),
                            image.get_height(),
                            composite.left,
                            composite.top,
                        )
                    } else {
                        calculate_crop(
                            composite_image.get_width(),
                            composite_image.get_height(),
                            image.get_width(),
                            image.get_height(),
                            composite.gravity,
                        )
                    };
                    composite_image = composite_image.extract_area(
                        left,
                        top,
                        image.get_width(),
                        image.get_height(),
                    )?;
                }
                // gravity was used for extract_area, set it back to its default value of 0
                composite.gravity = 0;
            }
            // Ensure image to composite is with unpremultiplied alpha
            composite_image = ensure_alpha(composite_image, 1.0)?;

            if composite.premultiplied {
                composite_image = composite_image.unpremultiply()?;
            }
            // Calculate position
            let (left, top) = if composite.has_offset {
                // Composite image at given offsets
                if composite.tile {
                    calculate_crop2(
                        image.get_width(),
                        image.get_height(),
                        composite_image.get_width(),
                        composite_image.get_height(),
                        composite.left,
                        composite.top,
                    )
                } else {
                    (composite.left, composite.top)
                }
            } else {
                // Composite image with given gravity
                calculate_crop(
                    image.get_width(),
                    image.get_height(),
                    composite_image.get_width(),
                    composite_image.get_height(),
                    composite.gravity,
                )
            };

            images.push(composite_image);
            modes.push(composite.mode as i32);
            xs.push(left);
            ys.push(top);
        }

        images.insert(0, image);
        image = VipsImage::composite_with_opts(
            images.as_mut_slice(),
            modes.as_mut_slice(),
            VOption::new()
                .set(
                    "compositing_space",
                    if baton.colourspace_pipeline == Interpretation::Last {
                        Interpretation::Srgb
                    } else {
                        baton.colourspace_pipeline
                    } as i32,
                )
                .set("x", xs.as_slice())
                .set("y", ys.as_slice()),
        )?;
        image = remove_gif_palette(image)?;
    }

    // Gamma decoding (brighten)
    if baton.gamma_out >= 1.0 && baton.gamma_out <= 3.0 {
        image = gamma(image, baton.gamma_out)?;
    }

    // Linear adjustment (a * in + b)
    if !baton.linear_a.is_empty() {
        image = linear(image, &baton.linear_a, &baton.linear_b)?;
    }

    // Apply normalisation - stretch luminance to cover full dynamic range
    if baton.normalise {
        image = stay_sequential(image, true)?;
        image = normalise(image, baton.normalise_lower as _, baton.normalise_upper as _)?;
    }

    // Apply contrast limiting adaptive histogram equalization (CLAHE)
    if baton.clahe_width != 0 && baton.clahe_height != 0 {
        image = stay_sequential(image, true)?;
        image = clahe(image, baton.clahe_width, baton.clahe_height, baton.clahe_max_slope)?;
    }

    // Apply bitwise boolean operation between images
    if let Some(boolean_descriptor) = baton.boolean_descriptor.as_mut() {
        boolean_descriptor.access = access;
        let (mut boolean_image, _) = open_input(boolean_descriptor)?;
        boolean_image = ensure_colourspace(boolean_image, baton.colourspace_pipeline)?;
        image = boolean(image, &boolean_image, baton.boolean_op)?;
        image = remove_gif_palette(image)?;
    }

    // Apply per-channel Bandbool bitwise operations after all other operations
    if baton.band_bool_op >= OperationBoolean::And && baton.band_bool_op < OperationBoolean::Last {
        image = bandbool(image, baton.band_bool_op)?;
    }

    // Tint the image
    if baton.tint[0] >= 0.0 {
        image = tint(image, &baton.tint)?;
    }

    // Remove alpha channel, if any
    if baton.remove_alpha {
        image = remove_alpha(image)?;
    }

    // Ensure alpha channel, if missing
    if baton.ensure_alpha != -1.0 {
        image = ensure_alpha(image, baton.ensure_alpha)?;
    }

    // Convert image to sRGB, if not already
    if is16_bit(image.get_interpretation()?) {
        image = image.cast(BandFormat::Ushort)?;
    }

    if image.get_interpretation()? != baton.colourspace {
        // Convert colourspace, pass the current known interpretation so libvips doesn't have to guess
        image = image.colourspace_with_opts(
            baton.colourspace,
            VOption::new().set("source_space", image.get_interpretation()? as i32),
        )?;
        // Transform colours from embedded profile to output profile
        if (baton.keep_metadata as u32 & VipsForeignKeep_VIPS_FOREIGN_KEEP_ICC != 0)
            && baton.colourspace_pipeline != Interpretation::Cmyk
            && baton.with_icc_profile.is_empty()
            && has_profile(&image)
        {
            image = image.icc_transform_with_opts(
                processing_profile,
                VOption::new()
                    .set("embedded", true)
                    .set(
                        "depth",
                        if is16_bit(image.get_interpretation()?) {
                            16
                        } else {
                            8
                        },
                    )
                    .set("intent", Intent::Perceptual as i32),
            )?;
        }
    }

    // Extract channel
    if baton.extract_channel > -1 {
        if baton.extract_channel >= image.get_bands() {
            if baton.extract_channel == 3 && image.hasalpha() {
                baton.extract_channel = image.get_bands() - 1;
            } else {
                let mut error = baton.err.clone();
                error.push_str(&format!(
                    "Cannot extract channel {:?} from  image with channels 0-{:?}",
                    baton.extract_channel,
                    (image.get_bands() - 1)
                ));

                return Err(OperationError(error));
            }
        }
        let colourspace_value = if is16_bit(image.get_interpretation()?) {
            Interpretation::Grey16
        } else {
            Interpretation::BW
        };
        image = image.extract_band(baton.extract_channel)?;
        image =
            image.copy_with_opts(VOption::new().set("interpretation", colourspace_value as i32))?;
    }

    // Apply output ICC profile
    if !baton.with_icc_profile.is_empty() {
        image = image
            .icc_transform_with_opts(
                &baton.with_icc_profile,
                VOption::new()
                    .set("input_profile", processing_profile)
                    .set("embedded", true)
                    .set(
                        "depth",
                        if is16_bit(image.get_interpretation()?) {
                            16
                        } else {
                            8
                        },
                    )
                    .set("intent", Intent::Perceptual as i32),
            )
            .unwrap_or_else(|_| {
                println!("Invalid profile");
                image
            });
    } else if baton.keep_metadata as u32 & VipsForeignKeep_VIPS_FOREIGN_KEEP_ICC != 0 {
        image = set_profile(image, input_profile)?;
    }

    // Negate the colours in the image
    if baton.negate {
        image = negate(image, baton.negate_alpha)?;
    }

    // Override EXIF Orientation tag
    if baton.with_metadata_orientation != -1 {
        image = set_exif_orientation(image, baton.with_metadata_orientation)?;
    }

    // Override pixel density
    if baton.with_metadata_density > 0.0 {
        image = set_density(image, baton.with_metadata_density)?;
    }

    // EXIF key/value pairs
    if baton.keep_metadata as u32 & VipsForeignKeep_VIPS_FOREIGN_KEEP_EXIF != 0 {
        image = image.copy()?;
        if !baton.with_exif_merge {
            image = remove_exif(image);
        }
        for (name, value) in baton.with_exif.iter() {
            image.set_string(name, value)?;
        }
    }

    // XMP buffer
    if (baton.keep_metadata as u32 & VipsForeignKeep_VIPS_FOREIGN_KEEP_XMP != 0)
        && !baton.with_xmp.is_empty()
    {
        image = image.copy()?;
        image.set_blob(VIPS_META_XMP_NAME, baton.with_xmp.as_bytes())?;
    }

    // Number of channels used in output image
    baton.channels = image.get_bands();
    baton.width = image.get_width();
    baton.height = image.get_height();

    image =
        set_animation_properties(image, n_pages, target_page_height, &baton.delay, baton.loop_)?;

    if image.get_typeof(VIPS_META_PAGE_HEIGHT)? == get_g_type(G_TYPE_INT) {
        baton.page_height_out = image.get_int(VIPS_META_PAGE_HEIGHT)?;
        baton.pages_out = image.get_int(VIPS_META_N_PAGES)?;
    }

    if baton.file_out.is_empty() {
        write_to_buffer(image, input_image_type, baton)
    } else {
        write_to_file(image, input_image_type, baton)
    }
}

fn write_to_buffer(
    mut image: VipsImage,
    input_image_type: ImageType,
    mut baton: PipelineBaton,
) -> Result<PipelineBaton> {
    // Buffer output
    set_timeout(&image, baton.timeout_seconds);

    if baton.format_out == "jpeg"
        || (baton.format_out == "input" && input_image_type == ImageType::Jpeg)
    {
        assert_image_type_dimensions(&image, ImageType::Jpeg)?;
        let area = image.jpegsave_buffer_with_opts(
            VOption::new()
                .set("keep", baton.keep_metadata)
                .set("Q", baton.jpeg_quality)
                .set("interlace", baton.jpeg_progressive)
                .set(
                    "subsample_mode",
                    if baton.jpeg_chroma_subsampling == *"4:4:4" {
                        ForeignSubsample::Off
                    } else {
                        ForeignSubsample::On
                    } as i32,
                )
                .set("trellis_quant", baton.jpeg_trellis_quantisation)
                .set("quant_table", baton.jpeg_quantisation_table)
                .set("overshoot_deringing", baton.jpeg_overshoot_deringing)
                .set("optimize_scans", baton.jpeg_optimise_scans)
                .set("optimize_coding", baton.jpeg_optimise_coding),
        )?;

        baton.buffer_out = area;
        baton.format_out = "jpeg".to_string();
        if baton.colourspace == Interpretation::Cmyk {
            baton.channels = std::cmp::min(baton.channels, 4);
        } else {
            baton.channels = std::cmp::min(baton.channels, 3);
        }
    } else if baton.format_out == "jp2"
        || (baton.format_out == "input" && input_image_type == ImageType::JP2)
    {
        // Write JP2 to Buffer
        assert_image_type_dimensions(&image, ImageType::JP2)?;
        let area = image.jp2ksave_buffer_with_opts(
            VOption::new()
                .set("Q", baton.jp2_quality)
                .set("lossless", baton.jp2_lossless)
                .set(
                    "subsample_mode",
                    if baton.jp2_chroma_subsampling == *"4:4:4" {
                        ForeignSubsample::Off
                    } else {
                        ForeignSubsample::On
                    } as i32,
                )
                .set("tile_height", baton.jp2_tile_height)
                .set("tile_width", baton.jp2_tile_width),
        )?;

        baton.buffer_out = area;
        baton.format_out = "jp2".to_string();
    } else if baton.format_out == "png"
        || (baton.format_out == "input"
            && (input_image_type == ImageType::Png || input_image_type == ImageType::SVG))
    {
        // Write PNG to buffer
        assert_image_type_dimensions(&image, ImageType::Png)?;
        let area = image.pngsave_buffer_with_opts(
            VOption::new()
                .set("keep", baton.keep_metadata)
                .set("interlace", baton.png_progressive)
                .set("compression", baton.png_compression_level)
                .set(
                    "filter",
                    if baton.png_adaptive_filtering {
                        ForeignPngFilter::All
                    } else {
                        ForeignPngFilter::None
                    } as i32,
                )
                .set("palette", baton.png_palette)
                .set("Q", baton.png_quality)
                .set("effort", baton.png_effort)
                .set(
                    "bitdepth",
                    if is16_bit(image.get_interpretation()?) {
                        16
                    } else {
                        baton.png_bitdepth
                    },
                )
                .set("dither", baton.png_dither),
        )?;

        baton.buffer_out = area;
        baton.format_out = "png".to_string();
    } else if baton.format_out == "webp"
        || (baton.format_out == "input" && input_image_type == ImageType::Webp)
    {
        // Write WEBP to buffer
        assert_image_type_dimensions(&image, ImageType::Webp)?;
        let area = image.webpsave_buffer_with_opts(
            VOption::new()
                .set("keep", baton.keep_metadata)
                .set("Q", baton.webp_quality)
                .set("lossless", baton.webp_lossless)
                .set("near_lossless", baton.webp_near_lossless)
                .set("smart_subsample", baton.webp_smart_subsample)
                .set("smart_deblock", baton.webp_smart_deblock)
                .set("preset", baton.webp_preset as i32)
                .set("effort", baton.webp_effort)
                .set("min_size", baton.webp_min_size)
                .set("mixed", baton.webp_mixed)
                .set("alpha_q", baton.webp_alpha_quality),
        )?;
        baton.buffer_out = area;
        baton.format_out = "webp".to_string();
    } else if baton.format_out == "gif"
        || (baton.format_out == "input" && input_image_type == ImageType::GIF)
    {
        // Write GIF to buffer
        assert_image_type_dimensions(&image, ImageType::GIF)?;
        let area = image.gifsave_buffer_with_opts(
            VOption::new()
                .set("keep", baton.keep_metadata)
                .set("bitdepth", baton.gif_bitdepth)
                .set("effort", baton.gif_effort)
                .set("reuse", baton.gif_reuse)
                .set("interlace", baton.gif_progressive)
                .set("interframe_maxerror", baton.gif_inter_frame_max_error)
                .set("interpalette_maxerror", baton.gif_inter_palette_max_error)
                .set("dither", baton.gif_dither),
        )?;
        baton.buffer_out = area;
        baton.format_out = "gif".to_string();
    } else if baton.format_out == "tiff"
        || (baton.format_out == "input" && input_image_type == ImageType::Tiff)
    {
        // Write TIFF to buffer
        if baton.tiff_compression == ForeignTiffCompression::Jpeg {
            assert_image_type_dimensions(&image, ImageType::Jpeg)?;
            baton.channels = std::cmp::min(baton.channels, 3);
        }
        // Cast pixel values to float, if required
        if baton.tiff_predictor == ForeignTiffPredictor::Float {
            image = image.cast(BandFormat::Float)?;
        }
        let area = image.tiffsave_buffer_with_opts(
            VOption::new()
                .set("keep", baton.keep_metadata)
                .set("Q", baton.tiff_quality)
                .set("bitdepth", baton.tiff_bitdepth)
                .set("compression", baton.tiff_compression as i32)
                .set("miniswhite", baton.tiff_miniswhite)
                .set("predictor", baton.tiff_predictor as i32)
                .set("pyramid", baton.tiff_pyramid)
                .set("tile", baton.tiff_tile)
                .set("tile_height", baton.tiff_tile_height)
                .set("tile_width", baton.tiff_tile_width)
                .set("xres", baton.tiff_xres)
                .set("yres", baton.tiff_yres)
                .set("resunit", baton.tiff_resolution_unit as i32),
        )?;
        baton.buffer_out = area;
        baton.format_out = "tiff".to_string();
    } else if baton.format_out == "heif"
        || (baton.format_out == "input" && input_image_type == ImageType::HEIF)
    {
        // Write HEIF to buffer
        assert_image_type_dimensions(&image, ImageType::HEIF)?;
        image = remove_animation_properties(image)?;
        let area = image.heifsave_buffer_with_opts(
            VOption::new()
                .set("keep", baton.keep_metadata)
                .set("Q", baton.heif_quality)
                .set("compression", baton.heif_compression as i32)
                .set("effort", baton.heif_effort)
                .set("bitdepth", baton.heif_bitdepth)
                .set(
                    "subsample_mode",
                    if baton.heif_chroma_subsampling == *"4:4:4" {
                        ForeignSubsample::Off
                    } else {
                        ForeignSubsample::On
                    } as i32,
                )
                .set("lossless", baton.heif_lossless),
        )?;
        baton.buffer_out = area;
        baton.format_out = "heif".to_string();
    } else if baton.format_out == "dz" {
        // Write DZ to buffer
        baton.tile_container = ForeignDzContainer::Zip;
        if !image.hasalpha() {
            baton.tile_background.pop();
        }
        image = stay_sequential(image, baton.tile_angle != 0)?;
        let suffix = build_dz_suffix(&baton);
        let mut options = VOption::new()
            .set("keep", baton.keep_metadata)
            .set("tile_size", baton.tile_size)
            .set("overlap", baton.tile_overlap)
            .set("container", baton.tile_container as i32)
            .set("layout", baton.tile_layout as i32)
            .set("suffix", &suffix)
            .set("angle", calculate_angle_rotation(baton.tile_angle) as i32)
            .set("background", baton.tile_background.as_slice())
            .set("centre", baton.tile_centre)
            .set("id", &baton.tile_id)
            .set("skip_blanks", baton.tile_skip_blanks);

        if baton.tile_depth < ForeignDzDepth::Last {
            options.add("depth", baton.tile_depth as i32);
        }
        if !baton.tile_basename.is_empty() {
            options.add("basename", &baton.tile_basename);
        }
        let area = image.dzsave_buffer_with_opts(options)?;
        baton.buffer_out = area;
        baton.format_out = "dz".to_string();
    } else if baton.format_out == "jxl"
        || (baton.format_out == "input" && input_image_type == ImageType::JXL)
    {
        // Write JXL to buffer
        image = remove_animation_properties(image)?;
        let area = image.jxlsave_buffer_with_opts(
            VOption::new()
                .set("keep", baton.keep_metadata)
                .set("distance", baton.jxl_distance)
                .set("tier", baton.jxl_decoding_tier)
                .set("effort", baton.jxl_effort)
                .set("lossless", baton.jxl_lossless),
        )?;
        baton.buffer_out = area;
        baton.format_out = "jxl".to_string();
    } else if baton.format_out == "raw"
        || (baton.format_out == "input" && input_image_type == ImageType::RAW)
    {
        // Write raw, uncompressed image data to buffer
        if baton.greyscale || image.get_interpretation()? == Interpretation::BW {
            // Extract first band for greyscale image
            image = image.extract_band(0)?;
            baton.channels = 1;
        }
        if image.get_format()? != baton.raw_depth {
            // Cast pixels to requested format
            image = image.cast(baton.raw_depth)?;
        }
        // Get raw image data
        let area = image.write_to_memory();
        baton.buffer_out = area;
        if baton.buffer_out.is_empty() {
            baton.err.push_str("Could not allocate enough memory for raw output");
            return Err(OperationError(baton.err.clone()));
        }
        baton.format_out = "raw".to_string();
    } else {
        // Unsupported output format
        baton.err.push_str("Unsupported output format ");
        if baton.format_out == "input" {
            baton.err.push_str("when trying to match input format of ");
            baton.err.push_str(&image_type_id(input_image_type));
        } else {
            baton.err.push_str(&baton.format_out);
        }
        return Err(OperationError(baton.err.clone()));
    }

    Ok(baton)
}

fn write_to_file(
    mut image: VipsImage,
    input_image_type: ImageType,
    mut baton: PipelineBaton,
) -> Result<PipelineBaton> {
    // File output
    set_timeout(&image, baton.timeout_seconds);

    let is_jpeg = is_jpeg(&baton.file_out);
    let is_png = is_png(&baton.file_out);
    let is_webp = is_webp(&baton.file_out);
    let is_gif = is_gif(&baton.file_out);
    let is_tiff = is_tiff(&baton.file_out);
    let is_jp2 = is_jp2(&baton.file_out);
    let is_heif = is_heif(&baton.file_out);
    let is_jxl = is_jxl(&baton.file_out);
    let is_dz = is_dz(&baton.file_out);
    let is_dz_zip = is_dz_zip(&baton.file_out);
    let is_v = is_v(&baton.file_out);
    let might_match_input = baton.format_out == "input";
    let will_match_input = might_match_input
        && !(is_jpeg
            || is_png
            || is_webp
            || is_gif
            || is_tiff
            || is_jp2
            || is_heif
            || is_dz
            || is_dz_zip
            || is_v);

    if baton.format_out == "jpeg"
        || (might_match_input && is_jpeg)
        || (will_match_input && input_image_type == ImageType::Jpeg)
    {
        // Write JPEG to file
        assert_image_type_dimensions(&image, ImageType::Jpeg)?;
        image.jpegsave_with_opts(
            &baton.file_out,
            VOption::new()
                .set("keep", baton.keep_metadata)
                .set("Q", baton.jpeg_quality)
                .set("interlace", baton.jpeg_progressive)
                .set(
                    "subsample_mode",
                    if baton.jpeg_chroma_subsampling == "4:4:4" {
                        ForeignSubsample::Off
                    } else {
                        ForeignSubsample::On
                    } as i32,
                )
                .set("trellis_quant", baton.jpeg_trellis_quantisation)
                .set("quant_table", baton.jpeg_quantisation_table)
                .set("overshoot_deringing", baton.jpeg_overshoot_deringing)
                .set("optimize_scans", baton.jpeg_optimise_scans)
                .set("optimize_coding", baton.jpeg_optimise_coding),
        )?;
        baton.format_out = "jpeg".to_string();
        baton.channels = std::cmp::min(baton.channels, 3);
    } else if baton.format_out == "jp2"
        || (might_match_input && is_jp2)
        || (will_match_input && input_image_type == ImageType::JP2)
    {
        // Write JP2 to file
        assert_image_type_dimensions(&image, ImageType::JP2)?;
        image.jp2ksave_with_opts(
            &baton.file_out,
            VOption::new()
                .set("Q", baton.jp2_quality)
                .set("lossless", baton.jp2_lossless)
                .set(
                    "subsample_mode",
                    if baton.jp2_chroma_subsampling == "4:4:4" {
                        ForeignSubsample::Off
                    } else {
                        ForeignSubsample::On
                    } as i32,
                )
                .set("tile_height", baton.jp2_tile_height)
                .set("tile_width", baton.jp2_tile_width),
        )?;
        baton.format_out = "jp2".to_string();
    } else if baton.format_out == "png"
        || (might_match_input && is_png)
        || (will_match_input
            && (input_image_type == ImageType::Png || input_image_type == ImageType::SVG))
    {
        // Write PNG to file
        assert_image_type_dimensions(&image, ImageType::Png)?;
        image.pngsave_with_opts(
            &baton.file_out,
            VOption::new()
                .set("keep", baton.keep_metadata)
                .set("interlace", baton.png_progressive)
                .set("compression", baton.png_compression_level)
                .set(
                    "filter",
                    if baton.png_adaptive_filtering {
                        ForeignPngFilter::All
                    } else {
                        ForeignPngFilter::None
                    } as i32,
                )
                .set("palette", baton.png_palette)
                .set("Q", baton.png_quality)
                .set(
                    "bitdepth",
                    if is16_bit(image.get_interpretation()?) {
                        16
                    } else {
                        baton.png_bitdepth
                    },
                )
                .set("effort", baton.png_effort)
                .set("dither", baton.png_dither),
        )?;
        baton.format_out = "png".to_string();
    } else if baton.format_out == "webp"
        || (might_match_input && is_webp)
        || (will_match_input && input_image_type == ImageType::Webp)
    {
        // Write WEBP to file
        assert_image_type_dimensions(&image, ImageType::Webp)?;
        image.webpsave_with_opts(
            &baton.file_out,
            VOption::new()
                .set("keep", baton.keep_metadata)
                .set("Q", baton.webp_quality)
                .set("lossless", baton.webp_lossless)
                .set("near_lossless", baton.webp_near_lossless)
                .set("smart_subsample", baton.webp_smart_subsample)
                .set("smart_deblock", baton.webp_smart_deblock)
                .set("preset", baton.webp_preset as i32)
                .set("effort", baton.webp_effort)
                .set("min_size", baton.webp_min_size)
                .set("mixed", baton.webp_mixed)
                .set("alpha_q", baton.webp_alpha_quality),
        )?;
        baton.format_out = "webp".to_string();
    } else if baton.format_out == "gif"
        || (might_match_input && is_gif)
        || (will_match_input && input_image_type == ImageType::GIF)
    {
        // Write GIF to file
        assert_image_type_dimensions(&image, ImageType::GIF)?;
        image.gifsave_with_opts(
            &baton.file_out,
            VOption::new()
                .set("keep", baton.keep_metadata)
                .set("bitdepth", baton.gif_bitdepth)
                .set("effort", baton.gif_effort)
                .set("reuse", baton.gif_reuse)
                .set("interlace", baton.gif_progressive)
                .set("dither", baton.gif_dither),
        )?;
        baton.format_out = "gif".to_string();
    } else if baton.format_out == "tiff"
        || (might_match_input && is_tiff)
        || (will_match_input && input_image_type == ImageType::Tiff)
    {
        // Write TIFF to file
        if baton.tiff_compression == ForeignTiffCompression::Jpeg {
            assert_image_type_dimensions(&image, ImageType::Jpeg)?;
            baton.channels = std::cmp::min(baton.channels, 3);
        }
        // Cast pixel values to float, if required
        if baton.tiff_predictor == ForeignTiffPredictor::Float {
            image = image.cast(BandFormat::Float)?;
        }
        image.tiffsave_with_opts(
            &baton.file_out,
            VOption::new()
                .set("keep", baton.keep_metadata)
                .set("Q", baton.tiff_quality)
                .set("bitdepth", baton.tiff_bitdepth)
                .set("compression", baton.tiff_compression as i32)
                .set("miniswhite", baton.tiff_miniswhite)
                .set("predictor", baton.tiff_predictor as i32)
                .set("pyramid", baton.tiff_pyramid)
                .set("tile", baton.tiff_tile)
                .set("tile_height", baton.tiff_tile_height)
                .set("tile_width", baton.tiff_tile_width)
                .set("xres", baton.tiff_xres)
                .set("yres", baton.tiff_yres)
                .set("resunit", baton.tiff_resolution_unit as i32),
        )?;
        baton.format_out = "tiff".to_string();
    } else if baton.format_out == "heif"
        || (might_match_input && is_heif)
        || (will_match_input && input_image_type == ImageType::HEIF)
    {
        // Write HEIF to file
        assert_image_type_dimensions(&image, ImageType::HEIF)?;
        image = remove_animation_properties(image)?;
        image.heifsave_with_opts(
            &baton.file_out,
            VOption::new()
                .set("keep", baton.keep_metadata)
                .set("Q", baton.heif_quality)
                .set("compression", baton.heif_compression as i32)
                .set("effort", baton.heif_effort)
                .set("bitdepth", baton.heif_bitdepth)
                .set(
                    "subsample_mode",
                    if baton.heif_chroma_subsampling == *"4:4:4" {
                        ForeignSubsample::Off
                    } else {
                        ForeignSubsample::On
                    } as i32,
                )
                .set("lossless", baton.heif_lossless),
        )?;
        baton.format_out = "heif".to_string();
    } else if baton.format_out == "jxl"
        || (might_match_input && is_jxl)
        || (will_match_input && input_image_type == ImageType::JXL)
    {
        // Write JXL to file
        image = remove_animation_properties(image)?;
        image.jxlsave_with_opts(
            &baton.file_out,
            VOption::new()
                .set("keep", baton.keep_metadata)
                .set("distance", baton.jxl_distance)
                .set("tier", baton.jxl_decoding_tier)
                .set("effort", baton.jxl_effort)
                .set("lossless", baton.jxl_lossless),
        )?;
        baton.format_out = "jxl".to_string();
    } else if baton.format_out == "dz" || is_dz || is_dz_zip {
        // Write DZ to file
        if is_dz_zip {
            baton.tile_container = ForeignDzContainer::Zip;
        }
        if !image.hasalpha() {
            baton.tile_background.pop();
        }
        image = stay_sequential(image, baton.tile_angle != 0)?;
        let suffix = build_dz_suffix(&baton);
        let mut options = VOption::new()
            .set("keep", baton.keep_metadata)
            .set("tile_size", baton.tile_size)
            .set("overlap", baton.tile_overlap)
            .set("container", baton.tile_container as i32)
            .set("layout", baton.tile_layout as i32)
            .set("suffix", &suffix)
            .set("angle", calculate_angle_rotation(baton.tile_angle) as i32)
            .set("background", baton.tile_background.as_slice())
            .set("centre", baton.tile_centre)
            .set("id", &baton.tile_id)
            .set("skip_blanks", baton.tile_skip_blanks);

        if baton.tile_depth < ForeignDzDepth::Last {
            options.add("depth", baton.tile_depth as i32);
        }
        if !baton.tile_basename.is_empty() {
            options.add("basename", &baton.tile_basename);
        }
        image.dzsave_with_opts(&baton.file_out, options)?;
        baton.format_out = "dz".to_string();
    } else if baton.format_out == "v"
        || (might_match_input && is_v)
        || (will_match_input && input_image_type == ImageType::VIPS)
    {
        // Write V to file
        image
            .vipssave_with_opts(&baton.file_out, VOption::new().set("keep", baton.keep_metadata))?;
        baton.format_out = "v".to_string();
    } else {
        // Unsupported output format
        baton.err.push_str(&format!("Unsupported output format {}", baton.file_out));
        return Err(OperationError(baton.err.clone()));
    }

    Ok(baton)
}

/*
    Calculate the angle of rotation and need-to-flip for the given Exif orientation
    By default, returns zero, i.e. no rotation.
*/
fn calculate_exif_rotation_and_flop(exif_orientation: i32) -> (Angle, bool) {
    let mut rotate = Angle::D0;
    let mut flop = false;
    match exif_orientation {
        6 => rotate = Angle::D90,
        3 => rotate = Angle::D180,
        8 => rotate = Angle::D270,
        2 => flop = true,
        7 => {
            flop = true;
            rotate = Angle::D270
        }
        4 => {
            flop = true;
            rotate = Angle::D180
        }
        5 => {
            flop = true;
            rotate = Angle::D90
        }
        _ => {}
    }
    (rotate, flop)
}

/*
  Calculate the rotation for the given angle.
  Supports any positive or negative angle that is a multiple of 90.
*/
fn calculate_angle_rotation(angle: i32) -> Angle {
    let mut angle = angle % 360;
    if angle < 0 {
        angle += 360;
    }
    match angle {
        90 => Angle::D90,
        180 => Angle::D180,
        270 => Angle::D270,
        _ => Angle::D0,
    }
}

fn multi_page_unsupported(pages: i32, op: &str) -> Result<()> {
    if pages > 1 {
        Err(OperationError(format!("{} is not supported for multi-page images", op)))
    } else {
        Ok(())
    }
}

/*
  Assemble the suffix argument to dzsave, which is the format (by extname)
  alongside comma-separated arguments to the corresponding `formatsave` vips
  action.
*/
fn assemble_suffix_string(extname: &str, options: Vec<(&str, String)>) -> String {
    let mut arguments = String::new();
    for option in options {
        if !arguments.is_empty() {
            arguments.push(',');
        }
        arguments.push_str(&format!("{}={}", option.0, option.1));
    }
    format!("{}[{}]", extname, arguments)
}

/*
  Build VOption for dzsave
*/
fn build_dz_suffix(baton: &PipelineBaton) -> String {
    // Forward format options through suffix
    if baton.tile_format == "png" {
        let options = vec![
            (
                "interlace",
                if baton.png_progressive {
                    "true".to_string()
                } else {
                    "false".to_string()
                },
            ),
            ("compression", baton.png_compression_level.to_string()),
            (
                "filter",
                if baton.png_adaptive_filtering {
                    "all".to_string()
                } else {
                    "none".to_string()
                },
            ),
        ];
        assemble_suffix_string(".png", options)
    } else if baton.tile_format == "webp" {
        let options = vec![
            ("Q", baton.webp_quality.to_string()),
            ("alpha_q", baton.webp_alpha_quality.to_string()),
            (
                "lossless",
                if baton.webp_lossless {
                    "true".to_string()
                } else {
                    "false".to_string()
                },
            ),
            (
                "near_lossless",
                if baton.webp_near_lossless {
                    "true".to_string()
                } else {
                    "false".to_string()
                },
            ),
            (
                "smart_subsample",
                if baton.webp_smart_subsample {
                    "true".to_string()
                } else {
                    "false".to_string()
                },
            ),
            (
                "smart_deblock",
                if baton.webp_smart_deblock {
                    "true".to_string()
                } else {
                    "false".to_string()
                },
            ),
            ("preset", foreign_webp_preset_string(baton.webp_preset)),
            (
                "min_size",
                if baton.webp_min_size {
                    "true".to_string()
                } else {
                    "false".to_string()
                },
            ),
            (
                "mixed",
                if baton.webp_mixed {
                    "true".to_string()
                } else {
                    "false".to_string()
                },
            ),
            ("effort", baton.webp_effort.to_string()),
        ];
        assemble_suffix_string(".webp", options)
    } else {
        let options = vec![
            ("Q", baton.jpeg_quality.to_string()),
            (
                "interlace",
                if baton.jpeg_progressive {
                    "true".to_string()
                } else {
                    "false".to_string()
                },
            ),
            (
                "subsample_mode",
                if baton.jpeg_chroma_subsampling == "4:4:4" {
                    "off".to_string()
                } else {
                    "on".to_string()
                },
            ),
            (
                "trellis_quant",
                if baton.jpeg_trellis_quantisation {
                    "true".to_string()
                } else {
                    "false".to_string()
                },
            ),
            ("quant_table", baton.jpeg_quantisation_table.to_string()),
            (
                "overshoot_deringing",
                if baton.jpeg_overshoot_deringing {
                    "true".to_string()
                } else {
                    "false".to_string()
                },
            ),
            (
                "optimize_scans",
                if baton.jpeg_optimise_scans {
                    "true".to_string()
                } else {
                    "false".to_string()
                },
            ),
            (
                "optimize_coding",
                if baton.jpeg_optimise_coding {
                    "true".to_string()
                } else {
                    "false".to_string()
                },
            ),
        ];
        let extname = if baton.tile_layout == ForeignDzLayout::Dz {
            ".jpeg"
        } else {
            ".jpg"
        };
        assemble_suffix_string(extname, options)
    }
}
