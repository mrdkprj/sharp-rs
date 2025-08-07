use sharp::{operation::SharpenOptions, Sharp};
mod fixtures;

#[test]
fn sharpen() {
    //specific radius 10 (sigma 6)
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .sharpen(Some(SharpenOptions {
            sigma: 6.0,
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //specific radius 3 (sigma 1.5) and levels 0.5, 2.5
    Sharp::new_from_file(fixtures::inputJpg())
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
        .to_buffer()
        .unwrap();

    //specific radius 5 (sigma 3.5) and levels 2, 4
    Sharp::new_from_file(fixtures::inputJpg())
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

    //sigma=3.5, m1=2, m2=4, x1=2, y2=5, y3=25
    Sharp::new_from_file(fixtures::inputJpg())
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

    //mild sharpen
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .sharpen(None)
        .unwrap()
        .to_buffer()
        .unwrap();
}
