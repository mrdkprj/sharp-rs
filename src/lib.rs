use crate::{
    metadata::Metadata,
    operation::{AffineOptions, BlurOptions, BooleanOptions, ClaheOptions, FlattenOptions, KernelOptions, ModulateOptions, NegateOptions, NormaliseOptions, SharpenOptions, ThresholdOptions},
    pipeline::{init_options, PipelineBaton},
    util::*,
};
use common::{rgba_from_hex, Canvas, InputDescriptor};
use input::{create_input_descriptor, CreateRaw, Input, RotateOptions, SharpOptions};
pub use libvips::ops::{
    BandFormat, FailOn, ForeignDzContainer, ForeignDzDepth, ForeignDzLayout, ForeignHeifCompression, ForeignTiffCompression, ForeignTiffPredictor, ForeignTiffResunit, ForeignWebpPreset,
    OperationBoolean, TextWrap,
};
use metadata::get_metadata;
use num_derive::{FromPrimitive, ToPrimitive};
use operation::{ExtendOptions, Fit, Region, ResizeOptions, TrimOptions};
use std::{collections::HashMap, path::Path};

mod common;
mod icon;
pub mod input;
pub mod metadata;
pub mod operation;
mod pipeline;
mod util;

macro_rules! InvalidParameterError {
    ($name:expr, $expected:expr, $actual:expr) => {
        format!("Expected {:?} for {:?} but received {:?}", stringify!($expected), $name, stringify!($actual))
    };
}

pub(crate) use InvalidParameterError;

#[derive(Debug, Clone, Default)]
pub struct Colour {
    rgba: Vec<f64>,
}

impl Colour {
    pub fn new(r: u32, g: u32, b: u32, alpha: f32) -> Self {
        Self {
            rgba: vec![r as _, g as _, b as _, alpha as _],
        }
    }

    pub fn from_hex(color: u32) -> Self {
        Self {
            rgba: rgba_from_hex(color),
        }
    }
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

pub struct WriteableMetadata {
    /** i32 of pixels per inch (DPI) */
    pub density: Option<f64>,
    /** Value between 1 and 8, used to update the EXIF Orientation tag. */
    pub orientation: Option<i32>,
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
    pub loop_: Option<u32>,
    /** delay(s) between animation frames (in milliseconds), each value between 0 and 65535. (optional) */
    pub delay: Option<u32>,
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
    pub loop_: Option<u32>,
    /** delay(s) between animation frames (in milliseconds), each value between 0 and 65535. (optional) */
    pub delay: Option<u32>,
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
    pub loop_: Option<u32>,
    /** delay(s) between animation frames (in milliseconds), each value between 0 and 65535. (optional) */
    pub delay: Option<u32>,
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

