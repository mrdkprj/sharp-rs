use crate::{
    operation::{AffineOptions, BlurOptions, BooleanOptions, ClaheOptions, FlattenOptions, KernelOptions, ModulateOptions, NegateOptions, NormaliseOptions, SharpenOptions, ThresholdOptions},
    pipeline::{init_options, PipelineBaton},
};
use common::{rgba_from_hex, InputDescriptor};
use input::{create_input_descriptor, CreateRaw, Input, RotateOptions, SharpOptions};
pub use libvips::operations::{
    BandFormat, BlendMode, Extend, ForeignDzContainer, ForeignDzDepth, ForeignDzLayout, ForeignHeifCompression, ForeignTiffCompression, ForeignTiffPredictor, ForeignTiffResunit, ForeignWebpPreset,
    Interpretation, Kernel, OperationBoolean,
};
use libvips::Vips;
use std::path::Path;

pub mod channel;
pub mod colour;
mod common;
pub mod composite;
mod icon;
pub mod input;
pub mod metadata;
pub mod operation;
pub mod output;
mod pipeline;
pub mod resize;
mod stats;
mod util;

macro_rules! InvalidParameterError {
    ($name:expr, $expected:expr, $actual:expr) => {
        format!("Expected {:?} for {:?} but received {:?}", stringify!($expected), $name, stringify!($actual))
    };
}

pub(crate) use InvalidParameterError;

pub enum ImageKind {
    File(String),
    Buffer(Vec<u8>),
}

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

pub struct Sharp {
    options: PipelineBaton,
}

impl Sharp {
    pub fn new(options: SharpOptions) -> Result<Self, String> {
        Vips::init("sharp-rs", false).map_err(|e| e.to_string())?;
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
        Vips::init("sharp-rs", false).map_err(|e| e.to_string())?;
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

        Vips::init("sharp-rs", false).map_err(|e| e.to_string())?;
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
        Vips::init("sharp-rs", false).map_err(|e| e.to_string())?;
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

        Vips::init("sharp-rs", false).map_err(|e| e.to_string())?;
        let mut all_options = init_options();
        // Join images together
        let join: Result<Vec<InputDescriptor>, String> = buffers.iter().map(|buffer| create_input_descriptor(Input::Buffer(buffer.to_vec()), options.clone())).collect();
        all_options.join = join?;

        Ok(Self {
            options: all_options,
        })
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
