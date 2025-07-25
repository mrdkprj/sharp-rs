use crate::{
    in_range,
    input::{create_input_descriptor, Input, SharpOptions},
    InvalidParameterError, Sharp,
};
use libvips::operations::OperationBoolean;
use std::path::Path;

impl Sharp {
    /**
     * Remove alpha channels, if any. This is a no-op if the image does not have an alpha channel.
     *
     * See also {@link /api-operation#flatten|flatten}.
     *
     * @example
     * sharp('rgba.png')
     *   .removeAlpha()
     *   .toFile('rgb.png', function(err, info) {
     *     // rgb.png is a 3 channel image without an alpha channel
     *   });
     *
     * @returns {Sharp}
     */
    pub fn remove_alpha(mut self) -> Self {
        self.options.remove_alpha = true;
        self
    }

    /**
     * Ensure the output image has an alpha transparency channel.
     * If missing, the added alpha channel will have the specified
     * transparency level, defaulting to fully-opaque (1).
     * This is a no-op if the image already has an alpha channel.
     *
     * @since 0.21.2
     *
     * @example
     * // rgba.png will be a 4 channel image with a fully-opaque alpha channel
     * await sharp('rgb.jpg')
     *   .ensureAlpha()
     *   .toFile('rgba.png')
     *
     * @example
     * // rgba is a 4 channel image with a fully-transparent alpha channel
     * const rgba = await sharp(rgb)
     *   .ensureAlpha(0)
     *   .toBuffer();
     *
     * @param {number} [alpha=1] - alpha transparency level (0=fully-transparent, 1=fully-opaque)
     * @returns {Sharp}
     * @throws {Error} Invalid alpha transparency level
     */
    pub fn ensure_alpha(mut self, alpha: u32) -> Result<Self, String> {
        if in_range(alpha as _, 0.0, 1.1) {
            self.options.ensure_alpha = alpha as _;
        } else {
            return Err(InvalidParameterError!("alpha", "number between 0 and 1", alpha));
        }
        Ok(self)
    }

    /**
     * Extract a single channel from a multi-channel image.
     *
     * @example
     * // green.jpg is a greyscale image containing the green channel of the input
     * await sharp(input)
     *   .extractChannel('green')
     *   .toFile('green.jpg');
     *
     * @example
     * // red1 is the red value of the first pixel, red2 the second pixel etc.
     * const [red1, red2, ...] = await sharp(input)
     *   .extractChannel(0)
     *   .raw()
     *   .toBuffer();
     *
     * @param {number|string} channel - zero-indexed channel/band number to extract, or `red`, `green`, `blue` or `alpha`.
     * @returns {Sharp}
     * @throws {Error} Invalid channel
     */
    pub fn extract_channel(mut self, channel: u32) -> Result<Self, String> {
        if in_range(channel as _, 0.0, 4.0) {
            self.options.extract_channel = channel as _;
        } else {
            return Err(InvalidParameterError!("channel", "integer or one of: red, green, blue, alpha", channel));
        }
        Ok(self)
    }

    /**
     * Join one or more channels to the image.
     * The meaning of the added channels depends on the output colourspace, set with `toColourspace()`.
     * By default the output image will be web-friendly sRGB, with additional channels interpreted as alpha channels.
     * Channel ordering follows vips convention:
     * - sRGB: 0: Red, 1: Green, 2: Blue, 3: Alpha.
     * - CMYK: 0: Magenta, 1: Cyan, 2: Yellow, 3: Black, 4: Alpha.
     *
     * Buffers may be any of the image formats supported by sharp.
     * For raw pixel input, the `options` object should contain a `raw` attribute, which follows the format of the attribute of the same name in the `sharp()` constructor.
     *
     * @param {Array<string|Buffer>|string|Buffer} images - one or more images (file paths, Buffers).
     * @param {Object} options - image options, see `sharp()` constructor.
     * @returns {Sharp}
     * @throws {Error} Invalid parameters
     */
    pub fn join_channel<P: AsRef<Path>>(mut self, images: &[P], options: Option<SharpOptions>) -> Result<Self, String> {
        if images.is_empty() {
            return Ok(self);
        }

        if images.len() > 1 {
            for image in images {
                self.options.join_channel_in.push(create_input_descriptor(Input::Path(image.as_ref().to_string_lossy().to_string()), options.clone())?);
            }
        } else {
            self.options.join_channel_in.push(create_input_descriptor(Input::Path(images[0].as_ref().to_string_lossy().to_string()), options)?);
        }
        Ok(self)
    }

    pub fn join_channel_buffers(mut self, images: &[Vec<u8>], options: Option<SharpOptions>) -> Result<Self, String> {
        if images.is_empty() {
            return Ok(self);
        }

        if images.len() > 1 {
            for image in images {
                self.options.join_channel_in.push(create_input_descriptor(Input::Buffer(image.to_vec()), options.clone())?);
            }
        } else {
            self.options.join_channel_in.push(create_input_descriptor(Input::Buffer(images.first().unwrap().to_vec()), options)?);
        }
        Ok(self)
    }

    /**
     * Perform a bitwise boolean operation on all input image channels (bands) to produce a single channel output image.
     *
     * @example
     * sharp('3-channel-rgb-input.png')
     *   .bandbool(sharp.bool.and)
     *   .toFile('1-channel-output.png', function (err, info) {
     *     // The output will be a single channel image where each pixel `P = R & G & B`.
     *     // If `I(1,1) = [247, 170, 14] = [0b11110111, 0b10101010, 0b00001111]`
     *     // then `O(1,1) = 0b11110111 & 0b10101010 & 0b00001111 = 0b00000010 = 2`.
     *   });
     *
     * @param {string} boolOp - one of `and`, `or` or `eor` to perform that bitwise operation, like the C logic operators `&`, `|` and `^` respectively.
     * @returns {Sharp}
     * @throws {Error} Invalid parameters
     */
    pub fn bandbool(mut self, bool_op: OperationBoolean) -> Self {
        self.options.band_bool_op = bool_op;
        self
    }
}
