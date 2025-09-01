use sharp::{operation::ThresholdOptions, Sharp};
mod fixtures;

#[test]
fn threshold() {
    //'threshold 1 jpeg
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .threshold(Some(1), None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("threshold-1.jpg"), data, None);

    //threshold 40 jpeg
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .threshold(Some(40), None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("threshold-40.jpg"), data, None);

    //threshold 128
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .threshold(Some(128), None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("threshold-128.jpg"), data, None);

    //threshold 0
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .threshold(Some(0), None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::inputJpg(), data, None);

    //threshold grayscale: true (=128)
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
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
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("threshold-128.jpg"), data, None);

    //threshold default jpeg
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .threshold(None, None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("threshold-128.jpg"), data, None);

    //threshold default png transparency
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .threshold(None, None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("threshold-128-transparency.png"), data, None);

    //threshold default png alpha
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .threshold(None, None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("threshold-128-alpha.png"), data, None);

    //threshold default webp transparency
    let data = Sharp::new_from_file(fixtures::inputWebPWithTransparency())
        .unwrap()
        .threshold(None, None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("threshold-128-transparency.webp"), data, None);

    //color threshold
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
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
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("threshold-color-128.jpg"), data, None);

    rs_vips::Vips::shutdown();
}
