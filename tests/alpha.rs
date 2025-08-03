use sharp::{operation::FlattenOptions, output::JpegOptions, Colour, Sharp};
mod fixtures;

#[test]
pub fn alpha() {
    //Flatten to black
    Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .flatten(None)
        .unwrap()
        .resize(400, 300)
        .unwrap()
        .to_buffer()
        .unwrap();

    //Flatten to RGB orange
    Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .resize(400, 300)
        .unwrap()
        .flatten(Some(FlattenOptions {
            background: Some(Colour::new(255, 102, 0, 0.0)),
        }))
        .unwrap()
        .jpeg(Some(JpegOptions {
            chroma_subsampling: Some("4:4:4".to_string()),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Flatten to CSS/hex orange
    Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .resize(400, 300)
        .unwrap()
        .flatten(Some(FlattenOptions {
            background: Some(Colour::from_hex(0xff6600)),
        }))
        .unwrap()
        .jpeg(Some(JpegOptions {
            chroma_subsampling: Some("4:4:4".to_string()),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Flatten 16-bit PNG with transparency to orange
    Sharp::new_from_file(fixtures::inputPngWithTransparency16bit())
        .unwrap()
        .flatten(Some(FlattenOptions {
            background: Some(Colour::new(255, 102, 0, 0.0)),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    // emoves alpha from fixtures with transparency, ignores those without
    Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .resize(10, 10)
        .unwrap()
        .remove_alpha()
        .to_buffer()
        .unwrap();

    // Ensures alpha from fixtures without transparency, ignores those with
    Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .resize(10, 10)
        .unwrap()
        .ensure_alpha(0.5)
        .unwrap()
        .to_buffer()
        .unwrap();
}
