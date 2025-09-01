use sharp::{operation::ThresholdOptions, Sharp};
mod fixtures;

#[test]
fn unflatten() {
    //unflatten white background
    let data = Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .unflatten()
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("unflatten-white-transparent.png"), data, Some(0));

    //unflatten transparent image
    let data = Sharp::new_from_file(fixtures::inputPngTrimSpecificColourIncludeAlpha())
        .unwrap()
        .unflatten()
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("unflatten-flag-white-transparent.png"), data, Some(0));

    //unflatten using threshold
    let data = Sharp::new_from_file(fixtures::inputPngPalette())
        .unwrap()
        .unflatten()
        .unwrap()
        .threshold(
            Some(128),
            Some(ThresholdOptions {
                grayscale: Some(false),
            }),
        )
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("unflatten-swiss.png"), data, Some(1));

    rs_vips::Vips::shutdown();
}
