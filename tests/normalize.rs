mod fixtures;
use sharp::{operation::NormaliseOptions, Sharp};

#[test]
pub fn normalize() {
    //spreads rgb image values between 0 and 255
    Sharp::new_from_file(fixtures::inputJpgWithLowContrast())
        .unwrap()
        .normalise(None)
        .unwrap()
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //spreads grayscaled image values between 0 and 255
    Sharp::new_from_file(fixtures::inputJpgWithLowContrast())
        .unwrap()
        .greyscale(true)
        .normalise(None)
        .unwrap()
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //stretches greyscale images with alpha channel
    Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .normalise(None)
        .unwrap()
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //keeps an existing alpha channel
    Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .resize(8, 8)
        .unwrap()
        .normalise(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //keeps the alpha channel of greyscale images intact
    Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .resize(8, 8)
        .unwrap()
        .normalise(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //works with 16-bit RGBA images
    Sharp::new_from_file(fixtures::inputPngWithTransparency16bit())
        .unwrap()
        .normalise(None)
        .unwrap()
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //should handle luminance range
    Sharp::new_from_file(fixtures::inputPngWithTransparency16bit())
        .unwrap()
        .normalise(Some(NormaliseOptions {
            lower: Some(10),
            upper: Some(70),
        }))
        .unwrap()
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();
}
