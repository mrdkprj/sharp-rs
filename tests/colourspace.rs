mod fixtures;
use sharp::{resize::ResizeOptions, Colour, Interpretation, Sharp};
use std::path::Path;

#[test]
pub fn colourspace() {
    //Colour space conversion
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .greyscale(true)
        .to_buffer()
        .unwrap();

    //To greyscale with gamma correction
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .gamma(None, None)
        .unwrap()
        .greyscale(true)
        .to_buffer()
        .unwrap();

    //Not to greyscale
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .greyscale(false)
        .to_buffer()
        .unwrap();

    //Greyscale with single channel output
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .greyscale(true)
        .to_colourspace(Interpretation::BW)
        .to_buffer()
        .unwrap();

    //From 1-bit TIFF to sRGB WebP
    Sharp::new_from_file(fixtures::inputTiff())
        .unwrap()
        .resize(8, 8)
        .unwrap()
        .webp(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //From CMYK to sRGB
    Sharp::new_from_file(fixtures::inputJpgWithCmykProfile())
        .unwrap()
        .resize(320, 320)
        .unwrap()
        .to_buffer()
        .unwrap();

    //From CMYK to sRGB with white background, not yellow
    Sharp::new_from_file(fixtures::inputJpgWithCmykProfile())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: 320,
            height: 240,
            fit: Some(sharp::resize::Fit::Contain),
            background: Some(Colour::new(255, 255, 255, 1.0)),
            ..Default::default()
        })
        .unwrap()
        .to_buffer()
        .unwrap();

    //From profile-less CMYK to sRGB
    Sharp::new_from_file(fixtures::inputJpgWithCmykNoProfile())
        .unwrap()
        .resize(320, 320)
        .unwrap()
        .to_buffer()
        .unwrap();

    //Profile-less CMYK roundtrip
    Sharp::new_from_file(fixtures::inputJpgWithCmykNoProfile())
        .unwrap()
        .pipeline_colourspace(Interpretation::Cmyk)
        .to_colourspace(Interpretation::Cmyk)
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //CMYK profile to CMYK profile conversion using perceptual intent
    Sharp::new_from_file(fixtures::inputTiffFogra())
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
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //CMYK profile to CMYK profile with negate
    Sharp::new_from_file(fixtures::inputTiffFogra())
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
        .negate(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //From sRGB with RGB16 pipeline, resize with gamma, to sRGB
    Sharp::new_from_file(fixtures::inputPngGradients())
        .unwrap()
        .pipeline_colourspace(Interpretation::Rgb16)
        .resize(320, 320)
        .unwrap()
        .to_colourspace(Interpretation::Srgb)
        .to_buffer()
        .unwrap();

    //Convert P3 to sRGB
    Sharp::new_from_file(fixtures::inputPngP3()).unwrap().raw(None).unwrap().to_buffer().unwrap();

    //Passthrough P3
    Sharp::new_from_file(fixtures::inputPngP3())
        .unwrap()
        .with_icc_profile("p3", None)
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();
}
