use crate::{common::Canvas, in_range, Colour, InvalidParameterError, Sharp};
use libvips::ops::{Extend, Kernel};
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone)]
pub enum Fit {
    Contain,
    Cover,
    Fill,
    Inside,
    Outside,
}

#[derive(Debug, Clone, Default)]
pub struct ResizeOptions {
    /** Alternative means of specifying width. If both are present self takes priority. */
    pub width: i32,
    /** Alternative means of specifying height. If both are present self takes priority. */
    pub height: i32,
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
pub enum Gravity {
    Centre = 0,
    North = 1,
    East = 2,
    South = 3,
    West = 4,
    Northeast = 5,
    Southeast = 6,
    Southwest = 7,
    Northwest = 8,
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

impl Sharp {
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
            width,
            height,
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
        (self.options.angle % 360) != 0
            || self.options.input.auto_orient
            || self.options.rotation_angle != 0.0
    }

    fn resize_(mut self, options: ResizeOptions) -> Result<Self, String> {
        if self.is_resize_expected() {
            println!("ignoring previous resize options");
        }
        if self.options.width_post != -1 {
            println!("operation order will be: extract, resize, extract");
        }

        // Width
        if options.width > 0 {
            self.options.width = options.width;
        } else {
            return Err(InvalidParameterError!("width", "positive integer", width));
        }

        // Height
        if options.height > 0 {
            self.options.height = options.height;
        } else {
            return Err(InvalidParameterError!("height", "positive integer", height));
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
                return Err(InvalidParameterError!(
                    "position",
                    "valid position/gravity/strategy",
                    position
                ));
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

        if (is_post && self.options.width_post != -1) || (!is_post && self.options.width_pre != -1)
        {
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
        if self.is_rotation_expected()
            && !self.is_resize_expected()
            && (self.options.width_pre == -1 || self.options.width_post == -1)
        {
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
}
