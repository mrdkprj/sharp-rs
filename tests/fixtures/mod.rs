#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
use sharp::{operation::FlattenOptions, resize::ResizeOptions, Colour, Sharp};
use std::path::{Path, PathBuf};
use zouni::fs;

use crate::fixtures;
pub mod maxcolor;

pub fn inputJpgWithLandscapeExif1() -> PathBuf {
    fixtures::path("Landscape_1.jpg")
}
pub fn inputJpgWithLandscapeExif2() -> PathBuf {
    fixtures::path("Landscape_2.jpg")
}
pub fn inputJpgWithLandscapeExif3() -> PathBuf {
    fixtures::path("Landscape_3.jpg")
}
pub fn inputJpgWithLandscapeExif4() -> PathBuf {
    fixtures::path("Landscape_4.jpg")
}
pub fn inputJpgWithLandscapeExif5() -> PathBuf {
    fixtures::path("Landscape_5.jpg")
}
pub fn inputJpgWithLandscapeExif6() -> PathBuf {
    fixtures::path("Landscape_6.jpg")
}
pub fn inputJpgWithLandscapeExif7() -> PathBuf {
    fixtures::path("Landscape_7.jpg")
}
pub fn inputJpgWithLandscapeExif8() -> PathBuf {
    fixtures::path("Landscape_8.jpg")
}
pub fn inputJpgWithPortraitExif1() -> PathBuf {
    fixtures::path("Portrait_1.jpg")
}
pub fn inputJpgWithPortraitExif2() -> PathBuf {
    fixtures::path("Portrait_2.jpg")
}
pub fn inputJpgWithPortraitExif3() -> PathBuf {
    fixtures::path("Portrait_3.jpg")
}
pub fn inputJpgWithPortraitExif4() -> PathBuf {
    fixtures::path("Portrait_4.jpg")
}
pub fn inputJpgWithPortraitExif5() -> PathBuf {
    fixtures::path("Portrait_5.jpg")
}
pub fn inputJpgWithPortraitExif6() -> PathBuf {
    fixtures::path("Portrait_6.jpg")
}
pub fn inputJpgWithPortraitExif7() -> PathBuf {
    fixtures::path("Portrait_7.jpg")
}
pub fn inputJpgWithPortraitExif8() -> PathBuf {
    fixtures::path("Portrait_8.jpg")
}
pub fn inputJpg() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("img")
        .join("2569067123_aca715a2ee_o.jpg")
}
pub fn inputJpgWithExif() -> PathBuf {
    fixtures::path("Landscape_8.jpg")
}
pub fn inputJpgWithIptcAndXmp() -> PathBuf {
    fixtures::path("Landscape_9.jpg")
}
pub fn inputJpgWithExifMirroring() -> PathBuf {
    fixtures::path("Landscape_5.jpg")
}
pub fn inputJpgWithGammaHoliness() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("img")
        .join("gamma_dalai_lama_gray.jpg")
}
pub fn inputJpgWithCmykProfile() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("img")
        .join("Channel_digital_image_CMYK_color.jpg")
}
pub fn inputJpgWithCmykNoProfile() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("img")
        .join("Channel_digital_image_CMYK_color_no_profile.jpg")
}
pub fn inputJpgWithCorruptHeader() -> PathBuf {
    fixtures::path("corrupt-header.jpg")
}
pub fn inputJpgWithLowContrast() -> PathBuf {
    fixtures::path("low-contrast.jpg")
}
pub fn inputJpgLarge() -> PathBuf {
    fixtures::path("giant-image.jpg")
}
pub fn inputJpg320x240() -> PathBuf {
    fixtures::path("320x240.jpg")
}
pub fn inputJpgOverlayLayer2() -> PathBuf {
    fixtures::path("alpha-layer-2-ink.jpg")
}
pub fn inputJpgTruncated() -> PathBuf {
    fixtures::path("truncated.jpg")
}
pub fn inputJpgCenteredImage() -> PathBuf {
    fixtures::path("centered_image.jpeg")
}
pub fn inputJpgRandom() -> PathBuf {
    fixtures::path("random.jpg")
}
pub fn inputJpgThRandom() -> PathBuf {
    fixtures::path("thRandom.jpg")
}
pub fn inputJpgLossless() -> PathBuf {
    fixtures::path("testimgl.jpg")
}
pub fn inputPng() -> PathBuf {
    fixtures::path("50020484-00001.png")
}
pub fn inputPngGradients() -> PathBuf {
    fixtures::path("gradients-rgb8.png")
}
pub fn inputPngWithTransparency() -> PathBuf {
    fixtures::path("blackbug.png")
}
pub fn inputPngCompleteTransparency() -> PathBuf {
    fixtures::path("full-transparent.png")
}
pub fn inputPngWithGreyAlpha() -> PathBuf {
    fixtures::path("grey-8bit-alpha.png")
}
pub fn inputPngWithOneColor() -> PathBuf {
    fixtures::path("2x2_fdcce6.png")
}
pub fn inputPngWithTransparency16bit() -> PathBuf {
    fixtures::path("tbgn2c16.png")
}
pub fn inputPng8BitGreyBackground() -> PathBuf {
    fixtures::path("bgbn4a08.png")
}
pub fn inputPng16BitGreyBackground() -> PathBuf {
    fixtures::path("bggn4a16.png")
}
pub fn inputPng16BitGreyAlpha() -> PathBuf {
    fixtures::path("16-bit-grey-alpha.png")
}
pub fn inputPngOverlayLayer0() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("img")
        .join("alpha-layer-0-background.png")
}
pub fn inputPngOverlayLayer1() -> PathBuf {
    fixtures::path("alpha-layer-1-fill.png")
}
pub fn inputPngAlphaPremultiplicationSmall() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("img")
        .join("alpha-premultiply-1024x768-paper.png")
}
pub fn inputPngAlphaPremultiplicationLarge() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("img")
        .join("alpha-premultiply-2048x1536-paper.png")
}
pub fn inputPngBooleanNoAlpha() -> PathBuf {
    fixtures::path("bandbool.png")
}
pub fn inputPngTestJoinChannel() -> PathBuf {
    fixtures::path("testJoinChannel.png")
}
pub fn inputPngTruncated() -> PathBuf {
    fixtures::path("truncated.png")
}
pub fn inputPngEmbed() -> PathBuf {
    fixtures::path("embedgravitybird.png")
}
pub fn inputPngRGBWithAlpha() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("img")
        .join("2569067123_aca715a2ee_o.png")
}
pub fn inputPngImageInAlpha() -> PathBuf {
    fixtures::path("image-in-alpha.png")
}
pub fn inputPngSolidAlpha() -> PathBuf {
    fixtures::path("with-alpha.png")
}
pub fn inputPngP3() -> PathBuf {
    fixtures::path("p3.png")
}
pub fn inputPngPalette() -> PathBuf {
    fixtures::path("swiss.png")
}
pub fn inputPngTrimIncludeAlpha() -> PathBuf {
    fixtures::path("trim-mc.png")
}
pub fn inputPngTrimSpecificColour() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("img")
        .join("Flag_of_the_Netherlands.png")
}
pub fn inputPngTrimSpecificColour16bit() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("img")
        .join("Flag_of_the_Netherlands-16bit.png")
}
pub fn inputPngTrimSpecificColourIncludeAlpha() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("img")
        .join("Flag_of_the_Netherlands-alpha.png")
}
pub fn inputPngUint32Limit() -> PathBuf {
    fixtures::path("65536-uint32-limit.png")
}
pub fn inputPngWithProPhotoProfile() -> PathBuf {
    fixtures::path("prophoto.png")
}
pub fn inputWebP() -> PathBuf {
    fixtures::path("4.webp")
}
pub fn inputWebPWithTransparency() -> PathBuf {
    fixtures::path("5_webp_a.webp")
}
pub fn inputWebPAnimated() -> PathBuf {
    fixtures::path("rotating-squares.webp")
}
pub fn inputWebPAnimatedLoop3() -> PathBuf {
    fixtures::path("animated-loop-3.webp")
}
pub fn inputWebPAnimatedBigHeight() -> PathBuf {
    fixtures::path("big-height.webp")
}
pub fn inputTiff() -> PathBuf {
    fixtures::path("G31D.TIF")
}
pub fn inputTiffMultipage() -> PathBuf {
    fixtures::path("G31D_MULTI.TIF")
}
pub fn inputTiffCielab() -> PathBuf {
    fixtures::path("cielab-dagams.tiff")
}
pub fn inputTiffUncompressed() -> PathBuf {
    fixtures::path("uncompressed_tiff.tiff")
}
pub fn inputTiff8BitDepth() -> PathBuf {
    fixtures::path("8bit_depth.tiff")
}
pub fn inputTifftagPhotoshop() -> PathBuf {
    fixtures::path("tifftag-photoshop.tiff")
}
pub fn inputTiffFogra() -> PathBuf {
    fixtures::path("fogra-0-100-100-0.tif")
}
pub fn inputJp2() -> PathBuf {
    fixtures::path("relax.jp2")
}
pub fn inputJp2TileParts() -> PathBuf {
    fixtures::path("relax_tileparts.jp2")
}
pub fn inputGif() -> PathBuf {
    fixtures::path("Crash_test.gif")
}
pub fn inputGifGreyPlusAlpha() -> PathBuf {
    fixtures::path("grey-plus-alpha.gif")
}
pub fn inputGifAnimated() -> PathBuf {
    fixtures::path("rotating-squares.gif")
}
pub fn inputGifAnimatedLoop3() -> PathBuf {
    fixtures::path("animated-loop-3.gif")
}
pub fn inputSvg() -> PathBuf {
    fixtures::path("check.svg")
}
pub fn inputSvgSmallViewBox() -> PathBuf {
    fixtures::path("circle.svg")
}
pub fn inputSvgWithEmbeddedImages() -> PathBuf {
    fixtures::path("struct-image-04-t.svg")
}
pub fn inputAvif() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("img")
        .join("sdr_cosmos12920_cicp1-13-6_yuv444_full_qp10.avif")
}
pub fn inputJPGBig() -> PathBuf {
    fixtures::path("flowers.jpeg")
}
pub fn inputPngDotAndLines() -> PathBuf {
    fixtures::path("dot-and-lines.png")
}
pub fn inputPngStripesV() -> PathBuf {
    fixtures::path("stripesV.png")
}
pub fn inputPngStripesH() -> PathBuf {
    fixtures::path("stripesH.png")
}
pub fn inputJpgBooleanTest() -> PathBuf {
    fixtures::path("booleanTest.jpg")
}
pub fn inputV() -> PathBuf {
    fixtures::path("vfile.v")
}
pub fn inputJpgClahe() -> PathBuf {
    fixtures::path("concert.jpg")
}
pub fn testPattern() -> PathBuf {
    fixtures::path("test-pattern.png")
}
pub fn inputPngWithTransparent() -> PathBuf {
    fixtures::path("d.png")
}

