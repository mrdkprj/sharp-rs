#![allow(non_snake_case)]
#![allow(dead_code)]

use std::path::{Path, PathBuf};
pub fn inputJpgWithLandscapeExif1() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Landscape_1.jpg")
}
pub fn inputJpgWithLandscapeExif2() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Landscape_2.jpg")
}
pub fn inputJpgWithLandscapeExif3() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Landscape_3.jpg")
}
pub fn inputJpgWithLandscapeExif4() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Landscape_4.jpg")
}
pub fn inputJpgWithLandscapeExif5() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Landscape_5.jpg")
}
pub fn inputJpgWithLandscapeExif6() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Landscape_6.jpg")
}
pub fn inputJpgWithLandscapeExif7() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Landscape_7.jpg")
}
pub fn inputJpgWithLandscapeExif8() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Landscape_8.jpg")
}

pub fn inputJpgWithPortraitExif1() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Portrait_1.jpg")
}
pub fn inputJpgWithPortraitExif2() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Portrait_2.jpg")
}
pub fn inputJpgWithPortraitExif3() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Portrait_3.jpg")
}
pub fn inputJpgWithPortraitExif4() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Portrait_4.jpg")
}
pub fn inputJpgWithPortraitExif5() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Portrait_5.jpg")
}
pub fn inputJpgWithPortraitExif6() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Portrait_6.jpg")
}
pub fn inputJpgWithPortraitExif7() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Portrait_7.jpg")
}
pub fn inputJpgWithPortraitExif8() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Portrait_8.jpg")
}

pub fn inputJpg() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("img")
        .join("2569067123_aca715a2ee_o.jpg")
}
pub fn inputJpgWithExif() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Landscape_8.jpg")
}
pub fn inputJpgWithIptcAndXmp() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Landscape_9.jpg")
}
pub fn inputJpgWithExifMirroring() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Landscape_5.jpg")
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
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("corrupt-header.jpg")
}
pub fn inputJpgWithLowContrast() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("low-contrast.jpg")
}
pub fn inputJpgLarge() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("giant-image.jpg")
}
pub fn inputJpg320x240() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("320x240.jpg")
}
pub fn inputJpgOverlayLayer2() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("alpha-layer-2-ink.jpg")
}
pub fn inputJpgTruncated() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("truncated.jpg")
}
pub fn inputJpgCenteredImage() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("centered_image.jpeg")
}
pub fn inputJpgRandom() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("random.jpg")
}
pub fn inputJpgThRandom() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("thRandom.jpg")
}
pub fn inputJpgLossless() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("testimgl.jpg")
}

pub fn inputPng() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("50020484-00001.png")
}
pub fn inputPngGradients() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("gradients-rgb8.png")
}
pub fn inputPngWithTransparency() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("blackbug.png")
}
pub fn inputPngCompleteTransparency() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("full-transparent.png")
}
pub fn inputPngWithGreyAlpha() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("grey-8bit-alpha.png")
}
pub fn inputPngWithOneColor() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("2x2_fdcce6.png")
}
pub fn inputPngWithTransparency16bit() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("tbgn2c16.png")
}
pub fn inputPng8BitGreyBackground() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("bgbn4a08.png")
}
pub fn inputPng16BitGreyBackground() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("bggn4a16.png")
}
pub fn inputPng16BitGreyAlpha() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("16-bit-grey-alpha.png")
}
pub fn inputPngOverlayLayer0() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("img")
        .join("alpha-layer-0-background.png")
}
pub fn inputPngOverlayLayer1() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("alpha-layer-1-fill.png")
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
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("bandbool.png")
}
pub fn inputPngTestJoinChannel() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("testJoinChannel.png")
}
pub fn inputPngTruncated() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("truncated.png")
}
pub fn inputPngEmbed() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("embedgravitybird.png")
}
pub fn inputPngRGBWithAlpha() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("img")
        .join("2569067123_aca715a2ee_o.png")
}
pub fn inputPngImageInAlpha() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("image-in-alpha.png")
}
pub fn inputPngSolidAlpha() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("with-alpha.png")
}
pub fn inputPngP3() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("p3.png")
}
pub fn inputPngPalette() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("swiss.png")
}
pub fn inputPngTrimIncludeAlpha() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("trim-mc.png")
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
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("65536-uint32-limit.png")
}
pub fn inputPngWithProPhotoProfile() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("prophoto.png")
}

pub fn inputWebP() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("4.webp")
}
pub fn inputWebPWithTransparency() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("5_webp_a.webp")
}
pub fn inputWebPAnimated() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("rotating-squares.webp")
}
pub fn inputWebPAnimatedLoop3() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("animated-loop-3.webp")
}
pub fn inputWebPAnimatedBigHeight() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("big-height.webp")
}
pub fn inputTiff() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("G31D.TIF")
}
pub fn inputTiffMultipage() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("G31D_MULTI.TIF")
}
pub fn inputTiffCielab() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("cielab-dagams.tiff")
}
pub fn inputTiffUncompressed() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("uncompressed_tiff.tiff")
}
pub fn inputTiff8BitDepth() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("8bit_depth.tiff")
}
pub fn inputTifftagPhotoshop() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("tifftag-photoshop.tiff")
}
pub fn inputTiffFogra() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("fogra-0-100-100-0.tif")
}

pub fn inputJp2() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("relax.jp2")
}
pub fn inputJp2TileParts() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("relax_tileparts.jp2")
}
pub fn inputGif() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("Crash_test.gif")
}
pub fn inputGifGreyPlusAlpha() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("grey-plus-alpha.gif")
}
pub fn inputGifAnimated() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("rotating-squares.gif")
}
pub fn inputGifAnimatedLoop3() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("animated-loop-3.gif")
}
pub fn inputSvg() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("check.svg")
}
pub fn inputSvgSmallViewBox() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("circle.svg")
}
pub fn inputSvgWithEmbeddedImages() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("struct-image-04-t.svg")
}
pub fn inputAvif() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("img")
        .join("sdr_cosmos12920_cicp1-13-6_yuv444_full_qp10.avif")
}

pub fn inputJPGBig() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("flowers.jpeg")
}

pub fn inputPngDotAndLines() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("dot-and-lines.png")
}

pub fn inputPngStripesV() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("stripesV.png")
}
pub fn inputPngStripesH() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("stripesH.png")
}

pub fn inputJpgBooleanTest() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("booleanTest.jpg")
}

pub fn inputV() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("vfile.v")
}

pub fn inputJpgClahe() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("concert.jpg")
}

pub fn testPattern() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("test-pattern.png")
}

pub fn inputPngWithTransparent() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("img").join("d.png")
}
