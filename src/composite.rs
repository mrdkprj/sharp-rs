use crate::{
    input::{create_input_descriptor, Input, SharpOptions},
    pipeline::Composite,
    InvalidParameterError, Sharp,
};
use libvips::operations::BlendMode;

pub struct OverlayOptions {
    input: Input,
    options: Option<SharpOptions>,
    blend: Option<BlendMode>,
    gravity: Option<i32>,
    top: Option<i32>,
    left: Option<i32>,
    tile: Option<bool>,
    has_offset: Option<bool>,
    premultiplied: Option<bool>,
}

impl Sharp {
    pub fn composite(mut self, images: &[OverlayOptions]) -> Result<Self, String> {
        if images.is_empty() {
            return Err(InvalidParameterError!("images to composite", "array", images));
        }

        let mut composite = Vec::new();
        for image in images {
            composite.push(Composite {
                input: create_input_descriptor(image.input.clone(), image.options.clone())?,
                mode: image.blend.unwrap_or(BlendMode::Over),
                tile: image.tile.unwrap_or(false),
                left: image.left.unwrap_or(0),
                top: image.top.unwrap_or(0),
                has_offset: image.has_offset.unwrap_or_default(),
                gravity: image.gravity.unwrap_or_default(),
                premultiplied: image.premultiplied.unwrap_or_default(),
            });
        }

        self.options.composite = composite;

        Ok(self)
    }
}