pub fn input(name: &str) -> PathBuf {
    match name {
        "inputJpgWithLandscapeExif1" => inputJpgWithLandscapeExif1(),
        "inputJpgWithLandscapeExif2" => inputJpgWithLandscapeExif2(),
        "inputJpgWithLandscapeExif3" => inputJpgWithLandscapeExif3(),
        "inputJpgWithLandscapeExif4" => inputJpgWithLandscapeExif4(),
        "inputJpgWithLandscapeExif5" => inputJpgWithLandscapeExif5(),
        "inputJpgWithLandscapeExif6" => inputJpgWithLandscapeExif6(),
        "inputJpgWithLandscapeExif7" => inputJpgWithLandscapeExif7(),
        "inputJpgWithLandscapeExif8" => inputJpgWithLandscapeExif8(),
        "inputJpgWithPortraitExif1" => inputJpgWithPortraitExif1(),
        "inputJpgWithPortraitExif2" => inputJpgWithPortraitExif2(),
        "inputJpgWithPortraitExif3" => inputJpgWithPortraitExif3(),
        "inputJpgWithPortraitExif4" => inputJpgWithPortraitExif4(),
        "inputJpgWithPortraitExif5" => inputJpgWithPortraitExif5(),
        "inputJpgWithPortraitExif6" => inputJpgWithPortraitExif6(),
        "inputJpgWithPortraitExif7" => inputJpgWithPortraitExif7(),
        "inputJpgWithPortraitExif8" => inputJpgWithPortraitExif8(),
        "inputJpg" => inputJpg(),
        "inputJpgWithExif" => inputJpgWithExif(),
        "inputJpgWithIptcAndXmp" => inputJpgWithIptcAndXmp(),
        "inputJpgWithExifMirroring" => inputJpgWithExifMirroring(),
        "inputJpgWithGammaHoliness" => inputJpgWithGammaHoliness(),
        "inputJpgWithCmykProfile" => inputJpgWithCmykProfile(),
        "inputJpgWithCmykNoProfile" => inputJpgWithCmykNoProfile(),
        "inputJpgWithCorruptHeader" => inputJpgWithCorruptHeader(),
        "inputJpgWithLowContrast" => inputJpgWithLowContrast(),
        "inputJpgLarge" => inputJpgLarge(),
        "inputJpg320x240" => inputJpg320x240(),
        "inputJpgOverlayLayer2" => inputJpgOverlayLayer2(),
        "inputJpgTruncated" => inputJpgTruncated(),
        "inputJpgCenteredImage" => inputJpgCenteredImage(),
        "inputJpgRandom" => inputJpgRandom(),
        "inputJpgThRandom" => inputJpgThRandom(),
        "inputJpgLossless" => inputJpgLossless(),
        "inputPng" => inputPng(),
        "inputPngGradients" => inputPngGradients(),
        "inputPngWithTransparency" => inputPngWithTransparency(),
        "inputPngCompleteTransparency" => inputPngCompleteTransparency(),
        "inputPngWithGreyAlpha" => inputPngWithGreyAlpha(),
        "inputPngWithOneColor" => inputPngWithOneColor(),
        "inputPngWithTransparency16bit" => inputPngWithTransparency16bit(),
        "inputPng8BitGreyBackground" => inputPng8BitGreyBackground(),
        "inputPng16BitGreyBackground" => inputPng16BitGreyBackground(),
        "inputPng16BitGreyAlpha" => inputPng16BitGreyAlpha(),
        "inputPngOverlayLayer0" => inputPngOverlayLayer0(),
        "inputPngOverlayLayer1" => inputPngOverlayLayer1(),
        "inputPngAlphaPremultiplicationSmall" => inputPngAlphaPremultiplicationSmall(),
        "inputPngAlphaPremultiplicationLarge" => inputPngAlphaPremultiplicationLarge(),
        "inputPngBooleanNoAlpha" => inputPngBooleanNoAlpha(),
        "inputPngTestJoinChannel" => inputPngTestJoinChannel(),
        "inputPngTruncated" => inputPngTruncated(),
        "inputPngEmbed" => inputPngEmbed(),
        "inputPngRGBWithAlpha" => inputPngRGBWithAlpha(),
        "inputPngImageInAlpha" => inputPngImageInAlpha(),
        "inputPngSolidAlpha" => inputPngSolidAlpha(),
        "inputPngP3" => inputPngP3(),
        "inputPngPalette" => inputPngPalette(),
        "inputPngTrimIncludeAlpha" => inputPngTrimIncludeAlpha(),
        "inputPngTrimSpecificColour" => inputPngTrimSpecificColour(),
        "inputPngTrimSpecificColour16bit" => inputPngTrimSpecificColour16bit(),
        "inputPngTrimSpecificColourIncludeAlpha" => inputPngTrimSpecificColourIncludeAlpha(),
        "inputPngUint32Limit" => inputPngUint32Limit(),
        "inputPngWithProPhotoProfile" => inputPngWithProPhotoProfile(),
        "inputWebP" => inputWebP(),
        "inputWebPWithTransparency" => inputWebPWithTransparency(),
        "inputWebPAnimated" => inputWebPAnimated(),
        "inputWebPAnimatedLoop3" => inputWebPAnimatedLoop3(),
        "inputWebPAnimatedBigHeight" => inputWebPAnimatedBigHeight(),
        "inputTiff" => inputTiff(),
        "inputTiffMultipage" => inputTiffMultipage(),
        "inputTiffCielab" => inputTiffCielab(),
        "inputTiffUncompressed" => inputTiffUncompressed(),
        "inputTiff8BitDepth" => inputTiff8BitDepth(),
        "inputTifftagPhotoshop" => inputTifftagPhotoshop(),
        "inputTiffFogra" => inputTiffFogra(),
        "inputJp2" => inputJp2(),
        "inputJp2TileParts" => inputJp2TileParts(),
        "inputGif" => inputGif(),
        "inputGifGreyPlusAlpha" => inputGifGreyPlusAlpha(),
        "inputGifAnimated" => inputGifAnimated(),
        "inputGifAnimatedLoop3" => inputGifAnimatedLoop3(),
        "inputSvg" => inputSvg(),
        "inputSvgSmallViewBox" => inputSvgSmallViewBox(),
        "inputSvgWithEmbeddedImages" => inputSvgWithEmbeddedImages(),
        "inputAvif" => inputAvif(),
        "inputJPGBig" => inputJPGBig(),
        "inputPngDotAndLines" => inputPngDotAndLines(),
        "inputPngStripesV" => inputPngStripesV(),
        "inputPngStripesH" => inputPngStripesH(),
        "inputJpgBooleanTest" => inputJpgBooleanTest(),
        "inputV" => inputV(),
        "inputJpgClahe" => inputJpgClahe(),
        "testPattern" => testPattern(),
        _ => testPattern(),
    }
}

