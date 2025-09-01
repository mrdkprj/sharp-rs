mod fixtures;
use sharp::{resize::ResizeOptions, Sharp};

#[test]
pub fn gamma() {
    //value of 0.0 (disabled)
    let data = Sharp::new_from_file(fixtures::inputJpgWithGammaHoliness())
        .unwrap()
        .resize(129, 111)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("gamma-0.0.jpg"), data, None);

    //value of 2.2 (default)
    let data = Sharp::new_from_file(fixtures::inputJpgWithGammaHoliness())
        .unwrap()
        .resize(129, 111)
        .unwrap()
        .gamma(None, None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("gamma-2.2.jpg"), data, None);

    //'value of 3.0
    let data = Sharp::new_from_file(fixtures::inputJpgWithGammaHoliness())
        .unwrap()
        .resize(129, 111)
        .unwrap()
        .gamma(Some(3.0), None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("gamma-3.0.jpg"), data, None);

    //input value of 2.2, output value of 3.0
    let data = Sharp::new_from_file(fixtures::inputJpgWithGammaHoliness())
        .unwrap()
        .resize(129, 111)
        .unwrap()
        .gamma(Some(2.2), Some(3.0))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("gamma-in-2.2-out-3.0.jpg"), data, None);

    //alpha transparency
    let data = Sharp::new_from_file(fixtures::inputPngOverlayLayer1())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            ..Default::default()
        })
        .unwrap()
        .gamma(None, None)
        .unwrap()
        .jpeg(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("gamma-alpha.jpg"), data, None);

    rs_vips::Vips::shutdown();
}
