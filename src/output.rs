#![allow(clippy::unnecessary_unwrap)]
use crate::{
    in_range,
    metadata::{get_metadata, Metadata},
    pipeline::{self, PipelineBaton},
    Colour, InvalidParameterError, Sharp,
};
use num_derive::{FromPrimitive, ToPrimitive};
use rs_vips::{
    bindings::vips_enum_nick,
    ops::{
        BandFormat, ForeignDzContainer, ForeignDzDepth, ForeignDzLayout, ForeignHeifCompression,
        ForeignTiffCompression, ForeignTiffPredictor, ForeignTiffResunit, ForeignWebpPreset,
    },
};
use std::{collections::HashMap, path::Path};

#[derive(Debug, Clone)]
pub struct OutputInfo {
    pub format: String,
    pub width: i32,
    pub height: i32,
    pub channels: i32,
    pub depth: i32,
    pub premultiplied: bool,
    pub crop_offset_left: i32,
    pub crop_offset_top: i32,
    pub attention_x: i32,
    pub attention_y: i32,
    pub trim_offset_left: i32,
    pub trim_offset_top: i32,
    pub page_height: i32,
    pub pages: i32,
}

pub struct WithIccProfileOptions {
    /**  Should the ICC profile be included in the output image metadata? (optional, default true) */
    pub attach: Option<bool>,
}

pub struct Exif {
    pub ifd0: Option<HashMap<String, String>>,
    pub ifd1: Option<HashMap<String, String>>,
    pub ifd2: Option<HashMap<String, String>>,
    pub ifd3: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Default)]
