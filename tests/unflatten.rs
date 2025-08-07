use sharp::{operation::ThresholdOptions, Sharp};
mod fixtures;

#[test]
fn unflatten() {
    //unflatten white background
    Sharp::new_from_file(fixtures::inputPng()).unwrap().unflatten().unwrap().to_buffer().unwrap();

    //unflatten transparent image
    Sharp::new_from_file(fixtures::inputPngTrimSpecificColourIncludeAlpha())
        .unwrap()
        .unflatten()
        .unwrap()
        .to_buffer()
        .unwrap();

    //unflatten using threshold
    Sharp::new_from_file(fixtures::inputPngPalette())
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
}
