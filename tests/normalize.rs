mod fixtures;
use sharp::{operation::NormaliseOptions, Sharp};

fn assert_normalized(data: Vec<u8>) {
    let mut min = 255u8;
    let mut max = 0u8;
    for x in data {
        min = min.min(x);
        max = max.max(x);
    }
    assert_eq!(0, min);
    assert!(max > 248);
}

#[test]
pub fn normalize() {
    //spreads rgb image values between 0 and 255
    let data = Sharp::new_from_file(fixtures::inputJpgWithLowContrast())
        .unwrap()
        .normalise(None)
        .unwrap()
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_normalized(data);

    //spreads grayscaled image values between 0 and 255
    let data = Sharp::new_from_file(fixtures::inputJpgWithLowContrast())
        .unwrap()
        .greyscale(true)
        .normalise(None)
        .unwrap()
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_normalized(data);

    //stretches greyscale images with alpha channel
    let data = Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .normalise(None)
        .unwrap()
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_normalized(data);

    //keeps an existing alpha channel
    let data = Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .resize(8, 8)
        .unwrap()
        .normalise(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(4, metadata.channels);
    assert!(metadata.has_alpha);
    assert_eq!("srgb", metadata.space);

    //keeps the alpha channel of greyscale images intact
    let data = Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .resize(8, 8)
        .unwrap()
        .normalise(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(4, metadata.channels);
    assert!(metadata.has_alpha);
    assert_eq!("srgb", metadata.space);

    //does not alter images with only one color
    let output = fixtures::output("output.unmodified-png-with-one-color.png");
    Sharp::new_from_file(fixtures::inputPngWithOneColor())
        .unwrap()
        .normalise(None)
        .unwrap()
        .to_file(output.clone())
        .unwrap();
    assert_max_colour_distance!(output, fixtures::inputPngWithOneColor(), 0.0);

    //works with 16-bit RGBA images
    let data = Sharp::new_from_file(fixtures::inputPngWithTransparency16bit())
        .unwrap()
        .normalise(None)
        .unwrap()
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_normalized(data);

    //should handle luminance range
    let data = Sharp::new_from_file(fixtures::inputPngWithTransparency16bit())
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
    assert_normalized(data);

    rs_vips::Vips::shutdown();
}
