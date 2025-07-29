use crate::{Colour, Sharp};
pub use libvips::ops::Interpretation;

impl Sharp {
    /**
     * Tint the image using the provided colour.
     * An alpha channel may be present and will be unchanged by the operation.
     *
     * @example
     * const output = await sharp(input)
     *   .tint({ r: 255, g: 240, b: 16 })
     *   .toBuffer();
     *
     * @param {string|Object} tint - Parsed by the [color](https://www.npmjs.org/package/color) module.
     * @returns {Sharp}
     * @throws {Error} Invalid parameter
     */
    pub fn tint(mut self, tint: Colour) -> Self {
        self.options.tint = tint.rgba;
        self
    }

    /**
     * Convert to 8-bit greyscale; 256 shades of grey.
     * This is a linear operation. If the input image is in a non-linear colour space such as sRGB, use `gamma()` with `greyscale()` for the best results.
     * By default the output image will be web-friendly sRGB and contain three (identical) colour channels.
     * This may be overridden by other sharp operations such as `toColourspace('b-w')`,
     * which will produce an output image containing one colour channel.
     * An alpha channel may be present, and will be unchanged by the operation.
     *
     * @example
     * const output = await sharp(input).greyscale().toBuffer();
     *
     * @param {Boolean} [greyscale=true]
     * @returns {Sharp}
     */
    pub fn greyscale(mut self, greyscale: bool) -> Self {
        self.options.greyscale = greyscale;
        self
    }

    /**
     * Set the pipeline colourspace.
     *
     * The input image will be converted to the provided colourspace at the start of the pipeline.
     * All operations will use this colourspace before converting to the output colourspace,
     * as defined by {@link #tocolourspace|toColourspace}.
     *
     * @since 0.29.0
     *
     * @example
     * // Run pipeline in 16 bits per channel RGB while converting final result to 8 bits per channel sRGB.
     * await sharp(input)
     *  .pipelineColourspace('rgb16')
     *  .toColourspace('srgb')
     *  .toFile('16bpc-pipeline-to-8bpc-output.png')
     *
     * @param {string} [colourspace] - pipeline colourspace e.g. `rgb16`, `scrgb`, `lab`, `grey16` [...](https://github.com/libvips/libvips/blob/41cff4e9d0838498487a00623462204eb10ee5b8/libvips/iofuncs/enumtypes.c#L774)
     * @returns {Sharp}
     * @throws {Error} Invalid parameters
     */
    pub fn pipeline_colourspace(mut self, colourspace: Interpretation) -> Self {
        self.options.colourspace_pipeline = colourspace;
        self
    }

    /**
     * Set the output colourspace.
     * By default output image will be web-friendly sRGB, with additional channels interpreted as alpha channels.
     *
     * @example
     * // Output 16 bits per pixel RGB
     * await sharp(input)
     *  .toColourspace('rgb16')
     *  .toFile('16-bpp.png')
     *
     * @param {string} [colourspace] - output colourspace e.g. `srgb`, `rgb`, `cmyk`, `lab`, `b-w` [...](https://github.com/libvips/libvips/blob/3c0bfdf74ce1dc37a6429bed47fa76f16e2cd70a/libvips/iofuncs/enumtypes.c#L777-L794)
     * @returns {Sharp}
     * @throws {Error} Invalid parameters
     */
    pub fn to_colourspace(mut self, colourspace: Interpretation) -> Self {
        self.options.colourspace = colourspace;
        self
    }
}