        if self.quality.is_some() || self.effort.is_some() || self.colours.is_some() || self.dither.is_some() {
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

pub struct Sharp {
    options: PipelineBaton,
}

impl Sharp {
    pub fn new(options: SharpOptions) -> Result<Self, String> {
        init("sharp-rs", false)?;
        let mut all_options = init_options();
        all_options.input = create_input_descriptor(Input::None(), Some(options))?;

        Ok(Self {
            options: all_options,
        })
    }

    pub fn new_from_file<P: AsRef<Path>>(filename: P) -> Result<Self, String> {
        Self::new_sharp_from_file(filename, None)
    }

    pub fn new_from_file_with_opts<P: AsRef<Path>>(filename: P, options: SharpOptions) -> Result<Self, String> {
        Self::new_sharp_from_file(filename, Some(options))
    }

    fn new_sharp_from_file<P: AsRef<Path>>(filename: P, options: Option<SharpOptions>) -> Result<Self, String> {
        init("sharp-rs", false)?;
        let mut all_options = init_options();
        all_options.input = create_input_descriptor(Input::Path(filename.as_ref().to_string_lossy().to_string()), options)?;
        Ok(Self {
            options: all_options,
        })
    }

    pub fn new_from_files<P: AsRef<Path>>(files: &[P]) -> Result<Self, String> {
        Self::new_sharp_from_files(files, None)
    }

    pub fn new_from_files_with_opts<P: AsRef<Path>>(files: &[P], options: SharpOptions) -> Result<Self, String> {
        Self::new_sharp_from_files(files, Some(options))
    }

    fn new_sharp_from_files<P: AsRef<Path>>(files: &[P], options: Option<SharpOptions>) -> Result<Self, String> {
        if files.len() <= 1 {
            return Err("Expected at least two images to join".to_string());
        }

        init("sharp-rs", false)?;
        let mut all_options = init_options();
        // Join images together
        let join: Result<Vec<InputDescriptor>, String> = files.iter().map(|file| create_input_descriptor(Input::Path(file.as_ref().to_string_lossy().to_string()), options.clone())).collect();
        all_options.join = join?;

        Ok(Self {
            options: all_options,
        })
    }

    pub fn new_from_buffer(buffer: Vec<u8>) -> Result<Self, String> {
        Self::new_sharp_from_buffer(buffer, None)
    }

    pub fn new_from_buffer_with_opts(buffer: Vec<u8>, options: SharpOptions) -> Result<Self, String> {
        Self::new_sharp_from_buffer(buffer, Some(options))
    }

    fn new_sharp_from_buffer(buffer: Vec<u8>, options: Option<SharpOptions>) -> Result<Self, String> {
        init("sharp-rs", false)?;
        let mut all_options = init_options();
        all_options.input = create_input_descriptor(Input::Buffer(buffer), options)?;

        Ok(Self {
            options: all_options,
        })
    }

    pub fn new_from_buffers(buffers: Vec<Vec<u8>>) -> Result<Self, String> {
        Self::new_sharp_from_buffers(buffers, None)
    }

    pub fn new_from_buffers_with_opts(buffers: Vec<Vec<u8>>, options: SharpOptions) -> Result<Self, String> {
        Self::new_sharp_from_buffers(buffers, Some(options))
    }

    fn new_sharp_from_buffers(buffers: Vec<Vec<u8>>, options: Option<SharpOptions>) -> Result<Self, String> {
        if buffers.len() <= 1 {
            return Err("Expected at least two images to join".to_string());
        }

        init("sharp-rs", false)?;
        let mut all_options = init_options();
        // Join images together
        let join: Result<Vec<InputDescriptor>, String> = buffers.iter().map(|buffer| create_input_descriptor(Input::Buffer(buffer.to_vec()), options.clone())).collect();
        all_options.join = join?;

        Ok(Self {
            options: all_options,
        })
    }

    pub fn cache(cache: bool) {
        if cache {
            cache_set_max_mem(50);
            cache_set_max_files(20);
            cache_set_max(100);
        } else {
            cache_set_max_mem(0);
            cache_set_max_files(0);
            cache_set_max(0);
        }
    }

    pub fn set_cache(memory: u64, files: i32, items: i32) {
        cache_set_max_mem(memory);
        cache_set_max_files(files);
        cache_set_max(items);
    }
    /*
      TODO
        // Get memory stats
    Napi::Object memory = Napi::Object::New(env);
    memory.Set("current", round(vips_tracked_get_mem() / 1048576));
    memory.Set("high", round(vips_tracked_get_mem_highwater() / 1048576));
    memory.Set("max", round(vips_cache_get_max_mem() / 1048576));
    // Get file stats
    Napi::Object files = Napi::Object::New(env);
    files.Set("current", vips_tracked_get_files());
    files.Set("max", vips_cache_get_max_files());

    // Get item stats
    Napi::Object items = Napi::Object::New(env);
    items.Set("current", vips_cache_get_size());
    items.Set("max", vips_cache_get_max());

    Napi::Object cache = Napi::Object::New(env);
    cache.Set("memory", memory);
    cache.Set("files", files);
    cache.Set("items", items);
    return cache;
       */

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
    pub fn to_file<P: AsRef<Path>>(mut self, file_out: P) -> Result<Self, String> {
        let file_out_string = file_out.as_ref().to_string_lossy().to_string();
        if self.options.input.file == file_out_string {
            return Err("Cannot use same file for input and output".to_string());
        }
        self.options.file_out = file_out_string;
        let baton = pipeline::pipline(self.options).map_err(|e| e.to_string())?;
        self.options = baton;

        Ok(self)
    }

    pub async fn to_file_async<P: AsRef<Path>>(mut self, file_out: P) -> Result<Self, String> {
        let file_out_string = file_out.as_ref().to_string_lossy().to_string();
        if self.options.input.file == file_out_string {
            return Err("Cannot use same file for input and output".to_string());
        }
        self.options.file_out = file_out_string;
        let baton = async_std::task::spawn(async move { pipeline::pipline(self.options).map_err(|e| e.to_string()) }).await?;
        self.options = baton;

        Ok(self)
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
        self.options = baton;
        Ok(self.options.buffer_out)
    }

    pub async fn to_buffer_async(mut self) -> Result<Vec<u8>, String> {
        self.options.file_out = String::new();
        let baton = async_std::task::spawn(async move { pipeline::pipline(self.options).map_err(|e| e.to_string()) }).await?;
        self.options = baton;
        Ok(self.options.buffer_out)
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
            self.options.with_exif.insert(format!("exif-{:?}-{:?}", ifd.to_ascii_lowercase(), k), v);
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
                    return Err(InvalidParameterError!("orientation", "integer between 1 and 8", orientation));
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
    pub fn to_format(self, format: FormatEnum, options: Option<FormatOptions>) -> Result<Self, String> {
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
        if let Some(options) = options {
            if let Some(quality) = options.quality {
                if !in_range(quality as _, 1.0, 100.0) {
                    return Err(InvalidParameterError!("quality", "integer between 1 and 100", quality));
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
                    return Err(InvalidParameterError!("quantisationTable", "integer between 0 and 8", quantisationTable));
                }
                self.options.jpeg_quantisation_table = quantisation_table;
            }

            if let Some(force) = options.force {
                if force {
                    self.options.format_out = "jpeg".to_string();
                }
            }
        } else {
            self.options.format_out = "jpeg".to_string();
        }

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
        if let Some(options) = options {
            if let Some(progressive) = options.progressive {
                self.options.png_progressive = progressive;
            }
            if let Some(compression_level) = options.compression_level {
                if !in_range(compression_level as _, 0.0, 9.0) {
                    return Err(InvalidParameterError!("compressionLevel", "integer between 0 and 9", compressionLevel));
                }
                self.options.png_compression_level = compression_level;
            }
            if let Some(adaptive_filtering) = options.adaptive_filtering {
                self.options.png_adaptive_filtering = adaptive_filtering;
            }

            if let Some(colors) = options.colours {
                if !in_range(colors as _, 2.0, 256.0) {
                    return Err(InvalidParameterError!("colours", "integer between 2 and 256", colors));
                }
                self.options.png_bitdepth = Self::bitdepth_from_colour_count(colors as _) as _;
            }
            self.options.png_palette = options.png_palette();

            if self.options.png_palette {
                if let Some(quality) = options.quality {
                    if !in_range(quality as _, 0.0, 100.0) {
                        return Err(InvalidParameterError!("quality", "integer between 0 and 100", quality));
                    }
                    self.options.png_quality = quality;
                }
                if let Some(effort) = options.effort {
                    if !in_range(effort as _, 1.0, 10.0) {
                        return Err(InvalidParameterError!("effort", "integer between 1 and 10", effort));
                    }
                    self.options.png_effort = effort;
                }
                if let Some(dither) = options.dither {
                    if !in_range(dither, 0.0, 1.0) {
                        return Err(InvalidParameterError!("dither", "number between 0.0 and 1.0", dither));
                    }
                    self.options.png_dither = dither;
                }
            }

            if let Some(force) = options.force {
                if force {
                    self.options.format_out = "png".to_string();
                }
            }
        } else {
            self.options.format_out = "png".to_string();
        }

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
        if let Some(options) = options {
            if let Some(quality) = options.quality {
                if !in_range(quality as _, 1.0, 100.0) {
                    return Err(InvalidParameterError!("quality", "integer between 1 and 100", quality));
                }
                self.options.webp_quality = quality;
            }
            if let Some(alpha_quality) = options.alpha_quality {
                if !in_range(alpha_quality as _, 0.0, 100.0) {
                    return Err(InvalidParameterError!("alphaQuality", "integer between 0 and 100", alphaQuality));
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
                    ForeignWebpPreset::Default | ForeignWebpPreset::Photo | ForeignWebpPreset::Picture | ForeignWebpPreset::Drawing | ForeignWebpPreset::Icon | ForeignWebpPreset::Text => {
                        return Err(InvalidParameterError!("preset", "one of: default, photo, picture, drawing, icon, text", preset));
                    }
                    _ => {
                        self.options.webp_preset = preset;
                    }
                }
            }
            if let Some(effort) = options.effort {
                if !in_range(effort as _, 0.0, 6.0) {
                    return Err(InvalidParameterError!("effort", "integer between 0 and 6", effort));
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

            if let Some(force) = options.force {
                if force {
                    self.options.format_out = "webp".to_string();
                }
            }
        } else {
            self.options.format_out = "webp".to_string();
        }

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
        if let Some(options) = options {
            if let Some(reuse) = options.reuse {
                self.options.gif_reuse = reuse;
            }
            if let Some(progressive) = options.progressive {
                self.options.gif_progressive = progressive;
            }

            if let Some(colors) = options.colours {
                if !in_range(colors as _, 2.0, 256.0) {
                    return Err(InvalidParameterError!("colours", "integer between 2 and 256", colors));
                }
                self.options.gif_bitdepth = Self::bitdepth_from_colour_count(colors as _) as _;
            }
            if let Some(effort) = options.effort {
                if !in_range(effort as _, 1.0, 10.0) {
                    return Err(InvalidParameterError!("effort", "integer between 1 and 10", effort));
                }
                self.options.gif_effort = effort;
            }
            if let Some(dither) = options.dither {
                if !in_range(dither, 0.0, 1.0) {
                    return Err(InvalidParameterError!("dither", "number between 0.0 and 1.0", dither));
                }
                self.options.gif_dither = dither;
            }
            if let Some(inter_frame_max_error) = options.inter_frame_max_error {
                if !in_range(inter_frame_max_error, 0.0, 32.0) {
                    return Err(InvalidParameterError!("interFrameMaxError", "number between 0.0 and 32.0", inter_frame_max_error));
                }
                self.options.gif_inter_frame_max_error = inter_frame_max_error;
            }
            if let Some(inter_palette_max_error) = options.inter_palette_max_error {
                if !in_range(inter_palette_max_error, 0.0, 256.0) {
                    return Err(InvalidParameterError!("interPaletteMaxError", "number between 0.0 and 256.0", inter_palette_max_error));
                }
                self.options.gif_inter_palette_max_error = inter_palette_max_error;
            }
            self = self.try_set_animation_options(options.loop_, options.delay)?;
            if let Some(force) = options.force {
                if force {
                    self.options.format_out = "gif".to_string();
                }
            }
        } else {
            self.options.format_out = "gif".to_string();
        }

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
        //   if !self.letructor.format.jp2k.output.buffer {
        //     throw errJp2Save();
        //   }
        if let Some(options) = options {
            if let Some(quality) = options.quality {
                if !in_range(quality as _, 1.0, 100.0) {
                    return Err(InvalidParameterError!("quality", "integer between 1 and 100", quality));
                }
                self.options.jp2_quality = quality;
            }
            if let Some(lossless) = options.lossless {
                self.options.jp2_lossless = lossless;
            }
            if let Some(tile_width) = options.tile_width {
                if !in_range(tile_width as _, 1.0, 32768.0) {
                    return Err(InvalidParameterError!("tileWidth", "integer between 1 and 32768", tile_width));
                }
                self.options.jp2_tile_width = tile_width;
            }
            if let Some(tile_height) = options.tile_height {
                if !in_range(tile_height as _, 1.0, 32768.0) {
                    return Err(InvalidParameterError!("tileHeight", "integer between 1 and 32768", tile_height));
                }
                self.options.jp2_tile_height = tile_height;
            }
            if let Some(chroma_subsampling) = options.chroma_subsampling {
                self.options.jp2_chroma_subsampling = chroma_subsampling.to_string();
            }

            if let Some(force) = options.force {
                if force {
                    self.options.format_out = "jp2".to_string();
                }
            }
        } else {
            self.options.format_out = "jp2".to_string();
        }

        Ok(self)
    }

    /**
     * Set animation options if available.
     */
    fn try_set_animation_options(mut self, loop_: Option<u32>, delay: Option<u32>) -> Result<Self, String> {
        if let Some(loop_) = loop_ {
            if !in_range(loop_ as _, 0.0, 65535.0) {
                return Err(InvalidParameterError!("loop", "integer between 0 and 65535", loop_));
            }
            self.options.loop_ = loop_ as _;
        }
        if let Some(delay) = delay {
            // We allow singular values as well
            if !in_range(delay as _, 0.0, 65535.0) {
                return Err(InvalidParameterError!("delay", "integer or an array of integers between 0 and 65535", delay));
            }
            self.options.delay = vec![delay as _];
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
        if let Some(options) = options {
            if let Some(quality) = options.quality {
                if !in_range(quality as _, 1.0, 100.0) {
                    return Err(InvalidParameterError!("quality", "integer between 1 and 100", quality));
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
                    return Err(InvalidParameterError!("tileWidth", "integer greater than zero", tile_width));
                }
            }
            if let Some(tile_height) = options.tile_height {
                if tile_height > 0 {
                    self.options.tiff_tile_height = tile_height;
                } else {
                    return Err(InvalidParameterError!("tileHeight", "integer greater than zero", tile_height));
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

            if let Some(force) = options.force {
                if force {
                    self.options.format_out = "tiff".to_string();
                }
            }
        } else {
            self.options.format_out = "tiff".to_string();
        }

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
        if let Some(options) = options {
            if let Some(compression) = options.compression {
                self.options.heif_compression = compression;
            }

            if let Some(quality) = options.quality {
                if !in_range(quality as _, 1.0, 100.0) {
                    return Err(InvalidParameterError!("quality", "integer between 1 and 100", quality));
                }
                self.options.heif_quality = quality;
            }
            if let Some(lossless) = options.lossless {
                self.options.heif_lossless = lossless;
            }
            if let Some(effort) = options.effort {
                if !in_range(effort as _, 0.0, 9.0) {
                    return Err(InvalidParameterError!("effort", "integer between 0 and 9", effort));
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

            if let Some(force) = options.force {
                if force {
                    self.options.format_out = "heif".to_string();
                }
            }
        } else {
            self.options.format_out = "heif".to_string();
        }

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
        if let Some(options) = options {
            if let Some(quality) = options.quality {
                if !in_range(quality as _, 1.0, 100.0) {
                    return Err(InvalidParameterError!("quality", "integer between 1 and 100", quality));
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
                    return Err(InvalidParameterError!("distance", "number between 0.0 and 15.0", distance));
                }
                self.options.jxl_distance = distance;
            }
            if let Some(decoding_tier) = options.decoding_tier {
                if !in_range(decoding_tier as _, 0.0, 4.0) {
                    return Err(InvalidParameterError!("decodingTier", "integer between 0 and 4", decoding_tier));
                }
                self.options.jxl_decoding_tier = decoding_tier;
            }
            if let Some(lossless) = options.lossless {
                self.options.jxl_lossless = lossless;
            }
            if let Some(effort) = options.effort {
                if !in_range(effort as _, 1.0, 9.0) {
                    return Err(InvalidParameterError!("effort", "integer between 1 and 9", effort));
                }
                self.options.jxl_effort = effort;
            }

            self = self.try_set_animation_options(options.loop_, options.delay)?;

            if let Some(force) = options.force {
                if force {
                    self.options.format_out = "jxl".to_string();
                }
            }
        } else {
            self.options.format_out = "jxl".to_string();
        }

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
        if let Some(options) = options {
            if let Some(depth) = options.depth {
                self.options.raw_depth = depth;
            }

            if let Some(force) = options.force {
                if force {
                    self.options.format_out = "raw".to_string();
                }
            }
        } else {
            self.options.format_out = "raw".to_string();
        }

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
                        return Err(InvalidParameterError!("overlap", format!("<= size ({:?})", self.options.tileSize), overlap));
                    }
                    self.options.tile_overlap = overlap;
                } else {
                    return Err(InvalidParameterError!("overlap", "integer between 0 and 8192", overlap));
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
                    return Err(InvalidParameterError!("angle", "positive/negative multiple of 90", angle));
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
                    return Err(InvalidParameterError!("skipBlanks", "integer between -1 and 255/65535", skip_blanks));
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

            if let Some(force) = options.force {
                if force {
                    self.options.format_out = "dz".to_string();
                }
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

        self.options.format_out = "dz".to_string();

        Ok(self)
    }

    /**
     * Rotate the output image.
     *
     * The provided angle is converted to a valid positive degree rotation.
     * For example, `-450` will produce a 270 degree rotation.
     *
     * When rotating by an angle other than a multiple of 90,
     * the background colour can be provided with the `background` option.
     *
     * Only one rotation can occur per pipeline (aside from an initial call without
     * arguments to orient via EXIF data). Previous calls to `rotate` in the same
     * pipeline will be ignored.
     *
     * Multi-page images can only be rotated by 180 degrees.
     *
     * Method order is important when rotating, resizing and/or extracting regions,
     * for example `.rotate(x).extract(y)` will produce a different result to `.extract(y).rotate(x)`.
     *
     * @example
     * let rotateThenResize = await sharp(input)
     *   .rotate(90)
     *   .resize({ width: 16, height: 8, fit: "fill" })
     *   .toBuffer();
     * let resizeThenRotate = await sharp(input)
     *   .resize({ width: 16, height: 8, fit: "fill" })
     *   .rotate(90)
     *   .toBuffer();
     *
     */
    pub fn rotate(mut self, angle: i32, options: Option<RotateOptions>) -> Result<Self, String> {
        if self.options.angle > 0 || self.options.rotation_angle > 0.0 {
            self.options.angle = 0;
            self.options.rotation_angle = 0.0;
        }

        if angle % 90 == 0 {
            self.options.angle = angle;
        } else {
            self.options.rotation_angle = angle as _;
            if let Some(option) = options {
                self.options.rotation_background = option.background.rgba;
            }
        }

        Ok(self)
    }

    /**
     * Auto-orient based on the EXIF `Orientation` tag, then remove the tag.
     * Mirroring is supported and may infer the use of a flip operation.
     *
     * Previous or subsequent use of `rotate(angle)` and either `flip()` or `flop()`
     * will logically occur after auto-orientation, regardless of call order.
     *
     * @example
     * let output = await sharp(input).autoOrient().toBuffer();
     *
     * @example
     * let pipeline = sharp()
     *   .autoOrient()
     *   .resize(null, 200)
     *   .toBuffer(pub fn (err, outputBuffer, info) {
     *     // outputBuffer contains 200px high JPEG image data,
     *     // auto-oriented using EXIF Orientation tag
     *     // info.width and info.height contain the dimensions of the resized image
     *   });
     * readableStream.pipe(pipeline);
     */
    pub fn auto_orient(mut self) -> Result<Self, String> {
        self.options.input.auto_orient = true;
        Ok(self)
    }

    /**
     * Mirror the image vertically (up-down) about the x-axis.
     * self always occurs before rotation, if any.
     *
     * self operation does not work correctly with multi-page images.
     *
     * @example
     * let output = await sharp(input).flip().toBuffer();
     */
    pub fn flip(mut self, flip: bool) -> Result<Self, String> {
        self.options.flip = flip;
        Ok(self)
    }

    /**
     * Mirror the image horizontally (left-right) about the y-axis.
     * self always occurs before rotation, if any.
     *
     * @example
     * let output = await sharp(input).flop().toBuffer();
     */
    pub fn flop(mut self, flop: bool) -> Result<Self, String> {
        self.options.flop = flop;
        Ok(self)
    }

    /**
     * Resize image to `width`, `height` or `width x height`.
     *
     * When both a `width` and `height` are provided, the possible methods by which the image should **fit** these are:
     * - `cover`: (default) Preserving aspect ratio, attempt to ensure the image covers both provided dimensions by cropping/clipping to fit.
     * - `contain`: Preserving aspect ratio, contain within both provided dimensions using "letterboxing" where necessary.
     * - `fill`: Ignore the aspect ratio of the input and stretch to both provided dimensions.
     * - `inside`: Preserving aspect ratio, resize the image to be as large as possible while ensuring its dimensions are less than or equal to both those specified.
     * - `outside`: Preserving aspect ratio, resize the image to be as small as possible while ensuring its dimensions are greater than or equal to both those specified.
     *
     * Some of these values are based on the [object-fit](https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit) CSS property.
     *
     * <img alt="Examples of various values for the fit property when resizing" width="100%" style="aspect-ratio: 998/243" src="/api-resize-fit.svg">
     *
     * When using a **fit** of `cover` or `contain`, the default **position** is `centre`. Other options are:
     * - `sharp.position`: `top`, `right top`, `right`, `right bottom`, `bottom`, `left bottom`, `left`, `left top`.
     * - `sharp.gravity`: `north`, `northeast`, `east`, `southeast`, `south`, `southwest`, `west`, `northwest`, `center` or `centre`.
     * - `sharp.strategy`: `cover` only, dynamically crop using either the `entropy` or `attention` strategy.
     *
     * Some of these values are based on the [object-position](https://developer.mozilla.org/en-US/docs/Web/CSS/object-position) CSS property.
     *
     * The strategy-based approach initially resizes so one dimension is at its target length
     * then repeatedly ranks edge regions, discarding the edge with the lowest score based on the selected strategy.
     * - `entropy`: focus on the region with the highest [Shannon entropy](https://en.wikipedia.org/wiki/Entropy_%28information_theory%29).
     * - `attention`: focus on the region with the highest luminance frequency, colour saturation and presence of skin tones.
     *
     * Possible downsizing kernels are:
     * - `nearest`: Use [nearest neighbour interpolation](http://en.wikipedia.org/wiki/Nearest-neighbor_interpolation).
     * - `linear`: Use a [triangle filter](https://en.wikipedia.org/wiki/Triangular_function).
     * - `cubic`: Use a [Catmull-Rom spline](https://en.wikipedia.org/wiki/Centripetal_Catmull%E2%80%93Rom_spline).
     * - `mitchell`: Use a [Mitchell-Netravali spline](https://www.cs.utexas.edu/~fussell/courses/cs384g-fall2013/lectures/mitchell/Mitchell.pdf).
     * - `lanczos2`: Use a [Lanczos kernel](https://en.wikipedia.org/wiki/Lanczos_resampling#Lanczos_kernel) with `a=2`.
     * - `lanczos3`: Use a Lanczos kernel with `a=3` (the default).
     *
     * When upsampling, these kernels map to `nearest`, `linear` and `cubic` interpolators.
     * Downsampling kernels without a matching upsampling interpolator map to `cubic`.
     *
     * Only one resize can occur per pipeline.
     * Previous calls to `resize` in the same pipeline will be ignored.
     *
     * @example
     * sharp(input)
     *   .resize({ width: 100 })
     *   .toBuffer()
     *   .then(data => {
     *     // 100 pixels wide, auto-scaled height
     *   });
     *
     * @example
     * sharp(input)
     *   .resize({ height: 100 })
     *   .toBuffer()
     *   .then(data => {
     *     // 100 pixels high, auto-scaled width
     *   });
     *
     * @example
     * sharp(input)
     *   .resize(200, 300, {
     *     kernel: sharp.kernel.nearest,
     *     fit: "contain",
     *     position: "right top",
     *     background: { r: 255, g: 255, b: 255, alpha: 0.5 }
     *   })
     *   .toFile("output.png")
     *   .then(() => {
     *     // output.png is a 200 pixels wide and 300 pixels high image
     *     // containing a nearest-neighbour scaled version
     *     // contained within the north-east corner of a semi-transparent white canvas
     *   });
     *
     * @example
     * const transformer = sharp()
     *   .resize({
     *     width: 200,
     *     height: 200,
     *     fit: sharp.fit.cover,
     *     position: sharp.strategy.entropy
     *   });
     * // Read image data from readableStream
     * // Write 200px square auto-cropped image data to writableStream
     * readableStream
     *   .pipe(transformer)
     *   .pipe(writableStream);
     *
     * @example
     * sharp(input)
     *   .resize(200, 200, {
     *     fit: sharp.fit.inside,
     *     withoutEnlargement: true
     *   })
     *   .toFormat("jpeg")
     *   .toBuffer()
     *   .then(function(outputBuffer) {
     *     // outputBuffer contains JPEG image data
     *     // no wider and no higher than 200 pixels
     *     // and no larger than the input image
     *   });
     *
     * @example
     * sharp(input)
     *   .resize(200, 200, {
     *     fit: sharp.fit.outside,
     *     withoutReduction: true
     *   })
     *   .toFormat("jpeg")
     *   .toBuffer()
     *   .then(function(outputBuffer) {
     *     // outputBuffer contains JPEG image data
     *     // of at least 200 pixels wide and 200 pixels high while maintaining aspect ratio
     *     // and no smaller than the input image
     *   });
     *
     * @example
     * const scaleByHalf = await sharp(input)
     *   .metadata()
     *   .then(({ width }) => sharp(input)
     *     .resize(Math.round(width * 0.5))
     *     .toBuffer()
     *   );
     *
     */
    pub fn resize(self, width: i32, height: i32) -> Result<Self, String> {
        let options = ResizeOptions {
            width: Some(width),
            height: Some(height),
            ..Default::default()
        };
        self.resize_(options)
    }

    pub fn resize_with_opts(self, options: ResizeOptions) -> Result<Self, String> {
        self.resize_(options)
    }

    fn is_resize_expected(&self) -> bool {
        self.options.width != -1 || self.options.height != -1
    }

    fn is_rotation_expected(&self) -> bool {
        (self.options.angle % 360) != 0 || self.options.input.auto_orient || self.options.rotation_angle != 0.0
    }

    fn resize_(mut self, options: ResizeOptions) -> Result<Self, String> {
        if self.is_resize_expected() {
            println!("ignoring previous resize options");
        }
        if self.options.width_post != -1 {
            println!("operation order will be: extract, resize, extract");
        }
        if options.width.is_none() {
            self.options.width = -1;
        }

        if options.height.is_none() {
            self.options.height = -1;
        }

        // Width
        if let Some(width) = options.width {
            if width > 0 {
                self.options.width = width;
            } else {
                return Err(InvalidParameterError!("width", "positive integer", width));
            }
        }

        // Height
        if let Some(height) = options.height {
            if height > 0 {
                self.options.height = height;
            } else {
                return Err(InvalidParameterError!("height", "positive integer", height));
            }
        }

        // Fit
        if let Some(fit) = options.fit {
            self.options.canvas = match fit {
                Fit::Contain => Canvas::Embed,
                Fit::Cover => Canvas::Crop,
                Fit::Fill => Canvas::IgnoreAspect,
                Fit::Inside => Canvas::Max,
                Fit::Outside => Canvas::Min,
            };
        }

        // Position
        if let Some(position) = options.position {
            let position = position as i32;
            if in_range(position as _, 0.0, 8.0) || in_range(position as _, 16.0, 17.0) {
                self.options.position = position;
            } else {
                return Err(InvalidParameterError!("position", "valid position/gravity/strategy", position));
            }
        }

        // Background
        if let Some(background) = options.background {
            self.options.resize_background = background.rgba;
        }

        // Kernel
        if let Some(kernel) = options.kernel {
            self.options.kernel = kernel
        }

        // Without enlargement
        if let Some(without_enlargement) = options.without_enlargement {
            self.options.without_enlargement = without_enlargement
        }
        // Without reduction
        if let Some(without_reduction) = options.without_reduction {
            self.options.without_reduction = without_reduction;
        }
        // Shrink on load
        if let Some(fast_shrink_on_load) = options.fast_shrink_on_load {
            self.options.fast_shrink_on_load = fast_shrink_on_load;
        }

        if self.is_rotation_expected() && self.is_resize_expected() {
            self.options.rotate_before_pre_extract = true;
        }
        Ok(self)
    }

    /**
     * Extend / pad / extrude one or more edges of the image with either
     * the provided background colour or pixels derived from the image.
     * self operation will always occur after resizing and extraction, if any.
     *
     * @example
     * // Resize to 140 pixels wide, then add 10 transparent pixels
     * // to the top, left and right edges and 20 to the bottom edge
     * sharp(input)
     *   .resize(140)
     *   .extend({
     *     top: 10,
     *     bottom: 20,
     *     left: 10,
     *     right: 10,
     *     background: { r: 0, g: 0, b: 0, alpha: 0 }
     *   })
     *   ...
     *
     * @example
     * // Add a row of 10 red pixels to the bottom
     * sharp(input)
     *   .extend({
     *     bottom: 10,
     *     background: "red"
     *   })
     *   ...
     *
     * @example
     * // Extrude image by 8 pixels to the right, mirroring existing right hand edge
     * sharp(input)
     *   .extend({
     *     right: 8,
     *     background: "mirror"
     *   })
     *   ...
     *
     */
    pub fn extend(mut self, extend: ExtendOptions) -> Result<Self, String> {
        if let Some(top) = extend.top {
            if top >= 0 {
                self.options.extend_top = top;
            } else {
                return Err(InvalidParameterError!("top", "positive integer", top));
            }
        }
        if let Some(bottom) = extend.bottom {
            if bottom >= 0 {
                self.options.extend_bottom = bottom;
            } else {
                return Err(InvalidParameterError!("bottom", "positive integer", bottom));
            }
        }
        if let Some(left) = extend.left {
            if left >= 0 {
                self.options.extend_left = left;
            } else {
                return Err(InvalidParameterError!("left", "positive integer", left));
            }
        }
        if let Some(right) = extend.right {
            if right >= 0 {
                self.options.extend_right = right;
            } else {
                return Err(InvalidParameterError!("right", "positive integer", right));
            }
        }
        if let Some(background) = extend.background {
            self.options.extend_background = background.rgba;
        }
        if let Some(extend_with) = extend.extend_with {
            self.options.extend_with = extend_with;
        }

        Ok(self)
    }

    /**
     * Extract/crop a region of the image.
     *
     * - Use `extract` before `resize` for pre-resize extraction.
     * - Use `extract` after `resize` for post-resize extraction.
     * - Use `extract` twice and `resize` once for extract-then-resize-then-extract in a fixed operation order.
     *
     * @example
     * sharp(input)
     *   .extract({ left: left, top: top, width: width, height: height })
     *   .toFile(output, function(err) {
     *     // Extract a region of the input image, saving in the same format.
     *   });
     * @example
     * sharp(input)
     *   .extract({ left: leftOffsetPre, top: topOffsetPre, width: widthPre, height: heightPre })
     *   .resize(width, height)
     *   .extract({ left: leftOffsetPost, top: topOffsetPost, width: widthPost, height: heightPost })
     *   .toFile(output, function(err) {
     *     // Extract a region, resize, then extract from the resized image
     *   });
     *
     */
    pub fn extract(mut self, region: Region) -> Result<Self, String> {
        let is_post = self.is_resize_expected() || self.options.width_pre != -1;

        if (is_post && self.options.width_post != -1) || (!is_post && self.options.width_pre != -1) {
            println!("ignoring previous extract options");
        }

        if is_post {
            self.options.left_offset_post = region.left as _;
            self.options.top_offset_post = region.top as _;
            self.options.width_post = region.width as _;
            self.options.height_post = region.height as _;
        } else {
            self.options.left_offset_pre = region.left as _;
            self.options.top_offset_pre = region.top as _;
            self.options.width_pre = region.width as _;
            self.options.height_pre = region.height as _;
        }

        // Ensure existing rotation occurs before pre-resize extraction
        if self.is_rotation_expected() && !self.is_resize_expected() && (self.options.width_pre == -1 || self.options.width_post == -1) {
            self.options.rotate_before_pre_extract = true;
        }
        Ok(self)
    }

    /**
     * Trim pixels from all edges that contain values similar to the given background colour, which defaults to that of the top-left pixel.
     *
     * Images with an alpha channel will use the combined bounding box of alpha and non-alpha channels.
     *
     * If the result of self operation would trim an image to nothing then no change is made.
     *
     * The `info` response Object will contain `trimOffsetLeft` and `trimOffsetTop` properties.
     *
     * @example
     * // Trim pixels with a colour similar to that of the top-left pixel.
     * await sharp(input)
     *   .trim()
     *   .toFile(output);
     *
     * @example
     * // Trim pixels with the exact same colour as that of the top-left pixel.
     * await sharp(input)
     *   .trim({
     *     threshold: 0
     *   })
     *   .toFile(output);
     *
     * @example
     * // Assume input is line art and trim only pixels with a similar colour to red.
     * const output = await sharp(input)
     *   .trim({
     *     background: "#FF0000",
     *     lineArt: true
     *   })
     *   .toBuffer();
     *
     * @example
     * // Trim all "yellow-ish" pixels, being more lenient with the higher threshold.
     * const output = await sharp(input)
     *   .trim({
     *     background: "yellow",
     *     threshold: 42,
     *   })
     *   .toBuffer();
     *
     * @param {Object} [options]
     * @param {string|Object} [options.background="top-left pixel"] - Background colour, parsed by the [color](https://www.npmjs.org/package/color) module, defaults to that of the top-left pixel.
     * @param {number} [options.threshold=10] - Allowed difference from the above colour, a positive number.
     * @param {boolean} [options.lineArt=false] - Does the input more closely resemble line art (e.g. vector) rather than being photographic?
     * @returns {Sharp}
     * @throws {Error} Invalid parameters
     */
    pub fn trim(mut self, options: Option<TrimOptions>) -> Result<Self, String> {
        self.options.trim_threshold = 10.0;
        if let Some(options) = options {
            if let Some(background) = options.background {
                self.options.trim_background = background.rgba;
            }
            if let Some(threshold) = options.threshold {
                if threshold >= 0.0 {
                    self.options.trim_threshold = threshold;
                } else {
                    return Err(InvalidParameterError!("threshold", "positive number", threshold));
                }
            }
            if let Some(line_art) = options.line_art {
                self.options.trim_line_art = line_art;
            }
        }
        if self.is_rotation_expected() {
            self.options.rotate_before_pre_extract = true;
        }

        Ok(self)
    }
    /**
     * Perform an affine transform on an image. self operation will always occur after resizing, extraction and rotation, if any.
     *
     * You must provide an array of length 4 or a 2x2 affine transformation matrix.
     * By default, new pixels are filled with a black background. You can provide a background colour with the `background` option.
     * A particular interpolator may also be specified. Set the `interpolator` option to an attribute of the `sharp.interpolators` Object e.g. `sharp.interpolators.nohalo`.
     *
     * In the case of a 2x2 matrix, the transform is:
     * - X = `matrix[0, 0]` \* (x + `idx`) + `matrix[0, 1]` \* (y + `idy`) + `odx`
     * - Y = `matrix[1, 0]` \* (x + `idx`) + `matrix[1, 1]` \* (y + `idy`) + `ody`
     *
     * where:
     * - x and y are the coordinates in input image.
     * - X and Y are the coordinates in output image.
     * - (0,0) is the upper left corner.
     *
     * @since 0.27.0
     *
     * @example
     * let pipeline = sharp()
     *   .affine([[1, 0.3], [0.1, 0.7]], {
     *      background: "white",
     *      interpolator: sharp.interpolators.nohalo
     *   })
     *   .toBuffer((err, outputBuffer, info) => {
     *      // outputBuffer contains the transformed image
     *      // info.width and info.height contain the new dimensions
     *   });
     *
     * inputStream
     *   .pipe(pipeline);
     *
     */
    pub fn affine(mut self, matrix: Vec<Vec<f64>>, options: Option<AffineOptions>) -> Result<Self, String> {
        let flat_matrix: Vec<f64> = matrix.into_iter().flatten().collect();
        if flat_matrix.len() == 4 {
            self.options.affine_matrix = flat_matrix;
        } else {
            return Err(InvalidParameterError!("matrix", "1x4 or 2x2 array", matrix));
        }

        if let Some(options) = options {
            if let Some(background) = options.background {
                self.options.affine_background = background.rgba;
            }
            if let Some(idx) = options.idx {
                self.options.affine_idx = idx;
            }
            if let Some(idy) = options.idy {
                self.options.affine_idy = idy;
            }
            if let Some(odx) = options.odx {
                self.options.affine_odx = odx;
            }
            if let Some(ody) = options.ody {
                self.options.affine_ody = ody;
            }
            if let Some(interpolator) = options.interpolator {
                self.options.affine_interpolator = interpolator.to_string();
            }
        }

        Ok(self)
    }

    /**
     * Sharpen the image.
     *
     * When used without parameters, performs a fast, mild sharpen of the output image.
     *
     * When a `sigma` is provided, performs a slower, more accurate sharpen of the L channel in the LAB colour space.
     * Fine-grained control over the level of sharpening in "flat" (m1) and "jagged" (m2) areas is available.
     *
     * See {@link https://www.libvips.org/API/current/libvips-convolution.html#vips-sharpen|libvips sharpen} operation.
     *
     * @example
     * let data = await sharp(input).sharpen().toBuffer();
     *
     * @example
     * let data = await sharp(input).sharpen({ sigma: 2 }).toBuffer();
     *
     * @example
     * let data = await sharp(input)
     *   .sharpen({
     *     sigma: 2,
     *     m1: 0,
     *     m2: 3,
     *     x1: 3,
     *     y2: 15,
     *     y3: 15,
     *   })
     *   .toBuffer();
     *
     */
    pub fn sharpen(mut self, options: Option<SharpenOptions>) -> Result<Self, String> {
        if let Some(options) = options {
            if !in_range(options.sigma, 0.000001, 10.0) {
                return Err(InvalidParameterError!("options.sigma", "number between 0.000001 and 10", options.sigma));
            }
            self.options.sharpen_sigma = options.sigma;

            if let Some(m1) = options.m1 {
                if !in_range(m1, 0.0, 1000000.0) {
                    return Err(InvalidParameterError!("options.m1", "number between 0 and 1000000", options.m1));
                }
                self.options.sharpen_m1 = m1;
            }
            if let Some(m2) = options.m2 {
                if !in_range(m2, 0.0, 1000000.0) {
                    return Err(InvalidParameterError!("options.m12", "number between 0 and 1000000", options.m2));
                }
                self.options.sharpen_m2 = m2;
            }
            if let Some(x1) = options.x1 {
                if !in_range(x1, 0.0, 1000000.0) {
                    return Err(InvalidParameterError!("options.x1", "number between 0 and 1000000", options.x1));
                }
                self.options.sharpen_x1 = x1;
            }
            if let Some(y2) = options.y2 {
                if !in_range(y2, 0.0, 1000000.0) {
                    return Err(InvalidParameterError!("options.y2", "number between 0 and 1000000", options.y2));
                }
                self.options.sharpen_y2 = y2;
            }
            if let Some(y3) = options.y3 {
                if !in_range(y3, 0.0, 1000000.0) {
                    return Err(InvalidParameterError!("options.y3", "number between 0 and 1000000", options.y3));
                }
                self.options.sharpen_y3 = y3;
            }
        } else {
            // No arguments: default to mild sharpen
            self.options.sharpen_sigma = -1.0;
        }
        Ok(self)
    }

    /**
     * Apply median filter.
     * When used without parameters the default window is 3x3.
     *
     * @example
     * let output = await sharp(input).median().toBuffer();
     *
     * @example
     * let output = await sharp(input).median(5).toBuffer();
     */
    pub fn median(mut self, size: Option<i32>) -> Result<Self, String> {
        if let Some(size) = size {
            if !in_range(size as _, 1.0, 1000.0) {
                return Err(InvalidParameterError!("size", "integer between 1 and 1000", size));
            }
            self.options.median_size = size;
        } else {
            // No arguments: default to 3x3
            self.options.median_size = 3;
        }
        Ok(self)
    }

    /**
     * Blur the image.
     *
     * When used without parameters, performs a fast 3x3 box blur (equivalent to a box linear filter).
     *
     * When a `sigma` is provided, performs a slower, more accurate Gaussian blur.
     *
     * @example
     * let boxBlurred = await sharp(input)
     *   .blur()
     *   .toBuffer();
     *
     * @example
     * let gaussianBlurred = await sharp(input)
     *   .blur(5)
     *   .toBuffer();
     */
    pub fn blur(mut self, options: Option<BlurOptions>) -> Result<Self, String> {
        if let Some(options) = options {
            if !in_range(options.sigma, 0.3, 1000.0) {
                return Err(InvalidParameterError!("options.sigma", "number between 0.3 and 1000", sigma));
            }
            self.options.blur_sigma = options.sigma;
            if let Some(precision) = options.precision {
                self.options.precision = precision;
            }
            if let Some(min_amplitude) = options.min_amplitude {
                if !in_range(min_amplitude, 0.001, 1.0) {
                    return Err(InvalidParameterError!("min_amplitude", "number between 0.001 and 1", min_amplitude));
                }
                self.options.min_ampl = min_amplitude;
            }
        } else {
            // No arguments: default to mild blur
            self.options.blur_sigma = -1.0;
        }

        Ok(self)
    }

    /**
     * Expand foreground objects using the dilate morphological operator.
     *
     * @example
     * let output = await sharp(input)
     *   .dilate()
     *   .toBuffer();
     *
     */
    pub fn dilate(mut self, width: Option<i32>) -> Result<Self, String> {
        if let Some(width) = width {
            if width < 0 {
                return Err(InvalidParameterError!("dilate", "positive integer", width));
            }
            self.options.dilate_width = width;
        } else {
            self.options.dilate_width = 1;
        }
        Ok(self)
    }

    /**
     * Shrink foreground objects using the erode morphological operator.
     *
     * @example
     * let output = await sharp(input)
     *   .erode()
     *   .toBuffer();
     *
     */
    pub fn erode(mut self, width: Option<i32>) -> Result<Self, String> {
        if let Some(width) = width {
            if width < 0 {
                return Err(InvalidParameterError!("erode", "positive integer", width));
            }
            self.options.erode_width = width;
        } else {
            self.options.erode_width = 1;
        }
        Ok(self)
    }

    /**
     * Merge alpha transparency channel, if any, with a background, then remove the alpha channel.
     *
     * See also {@link /api-channel#removealpha|removeAlpha}.
     *
     * @example
     * await sharp(rgbaInput)
     *   .flatten({ background: "#F0A703" })
     *   .toBuffer();
     *
     */
    pub fn flatten(mut self, options: Option<FlattenOptions>) -> Result<Self, String> {
        self.options.flatten = true;
        if let Some(options) = options {
            if let Some(background) = options.background {
                self.options.flatten_background = background.rgba;
            }
        }
        Ok(self)
    }

    /**
     * Ensure the image has an alpha channel
     * with all white pixel values made fully transparent.
     *
     * Existing alpha channel values for non-white pixels remain unchanged.
     *
     * self feature is experimental and the API may change.
     *
     * @since 0.32.1
     *
     * @example
     * await sharp(rgbInput)
     *   .unflatten()
     *   .toBuffer();
     *
     * @example
     * await sharp(rgbInput)
     *   .threshold(128, { grayscale: false }) // converter bright pixels to white
     *   .unflatten()
     *   .toBuffer();
     */
    pub fn unflatten(mut self) -> Result<Self, String> {
        self.options.unflatten = true;
        Ok(self)
    }

    /**
     * Apply a gamma correction by reducing the encoding (darken) pre-resize at a factor of `1/gamma`
     * then increasing the encoding (brighten) post-resize at a factor of `gamma`.
     * self can improve the perceived brightness of a resized image in non-linear colour spaces.
     * JPEG and WebP input images will not take advantage of the shrink-on-load performance optimisation
     * when applying a gamma correction.
     *
     * Supply a second argument to use a different output gamma value, otherwise the first value is used in both cases.
     *
     */
    pub fn gamma(mut self, gamma: Option<f64>, gamma_out: Option<f64>) -> Result<Self, String> {
        if let Some(gamma) = gamma {
            if !in_range(gamma, 1.0, 3.0) {
                return Err(InvalidParameterError!("gamma", "number between 1.0 and 3.0", gamma));
            }
            self.options.gamma = gamma;
        } else {
            // Default gamma correction of 2.2 (sRGB)
            self.options.gamma = 2.2;
        }

        if let Some(gamma_out) = gamma_out {
            if !in_range(gamma_out, 1.0, 3.0) {
                return Err(InvalidParameterError!("gamma_out", "number between 1.0 and 3.0", gamma_out));
            }
            self.options.gamma_out = gamma_out;
        } else {
            // Default gamma correction for output is same as input
            self.options.gamma_out = self.options.gamma;
        }

        Ok(self)
    }

    /**
     * Produce the "negative" of the image.
     *
     * @example
     * let output = await sharp(input)
     *   .negate()
     *   .toBuffer();
     *
     * @example
     * let output = await sharp(input)
     *   .negate({ alpha: false })
     *   .toBuffer();
     *
     */
    pub fn negate(mut self, options: Option<NegateOptions>) -> Result<Self, String> {
        self.options.negate = true;
        if let Some(options) = options {
            if let Some(alpha) = options.alpha {
                self.options.negate_alpha = alpha;
            }
        }
        Ok(self)
    }

    /**
     * Enhance output image contrast by stretching its luminance to cover a full dynamic range.
     *
     * Uses a histogram-based approach, taking a default range of 1% to 99% to reduce sensitivity to noise at the extremes.
     *
     * Luminance values below the `lower` percentile will be underexposed by clipping to zero.
     * Luminance values above the `upper` percentile will be overexposed by clipping to the max pixel value.
     *
     * @example
     * let output = await sharp(input)
     *   .normalise()
     *   .toBuffer();
     *
     * @example
     * let output = await sharp(input)
     *   .normalise({ lower: 0, upper: 100 })
     *   .toBuffer();
     *
     */
    pub fn normalise(mut self, options: Option<NormaliseOptions>) -> Result<Self, String> {
        if let Some(options) = options {
            if let Some(lower) = options.lower {
                if !in_range(lower as _, 0.0, 99.0) {
                    return Err(InvalidParameterError!("lower", "number between 0 and 99", lower));
                }
                self.options.normalise_lower = lower;
            }
            if let Some(upper) = options.upper {
                if !in_range(upper as _, 1.0, 100.0) {
                    return Err(InvalidParameterError!("upper", "number between 1 and 100", upper));
                }
                self.options.normalise_upper = upper;
            }
        }

        if self.options.normalise_lower >= self.options.normalise_upper {
            return Err(InvalidParameterError!("range", "lower to be less than upper", format!("{:?} >= {:?}", self.options.normaliseLower, self.options.normaliseUpper)));
        }
        self.options.normalise = true;
        Ok(self)
    }

    /**
     * Perform contrast limiting adaptive histogram equalization
     * {@link https://en.wikipedia.org/wiki/Adaptive_histogram_equalization#Contrast_Limited_AHE|CLAHE}.
     *
     * self will, in general, enhance the clarity of the image by bringing out darker details.
     *
     * @since 0.28.3
     *
     * @example
     * let output = await sharp(input)
     *   .clahe({
     *     width: 3,
     *     height: 3,
     *   })
     *   .toBuffer();
     *
     */
    pub fn clahe(mut self, options: Option<ClaheOptions>) -> Result<Self, String> {
        if let Some(options) = options {
            if options.width > 0 {
                self.options.clahe_width = options.width;
            } else {
                return Err(InvalidParameterError!("width", "integer greater than zero", options.width));
            }
            if options.height > 0 {
                self.options.clahe_height = options.height;
            } else {
                return Err(InvalidParameterError!("height", "integer greater than zero", options.height));
            }
            if let Some(max_slope) = options.max_slope {
                if !in_range(max_slope as _, 0.0, 100.0) {
                    return Err(InvalidParameterError!("max_slope", "integer between 0 and 100", max_slope));
                }
                self.options.clahe_max_slope = max_slope;
            }
        }

        Ok(self)
    }

    /**
     * Convolve the image with the specified kernel.
     *
     * @example
     * sharp(input)
     *   .convolve({
     *     width: 3,
     *     height: 3,
     *     kernel: [-1, 0, 1, -2, 0, 2, -1, 0, 1]
     *   })
     *   .raw()
     *   .toBuffer(pub fn(err, data, info) {
     *     // data contains the raw pixel data representing the convolution
     *     // of the input image with the horizontal Sobel operator
     *   });
     *
     */
    pub fn convolve(mut self, kernel: KernelOptions) -> Result<Self, String> {
        if !in_range(kernel.width as _, 3.0, 1001.0) || !in_range(kernel.height as _, 3.0, 1001.0) || kernel.height * kernel.width != kernel.kernel.len() as i32 {
            return Err("Invalid convolution kernel".to_string());
        }
        // Default scale is sum of kernel values
        let scale = if let Some(scale) = kernel.scale {
            // Clip scale to a minimum value of 1
            if scale < 1.0 {
                1.0
            } else {
                scale
            }
        } else {
            kernel.kernel.clone().iter().sum()
        };

        self.options.conv_kernel_scale = scale;

        if let Some(offset) = kernel.offset {
            self.options.conv_kernel_offset = offset;
        } else {
            self.options.conv_kernel_offset = 0.0;
        }

        self.options.conv_kernel_height = kernel.height;
        self.options.conv_kernel_width = kernel.width;
        self.options.conv_kernel = kernel.kernel;
        Ok(self)
    }

    /**
     * Any pixel value greater than or equal to the threshold value will be set to 255, otherwise it will be set to 0.
     */
    pub fn threshold(mut self, threshold: Option<i32>, options: Option<ThresholdOptions>) -> Result<Self, String> {
        if let Some(threshold) = threshold {
            if !in_range(threshold as _, 0.0, 255.0) {
                return Err(InvalidParameterError!("threshold", "integer between 0 and 255", threshold));
            }
            self.options.threshold = threshold;
        } else {
            self.options.threshold = 128;
        }

        if let Some(options) = options {
            if let Some(grayscale) = options.grayscale {
                self.options.threshold_grayscale = grayscale;
            }
        } else {
            self.options.threshold_grayscale = false;
        }

        Ok(self)
    }

    /**
     * Perform a bitwise boolean operation with operand image.
     *
     * self operation creates an output image where each pixel is the result of
     * the selected bitwise boolean `operation` between the corresponding pixels of the input images.
     *
     */
    pub fn boolean(mut self, operand: Input, operator: OperationBoolean, options: Option<BooleanOptions>) -> Result<Self, String> {
        let mut sharp_options = SharpOptions::default();
        if let Some(options) = options {
            sharp_options.raw = Some(CreateRaw {
                width: options.raw.width,
                height: options.raw.height,
                channels: options.raw.channels,
                premultiplied: false,
            });
        }
        self.options.boolean_descriptor = Some(create_input_descriptor(operand, Some(sharp_options))?);

        self.options.boolean_op = operator;

        Ok(self)
    }

    /**
     * Apply the linear formula `a` * input + `b` to the image to adjust image levels.
     *
     * When a single number is provided, it will be used for all image channels.
     * When an array of numbers is provided, the array length must match the number of channels.
     *
     * @example
     * await sharp(input)
     *   .linear(0.5, 2)
     *   .toBuffer();
     *
     * @example
     * await sharp(rgbInput)
     *   .linear(
     *     [0.25, 0.5, 0.75],
     *     [150, 100, 50]
     *   )
     *   .toBuffer();
     *
     */
    pub fn linear(mut self, a: Option<Vec<f64>>, b: Option<Vec<f64>>) -> Result<Self, String> {
        let (a, b) = if a.is_none() && b.is_some() {
            (Some(vec![1.0]), b)
        } else if a.is_some() && b.is_none() {
            (a, Some(vec![0.0]))
        } else {
            (None, None)
        };

        if let Some(a) = a {
            self.options.linear_a = a;
        } else {
            self.options.linear_a = Vec::new();
        }

        if let Some(b) = b {
            self.options.linear_b = b;
        } else {
            self.options.linear_b = Vec::new();
        }

        Ok(self)
    }

    /**
     * Recombine the image with the specified matrix.
     *
     * @since 0.21.1
     *
     * @example
     * sharp(input)
     *   .recomb([
     *    [0.3588, 0.7044, 0.1368],
     *    [0.2990, 0.5870, 0.1140],
     *    [0.2392, 0.4696, 0.0912],
     *   ])
     *   .raw()
     *   .toBuffer(pub fn(err, data, info) {
     *     // data contains the raw pixel data after applying the matrix
     *     // With self example input, a sepia filter has been applied
     *   });
     *
     */
    pub fn recomb(mut self, input_matrix: Vec<Vec<f64>>) -> Result<Self, String> {
        if input_matrix.len() != 3 && input_matrix.len() != 4 {
            return Err(InvalidParameterError!("input_matrix", "3x3 or 4x4 array", input_matrix));
        }
        let recomb_matrix: Vec<f64> = input_matrix.into_iter().flatten().collect();
        if recomb_matrix.len() != 9 && recomb_matrix.len() != 16 {
            return Err(InvalidParameterError!("recomb_matrix", "cardinality of 9 or 16", recomb_matrix.len()));
        }
        self.options.recomb_matrix = recomb_matrix;
        Ok(self)
    }

    /**
     * Transforms the image using brightness, saturation, hue rotation, and lightness.
     * Brightness and lightness both operate on luminance, with the difference being that
     * brightness is multiplicative whereas lightness is additive.
     *
     * @since 0.22.1
     *
     * @example
     * // increase brightness by a factor of 2
     * let output = await sharp(input)
     *   .modulate({
     *     brightness: 2
     *   })
     *   .toBuffer();
     *
     * @example
     * // hue-rotate by 180 degrees
     * let output = await sharp(input)
     *   .modulate({
     *     hue: 180
     *   })
     *   .toBuffer();
     *
     * @example
     * // increase lightness by +50
     * let output = await sharp(input)
     *   .modulate({
     *     lightness: 50
     *   })
     *   .toBuffer();
     *
     * @example
     * // decrease brightness and saturation while also hue-rotating by 90 degrees
     * let output = await sharp(input)
     *   .modulate({
     *     brightness: 0.5,
     *     saturation: 0.5,
     *     hue: 90,
     *   })
     *   .toBuffer();
     *
     */
    pub fn modulate(mut self, options: Option<ModulateOptions>) -> Result<Self, String> {
        if let Some(options) = options {
            if let Some(brightness) = options.brightness {
                if brightness < 0.0 {
                    return Err(InvalidParameterError!("brightness", "number above zero", brightness));
                }
                self.options.brightness = brightness;
            }
            if let Some(saturation) = options.saturation {
                if saturation < 0.0 {
                    return Err(InvalidParameterError!("saturation", "number above zero", saturation));
                }
                self.options.saturation = saturation;
            }
            if let Some(hue) = options.hue {
                self.options.hue = hue % 360;
            }
            if let Some(lightness) = options.lightness {
                self.options.lightness = lightness;
            }
        }
        Ok(self)
    }
}

fn in_range(value: f64, min: f64, max: f64) -> bool {
    value >= min && value <= max
}
