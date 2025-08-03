mod fixtures;
use sharp::Sharp;

#[test]
pub fn gamma() {
    //value of 0.0 (disabled)
    Sharp::new_from_file(fixtures::inputJpgWithGammaHoliness())
        .unwrap()
        .resize(129, 111)
        .unwrap()
        .to_buffer()
        .unwrap();

    //value of 2.2 (default)
    Sharp::new_from_file(fixtures::inputJpgWithGammaHoliness())
        .unwrap()
        .resize(129, 111)
        .unwrap()
        .gamma(None, None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //'value of 3.0
    Sharp::new_from_file(fixtures::inputJpgWithGammaHoliness())
        .unwrap()
        .resize(129, 111)
        .unwrap()
        .gamma(Some(3.0), None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //input value of 2.2, output value of 3.0
    Sharp::new_from_file(fixtures::inputJpgWithGammaHoliness())
        .unwrap()
        .resize(129, 111)
        .unwrap()
        .gamma(Some(2.2), Some(3.0))
        .unwrap()
        .to_buffer()
        .unwrap();

    //alpha transparency
    Sharp::new_from_file(fixtures::inputPngOverlayLayer1())
        .unwrap()
        .resize(320, 320)
        .unwrap()
        .gamma(None, None)
        .unwrap()
        .jpeg(None)
        .unwrap()
        .to_buffer()
        .unwrap();
}