enum PathOrBuffer {
    Path(String),
    Buffer(Vec<u8>),
}

fn fingerprint(image: PathOrBuffer) -> Result<Vec<u8>, String> {
    let sharp = match image {
        PathOrBuffer::Buffer(buffer) => Sharp::new_from_buffer(buffer.clone()).unwrap(),
        PathOrBuffer::Path(file) => Sharp::new_from_file(file).unwrap(),
    };

    let data = sharp
        .flatten(None)
        .unwrap()
        .greyscale(true)
        .normalise(None)
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(9),
            height: Some(8),
            fit: Some(sharp::resize::Fit::Fill),
            ..Default::default()
        })
        .unwrap()
        .raw(None)
        .unwrap()
        .to_buffer()?;

    let mut fingerprint = Vec::new();
    for col in 0..8 {
        for row in 0..8 {
            let left = data[(row * 8) + col];
            let right = data[(row * 8) + col + 1];
            fingerprint.push(if left < right {
                1
            } else {
                0
            });
        }
    }
    Ok(fingerprint)
}

pub fn assert_similar_<P: AsRef<Path>>(
    expected: P,
    actual: Vec<u8>,
    threshold: Option<i32>,
) -> Result<(), String> {
    let expectedFingerprint =
        fingerprint(PathOrBuffer::Path(expected.as_ref().to_string_lossy().to_string()))?;
    let actualFingerprint = fingerprint(PathOrBuffer::Buffer(actual))?;
    let mut distance = 0;
    for i in 0..64 {
        if expectedFingerprint[i] != actualFingerprint[i] {
            distance += 1;
        }
    }

    let threshold = threshold.unwrap_or(5);
    if distance > threshold {
        return Err(format!(
            "Expected maximum similarity distance: {:?}. Actual:{:?}",
            threshold, distance
        ));
    }

    Ok(())
}

pub fn path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join(name)
}

pub fn expected(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("expected").join(name)
}

pub fn output(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("output").join(name)
}

pub fn clean_up() {
    fs::readdir(output(""), false, false)
        .unwrap()
        .into_iter()
        .for_each(|ent| fs::delete(ent.full_path).unwrap());
}

#[macro_export]
macro_rules! assert_similar {
    ($expected:expr, $actual:expr, $th:expr) => {
        $crate::fixtures::assert_similar_($expected, $actual, $th).unwrap();
    };
}

// Default threshold = 1;
#[macro_export]
macro_rules! assert_max_colour_distance {
    ($actual:expr, $expected:expr, $th:expr) => {
        $crate::fixtures::maxcolor::assert_max_colour_distance($actual, $expected, $th).unwrap();
    };
}
