use crate::{
    input::{create_input_descriptor, Create, CreateRaw, CreateText, Input, SharpOptions},
    pipeline::Composite,
    resize::Gravity,
    InvalidParameterError, Sharp,
};
use libvips::ops::{BlendMode, FailOn};

#[derive(Debug, Clone)]
pub enum CompositeInput {
    Path(String),
    Buffer(Vec<u8>),
    Create(Create),
    Text(CreateText),
    Raw(CreateRaw),
    None(),
}

impl Default for CompositeInput {
    fn default() -> Self {
        CompositeInput::None()
    }
}

#[derive(Debug, Default)]
pub struct OverlayOptions {
    /** Buffer containing image data, String containing the path to an image file, or Create object  */
    pub input: CompositeInput,
    /** Image options **/
    pub options: Option<SharpOptions>,
    /** how to blend this image with the image below. (optional, default `'over'`) */
    pub blend: Option<BlendMode>,
    /** gravity at which to place the overlay. (optional, default 'centre') */
    pub gravity: Option<Gravity>,
    /** the pixel offset from the top edge. */
    pub top: Option<i32>,
    /** the pixel offset from the left edge. */
    pub left: Option<i32>,
    /** set to true to repeat the overlay image across the entire image with the given  gravity. (optional, default false) */
    pub tile: Option<bool>,
    /** Set to true to avoid premultipling the image below. Equivalent to the --premultiplied vips option. */
    pub premultiplied: Option<bool>,
    /** number representing the DPI for vector overlay image. (optional, default 72)*/
    pub density: Option<i32>,
    /** Set to true to read all frames/pages of an animated image. (optional, default false) */
    pub animated: Option<bool>,
    /** see sharp() constructor, (optional, default 'warning') */
    pub fail_on: Option<FailOn>,
    /** see sharp() constructor, (optional, default 268402689) */
    pub limit_input_pixels: Option<i32>,
    /** see sharp() constructor, (optional, default false) */
    pub auto_orient: Option<bool>,
}

impl Sharp {
    pub fn composite(mut self, images: &[OverlayOptions]) -> Result<Self, String> {
        if images.is_empty() {
            return Err(InvalidParameterError!("images to composite", "array", images));
        }

        let mut composites = Vec::new();
        for image in images {
            let mut options = image.options.clone().unwrap_or_default();
            let input = match image.input.clone() {
                CompositeInput::Raw(raw) => {
                    options.raw = Some(raw);
                    Input::None()
                }
                CompositeInput::Create(create) => {
                    options.create = Some(create);
                    Input::None()
                }
                CompositeInput::Text(text) => {
                    options.text = Some(text);
                    Input::None()
                }
                CompositeInput::Buffer(buffer) => Input::Buffer(buffer),
                CompositeInput::Path(path) => Input::Path(path),
                _ => Input::None(),
            };
            let compsite = Composite {
                input: create_input_descriptor(input, Some(options))?,
                mode: image.blend.unwrap_or(BlendMode::Over),
                tile: image.tile.unwrap_or(false),
                left: image.left.unwrap_or(0),
                top: image.top.unwrap_or(0),
                has_offset: image.top.is_some() && image.left.is_some(),
                gravity: image.gravity.clone().unwrap_or(Gravity::Centre) as _,
                premultiplied: image.premultiplied.unwrap_or_default(),
            };

            composites.push(compsite);
        }

        self.options.composite = composites;

        Ok(self)
    }
}
