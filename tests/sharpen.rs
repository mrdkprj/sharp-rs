use sharp::{operation::SharpenOptions, Sharp};
mod fixtures;

#[test]
fn sharpen() {
    //specific radius 10 (sigma 6)
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .sharpen(Some(SharpenOptions {
            sigma: 6.0,
            ..Default::default()
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("sharpen-10.jpg"), data, None);

    //specific radius 3 (sigma 1.5) and levels 0.5, 2.5
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .sharpen(Some(SharpenOptions {
            sigma: 1.5,
            m1: Some(0.5),
            m2: Some(2.5),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("sharpen-3-0.5-2.5.jpg"), data, None);

    //specific radius 5 (sigma 3.5) and levels 2, 4
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .sharpen(Some(SharpenOptions {
            sigma: 3.5,
            m1: Some(2.0),
            m2: Some(4.0),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("sharpen-5-2-4.jpg"), data, None);

    //sigma=3.5, m1=2, m2=4
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .sharpen(Some(SharpenOptions {
            sigma: 3.5,
            m1: Some(2.0),
            m2: Some(4.0),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("sharpen-5-2-4.jpg"), data, None);

    //sigma=3.5, m1=2, m2=4, x1=2, y2=5, y3=25
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .sharpen(Some(SharpenOptions {
            sigma: 3.5,
            m1: Some(2.0),
            m2: Some(4.0),
            x1: Some(2.0),
            y2: Some(5.0),
            y3: Some(25.0),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("sharpen-5-2-4.jpg"), data, None);

    //mild sharpen
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .sharpen(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("sharpen-mild.jpg"), data, None);

    //sharpened image is larger than non-sharpened
    let (not_sharpened, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);

    let (sharpened, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .sharpen(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(sharpened.len() > not_sharpened.len());
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
}