pub struct WriteableMetadata {
    /** i32 of pixels per inch (DPI) */
    pub density: Option<f64>,
    /** Value between 1 and 8, used to update the EXIF Orientation tag. */
    pub orientation: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct AvailableFormat {
    pub id: String,
    pub input: AvailableFormatInput,
    pub output: AvailableFormatOutput,
}

#[derive(Debug, Clone, Default)]
pub struct AvailableFormatInput {
    pub file: bool,
    pub buffer: bool,
    pub stream: bool,
}

#[derive(Debug, Clone, Default)]
pub struct AvailableFormatOutput {
    pub file: bool,
    pub buffer: bool,
    pub stream: bool,
}

#[derive(strum_macros::Display)]
pub enum FormatEnum {
    #[strum(to_string = "avif")]
    Avif,
    #[strum(to_string = "dz")]
    Dz,
    #[strum(to_string = "exr")]
    Exr,
    #[strum(to_string = "fits")]
    Fits,
    #[strum(to_string = "gif")]
    Gif,
    #[strum(to_string = "heif")]
    Heif,
    #[strum(to_string = "input")]
    Input,
    #[strum(to_string = "jpeg")]
    Jpeg,
    #[strum(to_string = "jpg")]
    Jpg,
    #[strum(to_string = "jp2")]
    Jp2,
    #[strum(to_string = "jxl")]
    Jxl,
    #[strum(to_string = "magick")]
    Magick,
    #[strum(to_string = "openslide")]
    Openslide,
    #[strum(to_string = "pdf")]
    Pdf,
    #[strum(to_string = "png")]
    Png,
    #[strum(to_string = "ppm")]
    Ppm,
    #[strum(to_string = "rad")]
    Rad,
    #[strum(to_string = "raw")]
    Raw,
    #[strum(to_string = "svg")]
    Svg,
    #[strum(to_string = "tiff")]
    Tiff,
    #[strum(to_string = "tif")]
    Tif,
    #[strum(to_string = "v")]
    V,
    #[strum(to_string = "webp")]
    Webp,
}

#[derive(Debug, Default)]
pub struct FormatOptions {
    pub output_options: Option<OutputOptions>,
    pub jpeg_options: Option<JpegOptions>,
    pub png_options: Option<PngOptions>,
    pub webp_options: Option<WebpOptions>,
    pub avif_options: Option<AvifOptions>,
    pub heif_options: Option<HeifOptions>,
    pub jxl_options: Option<JxlOptions>,
    pub gif_options: Option<GifOptions>,
    pub jp2_options: Option<Jp2Options>,
    pub tiff_options: Option<TiffOptions>,
    pub tile_options: Option<TileOptions>,
    pub raw_options: Option<RawOptions>,
}

#[derive(Debug, strum_macros::Display)]
pub enum ChromaSubsampling {
    #[strum(to_string = "4:4:4")]
    None,
    #[strum(to_string = "4:2:0")]
    Two,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum BitDepth {
    Eight = 8,
    Ten = 10,
    Twelve = 12,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum TiffBitDepth {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
}

#[derive(Debug, Default)]
pub struct OutputOptions {
    /** Force format output, otherwise attempt to use input format (optional, default true) */
    pub force: Option<bool>,
}

#[derive(Debug, Default)]
pub struct AnimationOptions {
    /** Number of animation iterations, a value between 0 and 65535. Use 0 for infinite animation. (optional, default 0) */
    pub loop_: Option<u32>,
    /** delay(s) between animation frames (in milliseconds), each value between 0 and 65535. (optional) */
    pub delay: Option<u32>,
}

#[derive(Debug, Default)]
pub struct JpegOptions {
    /** Force format output, otherwise attempt to use input format (optional, default true) */
    pub force: Option<bool>,
    /* Quality, integer 1-100 (optional, default 80) */
    pub quality: Option<i32>,
    /** Use progressive (interlace) scan (optional, default false) */
    pub progressive: Option<bool>,
    /** Set to '4:4:4' to prevent chroma subsampling when quality <= 90 (optional, default '4:2:0') */
    pub chroma_subsampling: Option<String>,
    /** Apply trellis quantisation (optional, default  false) */
    pub trellis_quantisation: Option<bool>,
    /** Apply overshoot deringing (optional, default  false) */
    pub overshoot_deringing: Option<bool>,
    /** Optimise progressive scans, forces progressive (optional, default false) */
    pub optimise_scans: Option<bool>,
    /** Optimise Huffman coding tables (optional, default true) */
    pub optimise_coding: Option<bool>,
    /** Quantization table to use, integer 0-8 (optional, default 0) */
    pub quantisation_table: Option<i32>,
    /** Use mozjpeg defaults (optional, default false) */
    pub mozjpeg: Option<bool>,
}

#[derive(Debug, Default)]
pub struct Jp2Options {
    /** Force format output, otherwise attempt to use input format (optional, default true) */
    pub force: Option<bool>,
    /** Quality, integer 1-100 (optional, default 80) */
    pub quality: Option<i32>,
    /** Use lossless compression mode (optional, default false) */
    pub lossless: Option<bool>,
    /** Horizontal tile size (optional, default 512) */
    pub tile_width: Option<i32>,
    /** Vertical tile size (optional, default 512) */
    pub tile_height: Option<i32>,
    /* Set to '4:2:0' to enable chroma subsampling (optional, default '4:4:4') */
    pub chroma_subsampling: Option<ChromaSubsampling>,
}

#[derive(Debug, Default)]
pub struct JxlOptions {
    /** Force format output, otherwise attempt to use input format (optional, default true) */
    pub force: Option<bool>,
    /** Number of animation iterations, a value between 0 and 65535. Use 0 for infinite animation. (optional, default 0) */
    pub loop_: Option<i32>,
    /** delay(s) between animation frames (in milliseconds), each value between 0 and 65535. (optional) */
    pub delay: Option<Vec<i32>>,
    /** Maximum encoding error, between 0 (highest quality) and 15 (lowest quality) (optional, default 1.0) */
    pub distance: Option<f64>,
    /** Calculate distance based on JPEG-like quality, between 1 and 100, overrides distance if specified */
    pub quality: Option<i32>,
    /** Target decode speed tier, between 0 (highest quality) and 4 (lowest quality) (optional, default 0) */
    pub decoding_tier: Option<i32>,
    /** Use lossless compression (optional, default false) */
    pub lossless: Option<bool>,
    /** CPU effort, between 3 (fastest) and 9 (slowest) (optional, default 7) */
    pub effort: Option<i32>,
}

#[derive(Debug, Default)]
pub struct WebpOptions {
    /** Force format output, otherwise attempt to use input format (optional, default true) */
    pub force: Option<bool>,
    /** Number of animation iterations, a value between 0 and 65535. Use 0 for infinite animation. (optional, default 0) */
    pub loop_: Option<i32>,
    /** delay(s) between animation frames (in milliseconds), each value between 0 and 65535. (optional) */
    pub delay: Option<Vec<i32>>,
    /** Quality, integer 1-100 (optional, default 80) */
    pub quality: Option<i32>,
    /** Quality of alpha layer, i32 from 0-100 (optional, default 100) */
    pub alpha_quality: Option<i32>,
    /** Use lossless compression mode (optional, default false) */
    pub lossless: Option<bool>,
    /** Use near_lossless compression mode (optional, default false) */
    pub near_lossless: Option<bool>,
    /** Use high quality chroma subsampling (optional, default false) */
    pub smart_subsample: Option<bool>,
    /** Auto-adjust the deblocking filter, slow but can improve low contrast edges (optional, default false) */
    pub smart_deblock: Option<bool>,
    /** Level of CPU effort to reduce file size, integer 0-6 (optional, default 4) */
    pub effort: Option<i32>,
    /** Prevent use of animation key frames to minimise file size (slow) (optional, default false) */
    pub min_size: Option<bool>,
    /** Allow mixture of lossy and lossless animation frames (slow) (optional, default false) */
    pub mixed: Option<bool>,
    /* Preset options: one of default, photo, picture, drawing, icon, text (optional, default 'default') */
    pub preset: Option<ForeignWebpPreset>,
}

#[derive(Debug, Default)]
pub struct AvifOptions {
    /** Force format output, otherwise attempt to use input format (optional, default true) */
    pub force: Option<bool>,
    /** quality, integer 1-100 (optional, default 50) */
    pub quality: Option<i32>,
    /** use lossless compression (optional, default false) */
    pub lossless: Option<bool>,
    /** Level of CPU effort to reduce file size, between 0 (fastest) and 9 (slowest) (optional, default 4) */
    pub effort: Option<i32>,
    /* set to '4:2:0' to use chroma subsampling, requires libvips v8.11.0 (optional, default '4:4:4') */
    pub chroma_subsampling: Option<ChromaSubsampling>,
    /* Set bitdepth to 8, 10 or 12 bit (optional, default 8) */
    pub bitdepth: Option<BitDepth>,
}

#[derive(Debug, strum_macros::Display)]
pub enum HeifCompression {
    #[strum(to_string = "av1")]
    Av1,
    #[strum(to_string = "hevc")]
    Hevc,
}

#[derive(Debug, Default)]
pub struct HeifOptions {
    /** Force format output, otherwise attempt to use input format (optional, default true) */
    pub force: Option<bool>,
    /** quality, integer 1-100 (optional, default 50) */
    pub quality: Option<i32>,
    /** compression format: av1, hevc (optional, default 'av1') */
    pub compression: Option<ForeignHeifCompression>,
    /** use lossless compression (optional, default false) */
    pub lossless: Option<bool>,
    /** Level of CPU effort to reduce file size, between 0 (fastest) and 9 (slowest) (optional, default 4) */
    pub effort: Option<i32>,
    /** set to '4:2:0' to use chroma subsampling (optional, default '4:4:4') */
    pub chroma_subsampling: Option<ChromaSubsampling>,
    /* Set bitdepth to 8, 10 or 12 bit (optional, default 8) */
    pub bitdepth: Option<BitDepth>,
}

#[derive(Debug, Default)]
pub struct GifOptions {
    /** Force format output, otherwise attempt to use input format (optional, default true) */
    pub force: Option<bool>,
    /** Number of animation iterations, a value between 0 and 65535. Use 0 for infinite animation. (optional, default 0) */
    pub loop_: Option<i32>,
    /** delay(s) between animation frames (in milliseconds), each value between 0 and 65535. (optional) */
    pub delay: Option<Vec<i32>>,
    /** Re-use existing palette, otherwise generate new (slow) */
    pub reuse: Option<bool>,
    /** Use progressive (interlace) scan */
    pub progressive: Option<bool>,
    /** Maximum number of palette entries, including transparency, between 2 and 256 (optional, default 256) */
    pub colours: Option<i32>,
    /** Level of CPU effort to reduce file size, between 1 (fastest) and 10 (slowest) (optional, default 7) */
    pub effort: Option<i32>,
    /** Level of Floyd-Steinberg error diffusion, between 0 (least) and 1 (most) (optional, default 1.0) */
    pub dither: Option<f64>,
    /** Maximum inter-frame error for transparency, between 0 (lossless) and 32 (optional, default 0) */
    pub inter_frame_max_error: Option<f64>,
    /** Maximum inter-palette error for palette reuse, between 0 and 256 (optional, default 3) */
    pub inter_palette_max_error: Option<f64>,
}

#[derive(Debug, Default)]
pub struct TiffOptions {
    /** Force format output, otherwise attempt to use input format (optional, default true) */
    pub force: Option<bool>,
    /** Quality, integer 1-100 (optional, default 80) */
    pub quality: Option<i32>,
    /** Compression options: none, jpeg, deflate, packbits, ccittfax4, lzw, webp, zstd, jp2k (optional, default 'jpeg') */
    pub compression: Option<ForeignTiffCompression>,
    /** Compression predictor options: none, horizontal, float (optional, default 'horizontal') */
    pub predictor: Option<ForeignTiffPredictor>,
    /** Write an image pyramid (optional, default false) */
    pub pyramid: Option<bool>,
    /** Write a tiled tiff (optional, default false) */
    pub tile: Option<bool>,
    /** Horizontal tile size (optional, default 256) */
    pub tile_width: Option<i32>,
    /** Vertical tile size (optional, default 256) */
    pub tile_height: Option<i32>,
    /** Horizontal resolution in pixels/mm (optional, default 1.0) */
    pub xres: Option<f64>,
    /** Vertical resolution in pixels/mm (optional, default 1.0) */
    pub yres: Option<f64>,
    /** Reduce bitdepth to 1, 2 or 4 bit (optional, default 8) */
    pub bitdepth: Option<TiffBitDepth>,
    /** Write 1-bit images as miniswhite (optional, default false) */
    pub miniswhite: Option<bool>,
    /* Resolution unit options: inch, cm (optional, default 'inch') */
    pub resolution_unit: Option<ForeignTiffResunit>,
}

#[derive(Debug, Default)]
pub struct PngOptions {
    /** Force format output, otherwise attempt to use input format (optional, default true) */
    pub force: Option<bool>,
    /** Use progressive (interlace) scan (optional, default false) */
    pub progressive: Option<bool>,
    /** zlib compression level, 0-9 (optional, default 6) */
    pub compression_level: Option<i32>,
    /** Use adaptive row filtering (optional, default false) */
    pub adaptive_filtering: Option<bool>,
    /** Use the lowest i32 of colours needed to achieve given quality (optional, default `100`) */
    pub quality: Option<i32>,
    /** Level of CPU effort to reduce file size, between 1 (fastest) and 10 (slowest), sets palette to true (optional, default 7) */
    pub effort: Option<i32>,
    /** Quantise to a palette-based image with alpha transparency support (optional, default false) */
    pub palette: Option<bool>,
    /** Maximum number of palette entries, including transparency, between 2 and 256 (optional, default 256) */
    pub colours: Option<i32>,
    /**  Level of Floyd-Steinberg error diffusion (optional, default 1.0) */
    pub dither: Option<f64>,
}

impl PngOptions {
    pub(crate) fn png_palette(&self) -> bool {
        if let Some(palette) = self.palette {
            return palette;
        }

        if self.quality.is_some()
            || self.effort.is_some()
            || self.colours.is_some()
            || self.dither.is_some()
        {
            return true;
        }

        false
    }
}

#[derive(Debug, Default)]
pub struct RawOptions {
    /** Force format output, otherwise attempt to use input format (optional, default true) */
    pub force: Option<bool>,
    pub depth: Option<BandFormat>,
}

#[derive(Debug, Default)]
pub struct TileOptions {
    /** Force format output, otherwise attempt to use input format (optional, default true) */
    pub force: Option<bool>,
    /** Tile size in pixels, a value between 1 and 8192. (optional, default 256) */
    pub size: Option<i32>,
    /** Tile overlap in pixels, a value between 0 and 8192. (optional, default 0) */
    pub overlap: Option<i32>,
    /** Tile angle of rotation, must be a multiple of 90. (optional, default 0) */
    pub angle: Option<i32>,
    /** background colour, parsed by the colour module, defaults to white without transparency. (optional, default {r:255,g:255,b:255,alpha:1}) */
    pub background: Option<Colour>,
    /** How deep to make the pyramid, possible values are "onepixel", "onetile" or "one" (default based on layout) */
    pub depth: Option<ForeignDzDepth>,
    /** Threshold to skip tile generation, a value 0 - 255 for 8-bit images or 0 - 65535 for 16-bit images */
    pub skip_blanks: Option<i32>,
    /** Tile container, with value fs (filesystem) or zip (compressed file). (optional, default 'fs') */
    pub container: Option<ForeignDzContainer>,
    /** Filesystem layout, possible values are dz, iiif, iiif3, zoomify or google. (optional, default 'dz') */
    pub layout: Option<ForeignDzLayout>,
    /** Centre image in tile. (optional, default false) */
    pub centre: Option<bool>,
    /** When layout is iiif/iiif3, sets the @id/id attribute of info.json (optional, default 'https://example.com/iiif') */
    pub id: Option<String>,
    /** The name of the directory within the zip file when container is `zip`. */
    pub basename: Option<String>,
}

impl Sharp {
    /**
     * Write output image data to a file.
     *
     * If an explicit output format is not selected, it will be inferred from the extension,
     * with JPEG, PNG, WebP, AVIF, TIFF, GIF, DZI, and libvips" V format supported.
     * Note that raw pixel data is only supported for buffer output.
     *
     * By default all metadata will be removed, which includes EXIF-based orientation.
     * See {@link #withmetadata|withMetadata} for control over self.
     *
     * The caller is responsible for ensuring directory structures and permissions exist.
     *
     */
    pub fn to_file<P: AsRef<Path>>(mut self, file_out: P) -> Result<(), String> {
        let file_out_string = file_out.as_ref().to_string_lossy().to_string();
        if self.options.input.file == file_out_string {
            return Err("Cannot use same file for input and output".to_string());
        }
        self.options.file_out = file_out_string;
        let _ = pipeline::pipline(self.options).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn to_file_with_info<P: AsRef<Path>>(mut self, file_out: P) -> Result<OutputInfo, String> {
        let file_out_string = file_out.as_ref().to_string_lossy().to_string();
        if self.options.input.file == file_out_string {
            return Err("Cannot use same file for input and output".to_string());
        }
        self.options.file_out = file_out_string;
        let baton = pipeline::pipline(self.options).map_err(|e| e.to_string())?;

        Ok(Self::create_output_info(baton))
    }

    pub async fn to_file_async<P: AsRef<Path>>(mut self, file_out: P) -> Result<(), String> {
        let file_out_string = file_out.as_ref().to_string_lossy().to_string();
        if self.options.input.file == file_out_string {
            return Err("Cannot use same file for input and output".to_string());
        }
        self.options.file_out = file_out_string;
        let _ = async_std::task::spawn(async move {
            pipeline::pipline(self.options).map_err(|e| e.to_string())
        })
        .await?;

        Ok(())
    }

    pub async fn to_file_async_with_info<P: AsRef<Path>>(
        mut self,
        file_out: P,
    ) -> Result<OutputInfo, String> {
        let file_out_string = file_out.as_ref().to_string_lossy().to_string();
        if self.options.input.file == file_out_string {
            return Err("Cannot use same file for input and output".to_string());
        }
        self.options.file_out = file_out_string;
        let baton = async_std::task::spawn(async move {
            pipeline::pipline(self.options).map_err(|e| e.to_string())
        })
        .await?;

        Ok(Self::create_output_info(baton))
    }

    /**
     * Write output to a Buffer.
     * JPEG, PNG, WebP, AVIF, TIFF, GIF and raw pixel data output are supported.
     *
     * Use {@link #toformat|toFormat} or one of the format-specific functions such as {@link jpeg}, {@link png} etc. to set the output format.
     *
     * If no explicit format is set, the output format will match the input image, except SVG input which becomes PNG output.
     *
     * By default all metadata will be removed, which includes EXIF-based orientation.
     * See {@link #withmetadata|withMetadata} for control over self.
     *
     */
    pub fn to_buffer(mut self) -> Result<Vec<u8>, String> {
        self.options.file_out = String::new();
        let baton = pipeline::pipline(self.options).map_err(|e| e.to_string())?;
        Ok(baton.buffer_out)
    }

    pub fn to_buffer_with_info(mut self) -> Result<(Vec<u8>, OutputInfo), String> {
        self.options.file_out = String::new();
        let baton = pipeline::pipline(self.options).map_err(|e| e.to_string())?;

        Ok((baton.buffer_out.clone(), Self::create_output_info(baton)))
    }

    pub async fn to_buffer_async(mut self) -> Result<Vec<u8>, String> {
        self.options.file_out = String::new();
        let baton = async_std::task::spawn(async move {
            pipeline::pipline(self.options).map_err(|e| e.to_string())
        })
        .await?;
        Ok(baton.buffer_out)
    }

    pub async fn to_buffer_with_info_async(mut self) -> Result<(Vec<u8>, OutputInfo), String> {
        self.options.file_out = String::new();
        let baton = async_std::task::spawn(async move {
            pipeline::pipline(self.options).map_err(|e| e.to_string())
        })
        .await?;
        Ok((baton.buffer_out.clone(), Self::create_output_info(baton)))
    }

    fn create_output_info(baton: PipelineBaton) -> OutputInfo {
        let mut width = baton.width;
        let mut height = baton.height;
        if baton.top_offset_pre != -1 && (baton.width == -1 || baton.height == -1) {
            width = baton.width_pre;
            height = baton.height_pre;
        }
        if baton.top_offset_post != -1 {
            width = baton.width_post;
            height = baton.height_post;
        }

        OutputInfo {
            format: baton.format_out.clone(),
            width,
            height,
            channels: baton.channels,
            depth: if baton.format_out == "raw" {
                unsafe {
                    vips_enum_nick(
                        rs_vips::bindings::vips_band_format_get_type(),
                        baton.raw_depth as i32,
                    ) as i32
                }
            } else {
                0
            },
            premultiplied: baton.premultiplied,
            crop_offset_left: baton.crop_offset_left,
            crop_offset_top: baton.crop_offset_top,
            attention_x: baton.attention_x,
            attention_y: baton.attention_y,
            trim_offset_left: baton.trim_offset_left,
            trim_offset_top: baton.trim_offset_top,
            page_height: baton.page_height_out,
            pages: baton.pages_out,
        }
    }

    /**
     * Fast access to (uncached) image metadata without decoding any compressed image data.
     */
    pub fn metadata(&self) -> Result<Metadata, String> {
        get_metadata(&self.options.input).map_err(|e| e.to_string())
    }

    /**
     * Keep all EXIF metadata from the input image in the output image.
     *
     * EXIF metadata is unsupported for TIFF output.
     *
     * @since 0.33.0
     *
     * @example
     * let outputWithExif = await sharp(inputWithExif)
     *   .keepExif()
     *   .toBuffer();
     *
     * @returns {Sharp}
     */
    pub fn keep_exif(mut self) -> Self {
        self.options.keep_metadata |= 0b00001;
        self
    }

    /**
     * Set EXIF metadata in the output image, ignoring any EXIF in the input image.
     *
     * @since 0.33.0
     *
     * @example
     * let dataWithExif = await sharp(input)
     *   .withExif({
     *     IFD0: {
     *       Copyright: "The National Gallery"
     *     },
     *     IFD3: {
     *       GPSLatitudeRef: "N",
     *       GPSLatitude: "51/1 30/1 3230/100",
     *       GPSLongitudeRef: "W",
     *       GPSLongitude: "0/1 7/1 4366/100"
     *     }
     *   })
     *   .toBuffer();
     *
     * @param {Object<string, Object<string, string>>} exif Object keyed by IFD0, IFD1 etc. of key/value string pairs to write as EXIF data.
     * @returns {Sharp}
     * @throws {Error} Invalid parameters
     */
    pub fn with_exif(mut self, exif: Exif) -> Self {
        self.apply_exif("IFD0", exif.ifd0.unwrap_or_default());
        self.apply_exif("IFD1", exif.ifd1.unwrap_or_default());
        self.apply_exif("IFD2", exif.ifd2.unwrap_or_default());
        self.apply_exif("IFD3", exif.ifd3.unwrap_or_default());
        self.options.with_exif_merge = false;
        self
    }

    fn apply_exif(&mut self, ifd: &str, values: HashMap<String, String>) {
        for (k, v) in values {
            self.options
                .with_exif
                .insert(format!("exif-{:?}-{:?}", ifd.to_ascii_lowercase(), k), v);
        }
    }

    /**
     * Update EXIF metadata from the input image in the output image.
     *
     * @since 0.33.0
     *
     * @example
     * let dataWithMergedExif = await sharp(inputWithExif)
     *   .withExifMerge({
     *     IFD0: {
     *       Copyright: "The National Gallery"
     *     }
     *   })
     *   .toBuffer();
     *
     * @param {Object<string, Object<string, string>>} exif Object keyed by IFD0, IFD1 etc. of key/value string pairs to write as EXIF data.
     * @returns {Sharp}
     * @throws {Error} Invalid parameters
     */
    pub fn with_exif_merge(mut self, exif: Exif) -> Self {
        self = self.with_exif(exif);
        self.options.with_exif_merge = true;
        self
    }

    /**
     * Keep ICC profile from the input image in the output image.
     *
     * Where necessary, will attempt to convert the output colour space to match the profile.
     *
     * @since 0.33.0
     *
     * @example
     * let outputWithIccProfile = await sharp(inputWithIccProfile)
     *   .keepIccProfile()
     *   .toBuffer();
     *
     * @returns {Sharp}
     */
    pub fn keep_icc_profile(mut self) -> Self {
        self.options.keep_metadata |= 0b01000;
        self
    }

    /**
     * Transform using an ICC profile and attach to the output image.
     *
     * self can either be an absolute filesystem path or
     * built-in profile name (`srgb`, `p3`, `cmyk`).
     *
     * @since 0.33.0
     *
     * @example
     * let outputWithP3 = await sharp(input)
     *   .withIccProfile("p3")
     *   .toBuffer();
     *
     * @param {string} icc - Absolute filesystem path to output ICC profile or built-in profile name (srgb, p3, cmyk).
     * @param {Object} [options]
     * @param {number} [options.attach=true] Should the ICC profile be included in the output image metadata?
     * @returns {Sharp}
     * @throws {Error} Invalid parameters
     */
    pub fn with_icc_profile(mut self, icc: &str, options: Option<WithIccProfileOptions>) -> Self {
        self.options.with_icc_profile = icc.to_string();
        self = self.keep_icc_profile();
        if let Some(options) = options {
            if let Some(attach) = options.attach {
                if !attach {
                    self.options.keep_metadata &= !0b01000;
                }
            }
        }
        self
    }

    /**
     * Keep all metadata (EXIF, ICC, XMP, IPTC) from the input image in the output image.
     *
     * The default behaviour, when `keepMetadata` is not used, is to convert to the device-independent
     * sRGB colour space and strip all metadata, including the removal of any ICC profile.
     *
     * @since 0.33.0
     *
     * @example
     * let outputWithMetadata = await sharp(inputWithMetadata)
     *   .keepMetadata()
     *   .toBuffer();
     *
     * @returns {Sharp}
     */
    pub fn keep_metadata(mut self) -> Self {
        self.options.keep_metadata = 0b11111;
        self
    }

    /**
     * Keep most metadata (EXIF, XMP, IPTC) from the input image in the output image.
     *
     * self will also convert to and add a web-friendly sRGB ICC profile if appropriate.
     *
     * Allows orientation and density to be set or updated.
     *
     * @example
     * let outputSrgbWithMetadata = await sharp(inputRgbWithMetadata)
     *   .withMetadata()
     *   .toBuffer();
     *
     * @example
     * // Set output metadata to 96 DPI
     * let data = await sharp(input)
     *   .withMetadata({ density: 96 })
     *   .toBuffer();
     *
     * @param {Object} [options]
     * @param {number} [options.orientation] Used to update the EXIF `Orientation` tag, integer between 1 and 8.
     * @param {number} [options.density] Number of pixels per inch (DPI).
     * @returns {Sharp}
     * @throws {Error} Invalid parameters
     */
    pub fn with_metadata(mut self, options: Option<WriteableMetadata>) -> Result<Self, String> {
        self = self.keep_metadata();
        self = self.with_icc_profile("srgb", None);
        if let Some(options) = options {
            if let Some(orientation) = options.orientation {
                if !in_range(orientation as _, 1.0, 8.0) {
                    return Err(InvalidParameterError!(
                        "orientation",
                        "integer between 1 and 8",
                        orientation
                    ));
                }
                self.options.with_metadata_orientation = orientation;
            }

            if let Some(density) = options.density {
                if density < 0.0 {
                    return Err(InvalidParameterError!("density", "positive number", density));
                }
                self.options.with_metadata_density = density;
            }
        }
        Ok(self)
    }
    /**
     * Force output to a given format.
     *
     * @example
     * // Convert any input to PNG output
     * let data = await sharp(input)
     *   .toFormat("png")
     *   .toBuffer();
     *
     */
    pub fn to_format(
        self,
        format: FormatEnum,
        options: Option<FormatOptions>,
    ) -> Result<Self, String> {
        let options = options.unwrap_or_default();

        match format {
            FormatEnum::Avif => self.avif(options.avif_options),
            FormatEnum::Dz => self.tile(options.tile_options),
            FormatEnum::Gif => self.gif(options.gif_options),
            FormatEnum::Heif => self.heif(options.heif_options),
            FormatEnum::Jp2 => self.jp2(options.jp2_options),
            FormatEnum::Jpeg | FormatEnum::Jpg => self.jpeg(options.jpeg_options),
            FormatEnum::Jxl => self.jxl(options.jxl_options),
            FormatEnum::Png => self.png(options.png_options),
            FormatEnum::Raw => self.raw(options.raw_options),
            FormatEnum::Tif | FormatEnum::Tiff => self.tiff(options.tiff_options),
            FormatEnum::Webp => self.webp(options.webp_options),
            _ => Err(InvalidParameterError!("format", "Invalid format", format)),
        }
    }

    /**
     * Use these JPEG options for output image.
     *
     * @example
     * // Convert any input to very high quality JPEG output
     * let data = await sharp(input)
     *   .jpeg({
     *     quality: 100,
     *     chromaSubsampling: "4:4:4"
     *   })
     *   .toBuffer();
     *
     * @example
     * // Use mozjpeg to reduce output JPEG file size (slower)
     * let data = await sharp(input)
     *   .jpeg({ mozjpeg: true })
     *   .toBuffer();
     *
     */
    pub fn jpeg(mut self, options: Option<JpegOptions>) -> Result<Self, String> {
        let force = if options.is_none() {
            None
        } else {
            options.as_ref().unwrap().force
        };

        if let Some(options) = options {
            if let Some(quality) = options.quality {
                if !in_range(quality as _, 1.0, 100.0) {
                    return Err(InvalidParameterError!(
                        "quality",
                        "integer between 1 and 100",
                        quality
                    ));
                }
                self.options.jpeg_quality = quality;
            }
            if let Some(progressive) = options.progressive {
                self.options.jpeg_progressive = progressive;
            }
            if let Some(chroma_subsampling) = options.chroma_subsampling {
                self.options.jpeg_chroma_subsampling = chroma_subsampling.to_string();
            }

            if let Some(optimise_coding) = options.optimise_coding {
                self.options.jpeg_optimise_coding = optimise_coding;
            }

            if let Some(mozjpeg) = options.mozjpeg {
                if mozjpeg {
                    self.options.jpeg_trellis_quantisation = true;
                    self.options.jpeg_overshoot_deringing = true;
                    self.options.jpeg_optimise_scans = true;
                    self.options.jpeg_progressive = true;
                    self.options.jpeg_quantisation_table = 3;
                }
            }

            if let Some(trellis_quantisation) = options.trellis_quantisation {
                self.options.jpeg_trellis_quantisation = trellis_quantisation;
            }
            if let Some(overshoot_deringing) = options.overshoot_deringing {
                self.options.jpeg_overshoot_deringing = overshoot_deringing;
            }

            if let Some(optimise_scans) = options.optimise_scans {
                self.options.jpeg_optimise_scans = optimise_scans;
                if optimise_scans {
                    self.options.jpeg_progressive = true;
                }
            }

            if let Some(quantisation_table) = options.quantisation_table {
                if !in_range(quantisation_table as _, 0.0, 8.0) {
                    return Err(InvalidParameterError!(
                        "quantisationTable",
                        "integer between 0 and 8",
                        quantisationTable
                    ));
                }
                self.options.jpeg_quantisation_table = quantisation_table;
            }
        }

        self.update_format_out("jpeg", force);

        Ok(self)
    }

    fn bitdepth_from_colour_count(colours: u32) -> u32 {
        if colours == 0 {
            return 0; // or panic, depending on your requirements
        }

        let log2 = (colours as f64).log2().ceil() as u32;
        let clz = (log2).leading_zeros();
        1 << (31 - clz)
    }

    /**
     * Use these PNG options for output image.
     *
     * By default, PNG output is full colour at 8 bits per pixel.
     *
     * Indexed PNG input at 1, 2 or 4 bits per pixel is converted to 8 bits per pixel.
     * Set `palette` to `true` for slower, indexed PNG output.
     *
     * For 16 bits per pixel output, convert to `rgb16` via
     * {@link /api-colour#tocolourspace|toColourspace}.
     *
     * @example
     * // Convert any input to full colour PNG output
     * let data = await sharp(input)
     *   .png()
     *   .toBuffer();
     *
     * @example
     * // Convert any input to indexed PNG output (slower)
     * let data = await sharp(input)
     *   .png({ palette: true })
     *   .toBuffer();
     *
     * @example
     * // Output 16 bits per pixel RGB(A)
     * let data = await sharp(input)
     *  .toColourspace("rgb16")
     *  .png()
     *  .toBuffer();
     *
     */
    pub fn png(mut self, options: Option<PngOptions>) -> Result<Self, String> {
        let force = if options.is_none() {
            None
        } else {
            options.as_ref().unwrap().force
        };

        if let Some(options) = options {
            if let Some(progressive) = options.progressive {
                self.options.png_progressive = progressive;
            }
            if let Some(compression_level) = options.compression_level {
                if !in_range(compression_level as _, 0.0, 9.0) {
                    return Err(InvalidParameterError!(
                        "compressionLevel",
                        "integer between 0 and 9",
                        compressionLevel
                    ));
                }
                self.options.png_compression_level = compression_level;
            }
            if let Some(adaptive_filtering) = options.adaptive_filtering {
                self.options.png_adaptive_filtering = adaptive_filtering;
            }

            if let Some(colors) = options.colours {
                if !in_range(colors as _, 2.0, 256.0) {
                    return Err(InvalidParameterError!(
                        "colours",
                        "integer between 2 and 256",
                        colors
                    ));
                }
                self.options.png_bitdepth = Self::bitdepth_from_colour_count(colors as _) as _;
            }
            self.options.png_palette = options.png_palette();

            if self.options.png_palette {
                if let Some(quality) = options.quality {
                    if !in_range(quality as _, 0.0, 100.0) {
                        return Err(InvalidParameterError!(
                            "quality",
                            "integer between 0 and 100",
                            quality
                        ));
                    }
                    self.options.png_quality = quality;
                }
                if let Some(effort) = options.effort {
                    if !in_range(effort as _, 1.0, 10.0) {
                        return Err(InvalidParameterError!(
                            "effort",
                            "integer between 1 and 10",
                            effort
                        ));
                    }
                    self.options.png_effort = effort;
                }
                if let Some(dither) = options.dither {
                    if !in_range(dither, 0.0, 1.0) {
                        return Err(InvalidParameterError!(
                            "dither",
                            "number between 0.0 and 1.0",
                            dither
                        ));
                    }
                    self.options.png_dither = dither;
                }
            }
        }

        self.update_format_out("png", force);

        Ok(self)
    }

    /**
     * Use these WebP options for output image.
     *
     * @example
     * // Convert any input to lossless WebP output
     * let data = await sharp(input)
     *   .webp({ lossless: true })
     *   .toBuffer();
     *
     * @example
     * // Optimise the file size of an animated WebP
     * let outputWebp = await sharp(inputWebp, { animated: true })
     *   .webp({ effort: 6 })
     *   .toBuffer();
     *
     */
    pub fn webp(mut self, options: Option<WebpOptions>) -> Result<Self, String> {
        let force = if options.is_none() {
            None
        } else {
            options.as_ref().unwrap().force
        };

        if let Some(options) = options {
            if let Some(quality) = options.quality {
                if !in_range(quality as _, 1.0, 100.0) {
                    return Err(InvalidParameterError!(
                        "quality",
                        "integer between 1 and 100",
                        quality
                    ));
                }
                self.options.webp_quality = quality;
            }
            if let Some(alpha_quality) = options.alpha_quality {
                if !in_range(alpha_quality as _, 0.0, 100.0) {
                    return Err(InvalidParameterError!(
                        "alphaQuality",
                        "integer between 0 and 100",
                        alphaQuality
                    ));
                }
                self.options.webp_alpha_quality = alpha_quality;
            }
            if let Some(lossless) = options.lossless {
                self.options.webp_lossless = lossless;
            }
            if let Some(near_lossless) = options.near_lossless {
                self.options.webp_near_lossless = near_lossless;
            }
            if let Some(smart_subsample) = options.smart_subsample {
                self.options.webp_smart_subsample = smart_subsample;
            }
            if let Some(smart_deblock) = options.smart_deblock {
                self.options.webp_smart_deblock = smart_deblock;
            }
            if let Some(preset) = options.preset {
                match preset {
                    ForeignWebpPreset::Default
                    | ForeignWebpPreset::Photo
                    | ForeignWebpPreset::Picture
                    | ForeignWebpPreset::Drawing
                    | ForeignWebpPreset::Icon
                    | ForeignWebpPreset::Text => {
                        return Err(InvalidParameterError!(
                            "preset",
                            "one of: default, photo, picture, drawing, icon, text",
                            preset
                        ));
                    }
                    _ => {
                        self.options.webp_preset = preset;
                    }
                }
            }
            if let Some(effort) = options.effort {
                if !in_range(effort as _, 0.0, 6.0) {
                    return Err(InvalidParameterError!(
                        "effort",
                        "integer between 0 and 6",
                        effort
                    ));
                }
                self.options.webp_effort = effort;
            }
            if let Some(min_size) = options.min_size {
                self.options.webp_min_size = min_size;
            }
            if let Some(mixed) = options.mixed {
                self.options.webp_mixed = mixed;
            }
            self = self.try_set_animation_options(options.loop_, options.delay)?;
        }

        self.update_format_out("webp", force);

        Ok(self)
    }

    /**
     * Use these GIF options for the output image.
     *
     * The first entry in the palette is reserved for transparency.
     *
     * The palette of the input image will be re-used if possible.
     *
     * @since 0.30.0
     *
     * @example
     * // Convert PNG to GIF
     * await sharp(pngBuffer)
     *   .gif()
     *   .toBuffer();
     *
     * @example
     * // Convert animated WebP to animated GIF
     * await sharp("animated.webp", { animated: true })
     *   .toFile("animated.gif");
     *
     * @example
     * // Create a 128x128, cropped, non-dithered, animated thumbnail of an animated GIF
     * let out = await sharp("in.gif", { animated: true })
     *   .resize({ width: 128, height: 128 })
     *   .gif({ dither: 0 })
     *   .toBuffer();
     *
     * @example
     * // Lossy file size reduction of animated GIF
     * await sharp("in.gif", { animated: true })
     *   .gif({ interFrameMaxError: 8 })
     *   .toFile("optim.gif");
     *
     */
    pub fn gif(mut self, options: Option<GifOptions>) -> Result<Self, String> {
        let force = if options.is_none() {
            None
        } else {
            options.as_ref().unwrap().force
        };

        if let Some(options) = options {
            if let Some(reuse) = options.reuse {
                self.options.gif_reuse = reuse;
            }
            if let Some(progressive) = options.progressive {
                self.options.gif_progressive = progressive;
            }

            if let Some(colors) = options.colours {
                if !in_range(colors as _, 2.0, 256.0) {
                    return Err(InvalidParameterError!(
                        "colours",
                        "integer between 2 and 256",
                        colors
                    ));
                }
                self.options.gif_bitdepth = Self::bitdepth_from_colour_count(colors as _) as _;
            }
            if let Some(effort) = options.effort {
                if !in_range(effort as _, 1.0, 10.0) {
                    return Err(InvalidParameterError!(
                        "effort",
                        "integer between 1 and 10",
                        effort
                    ));
                }
                self.options.gif_effort = effort;
            }
            if let Some(dither) = options.dither {
                if !in_range(dither, 0.0, 1.0) {
                    return Err(InvalidParameterError!(
                        "dither",
                        "number between 0.0 and 1.0",
                        dither
                    ));
                }
                self.options.gif_dither = dither;
            }
            if let Some(inter_frame_max_error) = options.inter_frame_max_error {
                if !in_range(inter_frame_max_error, 0.0, 32.0) {
                    return Err(InvalidParameterError!(
                        "interFrameMaxError",
                        "number between 0.0 and 32.0",
                        inter_frame_max_error
                    ));
                }
                self.options.gif_inter_frame_max_error = inter_frame_max_error;
            }
            if let Some(inter_palette_max_error) = options.inter_palette_max_error {
                if !in_range(inter_palette_max_error, 0.0, 256.0) {
                    return Err(InvalidParameterError!(
                        "interPaletteMaxError",
                        "number between 0.0 and 256.0",
                        inter_palette_max_error
                    ));
                }
                self.options.gif_inter_palette_max_error = inter_palette_max_error;
            }
            self = self.try_set_animation_options(options.loop_, options.delay)?;
        }

        self.update_format_out("gif", force);

        Ok(self)
    }

    /**
     * Use these JP2 options for output image.
     *
     * Requires libvips compiled with support for OpenJPEG.
     * The prebuilt binaries do not include self - see
     * {@link https://sharp.pixelplumbing.com/install#custom-libvips installing a custom libvips}.
     *
     * @example
     * // Convert any input to lossless JP2 output
     * let data = await sharp(input)
     *   .jp2({ lossless: true })
     *   .toBuffer();
     *
     * @example
     * // Convert any input to very high quality JP2 output
     * let data = await sharp(input)
     *   .jp2({
     *     quality: 100,
     *     chromaSubsampling: "4:4:4"
     *   })
     *   .toBuffer();
     *
     * @since 0.29.1
     *
     */
    pub fn jp2(mut self, options: Option<Jp2Options>) -> Result<Self, String> {
        let force = if options.is_none() {
            None
        } else {
            options.as_ref().unwrap().force
        };

        if let Some(options) = options {
            if let Some(quality) = options.quality {
                if !in_range(quality as _, 1.0, 100.0) {
                    return Err(InvalidParameterError!(
                        "quality",
                        "integer between 1 and 100",
                        quality
                    ));
                }
                self.options.jp2_quality = quality;
            }
            if let Some(lossless) = options.lossless {
                self.options.jp2_lossless = lossless;
            }
            if let Some(tile_width) = options.tile_width {
                if !in_range(tile_width as _, 1.0, 32768.0) {
                    return Err(InvalidParameterError!(
                        "tileWidth",
                        "integer between 1 and 32768",
                        tile_width
                    ));
                }
                self.options.jp2_tile_width = tile_width;
            }
            if let Some(tile_height) = options.tile_height {
                if !in_range(tile_height as _, 1.0, 32768.0) {
                    return Err(InvalidParameterError!(
                        "tileHeight",
                        "integer between 1 and 32768",
                        tile_height
                    ));
                }
                self.options.jp2_tile_height = tile_height;
            }
            if let Some(chroma_subsampling) = options.chroma_subsampling {
                self.options.jp2_chroma_subsampling = chroma_subsampling.to_string();
            }
        }

        self.update_format_out("jp2", force);

        Ok(self)
    }

    /**
     * Set animation options if available.
     */
    fn try_set_animation_options(
        mut self,
        loop_: Option<i32>,
        delay: Option<Vec<i32>>,
    ) -> Result<Self, String> {
        if let Some(loop_) = loop_ {
            if !in_range(loop_ as _, 0.0, 65535.0) {
                return Err(InvalidParameterError!("loop", "integer between 0 and 65535", loop_));
            }
            self.options.loop_ = loop_;
        }
        if let Some(delay) = delay {
            for v in &delay {
                if !in_range(*v as _, 0.0, 65535.0) {
                    return Err(InvalidParameterError!(
                        "delay",
                        "integer or an array of integers between 0 and 65535",
                        delay
                    ));
                }
            }
            self.options.delay = delay;
        }

        Ok(self)
    }

    /**
     * Use these TIFF options for output image.
     *
     * The `density` can be set in pixels/inch via {@link #withmetadata|withMetadata}
     * instead of providing `xres` and `yres` in pixels/mm.
     *
     * @example
     * // Convert SVG input to LZW-compressed, 1 bit per pixel TIFF output
     * sharp("input.svg")
     *   .tiff({
     *     compression: "lzw",
     *     bitdepth: 1
     *   })
     *   .toFile("1-bpp-output.tiff")
     *   .then(info => { ... });
     *
     */
    pub fn tiff(mut self, options: Option<TiffOptions>) -> Result<Self, String> {
        let force = if options.is_none() {
            None
        } else {
            options.as_ref().unwrap().force
        };

        if let Some(options) = options {
            if let Some(quality) = options.quality {
                if !in_range(quality as _, 1.0, 100.0) {
                    return Err(InvalidParameterError!(
                        "quality",
                        "integer between 1 and 100",
                        quality
                    ));
                }

                self.options.tiff_quality = quality;
            }
            if let Some(bitdepth) = options.bitdepth {
                self.options.tiff_bitdepth = bitdepth as i32;
            }
            // tiling
            if let Some(tile) = options.tile {
                self.options.tiff_tile = tile;
            }
            if let Some(tile_width) = options.tile_width {
                if tile_width > 0 {
                    self.options.tiff_tile_width = tile_width;
                } else {
                    return Err(InvalidParameterError!(
                        "tileWidth",
                        "integer greater than zero",
                        tile_width
                    ));
                }
            }
            if let Some(tile_height) = options.tile_height {
                if tile_height > 0 {
                    self.options.tiff_tile_height = tile_height;
                } else {
                    return Err(InvalidParameterError!(
                        "tileHeight",
                        "integer greater than zero",
                        tile_height
                    ));
                }
            }
            // miniswhite
            if let Some(miniswhite) = options.miniswhite {
                self.options.tiff_miniswhite = miniswhite;
            }
            // pyramid
            if let Some(pyramid) = options.pyramid {
                self.options.tiff_pyramid = pyramid;
            }
            // resolution
            if let Some(xres) = options.xres {
                if xres > 0.0 {
                    self.options.tiff_xres = xres;
                } else {
                    return Err(InvalidParameterError!("xres", "number greater than zero", xres));
                }
            }
            if let Some(yres) = options.yres {
                if yres > 0.0 {
                    self.options.tiff_yres = yres;
                } else {
                    return Err(InvalidParameterError!("yres", "number greater than zero", yres));
                }
            }
            // compression
            if let Some(compression) = options.compression {
                self.options.tiff_compression = compression;
            }
            // predictor
            if let Some(predictor) = options.predictor {
                self.options.tiff_predictor = predictor;
            }
            // resolutionUnit
            if let Some(resolution_unit) = options.resolution_unit {
                self.options.tiff_resolution_unit = resolution_unit;
            }
        }

        self.update_format_out("tiff", force);

        Ok(self)
    }

    /**
     * Use these AVIF options for output image.
     *
     * AVIF image sequences are not supported.
     * Prebuilt binaries support a bitdepth of 8 only.
     *
     * self feature is experimental on the Windows ARM64 platform
     * and requires a CPU with ARM64v8.4 or later.
     *
     * @example
     * let data = await sharp(input)
     *   .avif({ effort: 2 })
     *   .toBuffer();
     *
     * @example
     * let data = await sharp(input)
     *   .avif({ lossless: true })
     *   .toBuffer();
     *
     * @since 0.27.0
     *
     */
    pub fn avif(mut self, options: Option<AvifOptions>) -> Result<Self, String> {
        let heif_options = if let Some(options) = options {
            Some(HeifOptions {
                force: options.force,
                quality: options.quality,
                compression: Some(ForeignHeifCompression::Av1),
                lossless: options.lossless,
                effort: options.effort,
                chroma_subsampling: options.chroma_subsampling,
                bitdepth: options.bitdepth,
            })
        } else {
            None
        };
        self = self.heif(heif_options)?;

        Ok(self)
    }

    /**
     * Use these HEIF options for output image.
     *
     * Support for patent-encumbered HEIC images using `hevc` compression requires the use of a
     * globally-installed libvips compiled with support for libheif, libde265 and x265.
     *
     * @example
     * let data = await sharp(input)
     *   .heif({ compression: "hevc" })
     *   .toBuffer();
     *
     * @since 0.23.0
     *
     */
    pub fn heif(mut self, options: Option<HeifOptions>) -> Result<Self, String> {
        let force = if options.is_none() {
            None
        } else {
            options.as_ref().unwrap().force
        };

        if let Some(options) = options {
            if let Some(compression) = options.compression {
                self.options.heif_compression = compression;
            }

            if let Some(quality) = options.quality {
                if !in_range(quality as _, 1.0, 100.0) {
                    return Err(InvalidParameterError!(
                        "quality",
                        "integer between 1 and 100",
                        quality
                    ));
                }
                self.options.heif_quality = quality;
            }
            if let Some(lossless) = options.lossless {
                self.options.heif_lossless = lossless;
            }
            if let Some(effort) = options.effort {
                if !in_range(effort as _, 0.0, 9.0) {
                    return Err(InvalidParameterError!(
                        "effort",
                        "integer between 0 and 9",
                        effort
                    ));
                }
                self.options.heif_effort = effort;
            }
            if let Some(chroma_subsampling) = options.chroma_subsampling {
                self.options.heif_chroma_subsampling = chroma_subsampling.to_string();
            }
            if let Some(bitdepth) = options.bitdepth {
                // if options.bitdepth !== 8 && self.letructor.versions.heif {
                //   return Err(InvalidParameterError!("bitdepth when using prebuilt binaries", 8, options.bitdepth));
                // }
                self.options.heif_bitdepth = bitdepth as _;
            }
        }

        self.update_format_out("heif", force);

        Ok(self)
    }

    /**
     * Use these JPEG-XL (JXL) options for output image.
     *
     * self feature is experimental, please do not use in production systems.
     *
     * Requires libvips compiled with support for libjxl.
     * The prebuilt binaries do not include self - see
     * {@link https://sharp.pixelplumbing.com/install#custom-libvips installing a custom libvips}.
     *
     * @since 0.31.3
     *
     */
    pub fn jxl(mut self, options: Option<JxlOptions>) -> Result<Self, String> {
        let force = if options.is_none() {
            None
        } else {
            options.as_ref().unwrap().force
        };

        if let Some(options) = options {
            if let Some(quality) = options.quality {
                if !in_range(quality as _, 1.0, 100.0) {
                    return Err(InvalidParameterError!(
                        "quality",
                        "integer between 1 and 100",
                        quality
                    ));
                }
                let quality = quality as f64;
                // https://github.com/libjxl/libjxl/blob/0aeea7f180bafd6893c1db8072dcb67d2aa5b03d/tools/cjxl_main.cc#L640-L644
                self.options.jxl_distance = if quality >= 30.0 {
                    0.1 + (100.0 - quality) * 0.09
                } else {
                    53.0 / 3000.0 * quality * quality - 23.0 / 20.0 * quality + 25.0
                };
            } else if let Some(distance) = options.distance {
                if !in_range(distance as _, 0.0, 15.0) {
                    return Err(InvalidParameterError!(
                        "distance",
                        "number between 0.0 and 15.0",
                        distance
                    ));
                }
                self.options.jxl_distance = distance;
            }
            if let Some(decoding_tier) = options.decoding_tier {
                if !in_range(decoding_tier as _, 0.0, 4.0) {
                    return Err(InvalidParameterError!(
                        "decodingTier",
                        "integer between 0 and 4",
                        decoding_tier
                    ));
                }
                self.options.jxl_decoding_tier = decoding_tier;
            }
            if let Some(lossless) = options.lossless {
                self.options.jxl_lossless = lossless;
            }
            if let Some(effort) = options.effort {
                if !in_range(effort as _, 1.0, 9.0) {
                    return Err(InvalidParameterError!(
                        "effort",
                        "integer between 1 and 9",
                        effort
                    ));
                }
                self.options.jxl_effort = effort;
            }

            self = self.try_set_animation_options(options.loop_, options.delay)?;
        }

        self.update_format_out("jxl", force);

        Ok(self)
    }

    /**
     * Force output to be raw, uncompressed pixel data.
     * Pixel ordering is left-to-right, top-to-bottom, without padding.
     * Channel ordering will be RGB or RGBA for non-greyscale colourspaces.
     *
     * @example
     * // Extract raw, unsigned 8-bit RGB pixel data from JPEG input
     * let { data, info } = await sharp("input.jpg")
     *   .raw()
     *   .toBuffer({ resolveWithObject: true });
     *
     * @example
     * // Extract alpha channel as raw, unsigned 16-bit pixel data from PNG input
     * let data = await sharp("input.png")
     *   .ensureAlpha()
     *   .extractChannel(3)
     *   .toColourspace("b-w")
     *   .raw({ depth: "ushort" })
     *   .toBuffer();
     *
     */
    pub fn raw(mut self, options: Option<RawOptions>) -> Result<Self, String> {
        let force = if options.is_none() {
            None
        } else {
            options.as_ref().unwrap().force
        };

        if let Some(options) = options {
            if let Some(depth) = options.depth {
                self.options.raw_depth = depth;
            }
        }
        self.update_format_out("raw", force);

        Ok(self)
    }

    /**
     * Use tile-based deep zoom (image pyramid) output.
     *
     * Set the format and options for tile images via the `toFormat`, `jpeg`, `png` or `webp` functions.
     * Use a `.zip` or `.szi` file extension with `toFile` to write to a compressed archive file format.
     *
     * The container will be set to `zip` when the output is a Buffer or Stream, otherwise it will default to `fs`.
     *
     * @example
     *  sharp("input.tiff")
     *   .png()
     *   .tile({
     *     size: 512
     *   })
     *   .toFile("output.dz", function(err, info) {
     *     // output.dzi is the Deep Zoom XML definition
     *     // output_files contains 512x512 tiles grouped by zoom level
     *   });
     *
     * @example
     * let zipFileWithTiles = await sharp(input)
     *   .tile({ basename: "tiles" })
     *   .toBuffer();
     *
     * @example
     * let iiififier = sharp().tile({ layout: "iiif" });
     * readableStream
     *   .pipe(iiififier)
     *   .pipe(writeableStream);
     *
     */
    pub fn tile(mut self, options: Option<TileOptions>) -> Result<Self, String> {
        let force = if options.is_none() {
            None
        } else {
            options.as_ref().unwrap().force
        };

        if let Some(options) = options {
            // Size of square tiles, in pixels
            if let Some(size) = options.size {
                if !in_range(size as _, 1.0, 8192.0) {
                    return Err(InvalidParameterError!("size", "integer between 1 and 8192", size));
                }
                self.options.tile_size = size;
            }
            // Overlap of tiles, in pixels
            if let Some(overlap) = options.overlap {
                if in_range(overlap as _, 0.0, 8192.0) {
                    if overlap > self.options.tile_size {
                        return Err(InvalidParameterError!(
                            "overlap",
                            format!("<= size ({:?})", self.options.tileSize),
                            overlap
                        ));
                    }
                    self.options.tile_overlap = overlap;
                } else {
                    return Err(InvalidParameterError!(
                        "overlap",
                        "integer between 0 and 8192",
                        overlap
                    ));
                }
            }
            // Container
            if let Some(container) = options.container {
                self.options.tile_container = container;
            }
            // Layout
            if let Some(layout) = options.layout {
                self.options.tile_layout = layout;
            }
            // Angle of rotation,
            if let Some(angle) = options.angle {
                if angle % 90 == 0 {
                    self.options.tile_angle = angle;
                } else {
                    return Err(InvalidParameterError!(
                        "angle",
                        "positive/negative multiple of 90",
                        angle
                    ));
                }
            }
            // Background colour
            if let Some(background) = options.background {
                self.options.tile_background = background.rgba;
            }
            // Depth of tiles
            if let Some(depth) = options.depth {
                self.options.tile_depth = depth;
            }
            // Threshold to skip blank tiles
            if let Some(skip_blanks) = options.skip_blanks {
                if !in_range(skip_blanks as _, -1.0, 65535.0) {
                    return Err(InvalidParameterError!(
                        "skipBlanks",
                        "integer between -1 and 255/65535",
                        skip_blanks
                    ));
                }
                self.options.tile_skip_blanks = skip_blanks;
            } else if let Some(layout) = options.layout {
                if layout == ForeignDzLayout::Google {
                    self.options.tile_skip_blanks = 5;
                }
            }
            // Center image in tile
            if let Some(centre) = options.centre {
                self.options.tile_centre = centre;
            }
            // @id attribute for IIIF layout
            if let Some(id) = options.id {
                self.options.tile_id = id;
            }
            // Basename for zip container
            if let Some(basename) = options.basename {
                self.options.tile_basename = basename;
            }
        }
        let formats = ["jpeg".to_string(), "png".to_string(), "webp".to_string()];
        // Format
        let format_out = self.options.format_out.clone();
        if formats.contains(&format_out) {
            self.options.tile_format = format_out;
        } else if self.options.format_out != *"input" {
            return Err(InvalidParameterError!("format", "one of: jpeg, png, webp", format_out));
        }

        self.update_format_out("dz", force);

        Ok(self)
    }

    fn update_format_out(&mut self, formst: &str, force: Option<bool>) {
        if force.is_none() || force.is_some_and(|force| force) {
            self.options.format_out = formst.to_string();
        }
    }
}
