use sharp::{operation::ThresholdOptions, Sharp};
mod fixtures;

#[test]
fn threshold() {
    //'threshold 1 jpeg
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .threshold(Some(1), None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //threshold 40 jpeg
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .threshold(Some(40), None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //threshold 128 jpeg
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .threshold(Some(128), None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //threshold 0 jpeg
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .threshold(Some(0), None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //threshold grayscale: true (=128)
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .threshold(
            Some(128),
            Some(ThresholdOptions {
                grayscale: Some(true),
            }),
        )
        .unwrap()
        .to_buffer()
        .unwrap();

    //threshold default jpeg
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .threshold(None, None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //threshold default png transparency
    Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .threshold(None, None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //threshold default png alpha
    Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .threshold(None, None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //threshold default webp transparency
    Sharp::new_from_file(fixtures::inputWebPWithTransparency())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .threshold(None, None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //color threshold
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
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
