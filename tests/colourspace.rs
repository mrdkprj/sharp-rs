mod fixtures;
use sharp::{resize::ResizeOptions, Colour, Interpretation, Sharp};
use std::path::Path;

#[test]
pub fn colourspace() {
    //To greyscale
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .greyscale(true)
        .to_file(fixtures::output("output.greyscale-gamma-0.0.jpg"))
        .unwrap();

    //To greyscale with gamma correction
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .gamma(None, None)
        .unwrap()
        .greyscale(true)
        .to_file(fixtures::output("output.greyscale-gamma-2.2.jpg"))
        .unwrap();

    //Not to greyscale
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .greyscale(false)
        .to_file(fixtures::output("output.greyscale-not.jpg"))
        .unwrap();

    //Greyscale with single channel output
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .greyscale(true)
        .to_colourspace(Interpretation::BW)
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(1, info.channels);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("output.greyscale-single.jpg"), data, None);

    //From 1-bit TIFF to sRGB WebP
    let (_, info) = Sharp::new_from_file(fixtures::inputTiff())
        .unwrap()
        .resize(8, 8)
        .unwrap()
        .webp(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("webp".to_string(), info.format);

    //From CMYK to sRGB
    Sharp::new_from_file(fixtures::inputJpgWithCmykProfile())
        .unwrap()
        .resize(320, 320)
        .unwrap()
        .to_buffer()
        .unwrap();

    //From CMYK to sRGB with white background, not yellow
    let data = Sharp::new_from_file(fixtures::inputJpgWithCmykProfile())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(240),
            fit: Some(sharp::resize::Fit::Contain),
            background: Some(Colour::new(255, 255, 255, 1.0)),
            ..Default::default()
        })
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("colourspace.cmyk.jpg"), data, None);

    //From profile-less CMYK to sRGB
    let data = Sharp::new_from_file(fixtures::inputJpgWithCmykNoProfile())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            ..Default::default()
        })
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("colourspace.cmyk-without-profile.jpg"), data, None);

    //Profile-less CMYK roundtrip
    let buf = Sharp::new_from_file(fixtures::inputJpgWithCmykNoProfile())
        .unwrap()
        .pipeline_colourspace(Interpretation::Cmyk)
        .to_colourspace(Interpretation::Cmyk)
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_eq!(buf[0..4], vec![55, 27, 0, 0]);

    //CMYK profile to CMYK profile conversion using perceptual intent
    let buf = Sharp::new_from_file(fixtures::inputTiffFogra())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .to_colourspace(Interpretation::Cmyk)
        .pipeline_colourspace(Interpretation::Cmyk)
        .with_icc_profile(fixtures::path("XCMYK 2017.icc").to_str().unwrap(), None)
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_eq!(buf[0..4], vec![1, 239, 227, 5]);

    //CMYK profile to CMYK profile with negate
    let (data, info) = Sharp::new_from_file(fixtures::inputTiffFogra())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .to_colourspace(Interpretation::Cmyk)
        .pipeline_colourspace(Interpretation::Cmyk)
        .with_icc_profile(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("img")
                .join("XCMYK 2017.icc")
                .to_str()
                .unwrap(),
            None,
        )
        .negate(true, None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("tiff".to_string(), info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("colourspace.cmyk-to-cmyk-negated.tif"), data, Some(0));

    //From sRGB with RGB16 pipeline, resize with gamma, to sRGB
    let data = Sharp::new_from_file(fixtures::inputPngGradients())
        .unwrap()
        .pipeline_colourspace(Interpretation::Rgb16)
        .resize(320, 320)
        .unwrap()
        .to_colourspace(Interpretation::Srgb)
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("colourspace-gradients-gamma-resize.png"), data, Some(0));

    //Convert P3 to sRGB
    let buf = Sharp::new_from_file(fixtures::inputPngP3())
        .unwrap()
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_eq!(buf[0..3], vec![255, 0, 0]);

    //Passthrough P3
    let buf = Sharp::new_from_file(fixtures::inputPngP3())
        .unwrap()
        .with_icc_profile("p3", None)
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_eq!(buf[0..3], vec![234, 51, 34]);

    fixtures::clean_up();
    rs_vips::Vips::shutdown();
}
