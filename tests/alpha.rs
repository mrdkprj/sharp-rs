use sharp::{
    input::{Create, Inputs},
    operation::FlattenOptions,
    output::JpegOptions,
    Colour, Sharp,
};
mod fixtures;

#[test]
pub fn alpha() {
    //Flatten to black
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .flatten(None)
        .unwrap()
        .resize(400, 300)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(400, info.width);
    assert_eq!(300, info.height);
    assert_similar!(fixtures::expected("flatten-black.jpg"), data, None);

    //Flatten to RGB orange
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .resize(400, 300)
        .unwrap()
        .flatten(Some(FlattenOptions {
            background: Some(Colour::rgb(255, 102, 0)),
        }))
        .unwrap()
        .jpeg(Some(JpegOptions {
            chroma_subsampling: Some("4:4:4".to_string()),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(400, info.width);
    assert_eq!(300, info.height);
    assert_similar!(fixtures::expected("flatten-orange.jpg"), data, None);

    //Flatten to CSS/hex orange
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithTransparency())
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
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(400, info.width);
    assert_eq!(300, info.height);
    assert_similar!(fixtures::expected("flatten-orange.jpg"), data, None);

    //Flatten 16-bit PNG with transparency to orange
    let output = fixtures::output("output.flatten-rgb16-orange.jpg");
    Sharp::new_from_file(fixtures::inputPngWithTransparency16bit())
        .unwrap()
        .flatten(Some(FlattenOptions {
            background: Some(Colour::rgb(255, 102, 0)),
        }))
        .unwrap()
        .to_file(output.clone())
        .unwrap();
    assert_max_colour_distance!(output, fixtures::expected("flatten-rgb16-orange.jpg"), 10.0);

    //Ignored for JPEG
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .flatten(Some(FlattenOptions {
            background: Some(Colour::rgb(255, 0, 0)),
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg".to_string(), info.format);
    assert_eq!(3, info.channels);

    //Enlargement with non-nearest neighbor interpolation shouldn’t cause dark edges
    let expected = "alpha-premultiply-enlargement-2048x1536-paper.png";
    let actual = fixtures::output("output.alpha-premultiply-enlargement-2048x1536-paper.png");
    Sharp::new_from_file(fixtures::inputPngAlphaPremultiplicationSmall())
        .unwrap()
        .resize(2048, 1536)
        .unwrap()
        .to_file(actual.clone())
        .unwrap();
    assert_max_colour_distance!(actual, fixtures::expected(expected), 102.0);

    //Reduction with non-nearest neighbor interpolation shouldn’t cause dark edges
    let expected = "alpha-premultiply-reduction-1024x768-paper.png";
    let actual = fixtures::output("output.alpha-premultiply-reduction-1024x768-paper.png");
    Sharp::new_from_file(fixtures::inputPngAlphaPremultiplicationLarge())
        .unwrap()
        .resize(1024, 768)
        .unwrap()
        .to_file(actual.clone())
        .unwrap();
    assert_max_colour_distance!(actual, fixtures::expected(expected), 102.0);

    // Removes alpha from fixtures with transparency, ignores those without
    let (_, info) = Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .resize(10, 10)
        .unwrap()
        .remove_alpha()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(3, info.channels);

    // Ensures alpha from fixtures without transparency, ignores those with
    let (_, info) = Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .resize(10, 10)
        .unwrap()
        .ensure_alpha(1.0)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(4, info.channels);

    //Valid ensureAlpha value used for alpha channel
    let buf = Sharp::new(Inputs::new().create(Create {
        width: 8,
        height: 8,
        channels: 3,
        background: Colour::rgb(255, 0, 0),
        ..Default::default()
    }))
    .unwrap()
    .ensure_alpha(0.5)
    .unwrap()
    .raw(None)
    .unwrap()
    .to_buffer()
    .unwrap();
    assert_eq!(buf[0..4], vec![255, 0, 0, 127]);

    fixtures::clean_up();

    rs_vips::Vips::shutdown();
}
