use rs_vips::{
    bindings::vips_foreign_find_load,
    voption::{Setter, VOption},
    VipsImage,
};
use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    path::Path,
    sync::OnceLock,
};

pub fn assert_max_colour_distance<P1: AsRef<Path>, P2: AsRef<Path>>(
    actual: P1,
    expected: P2,
    acceptedDistance: f64,
) -> Result<(), String> {
    /*
        if (typeof acceptedDistance !== 'number') {
      // Default threshold
      acceptedDistance = 1;
    }
    const distance = maxColourDistance(actualImagePath, expectedImagePath);
    if (distance > acceptedDistance) {
      throw new Error('Expected maximum absolute distance of ' + acceptedDistance + ', actual ' + distance);
    }
     */

    let distance = max_colour_distance(actual, expected)?;
    if distance > acceptedDistance {
        return Err(format!(
            "Expected maximum absolute distance of {:?}, actual {:?}",
            acceptedDistance, distance
        ));
    }

    Ok(())
}
fn max_colour_distance<P1: AsRef<Path>, P2: AsRef<Path>>(
    actual: P1,
    expected: P2,
) -> Result<f64, String> {
    let image_type1 = determindetermine_image_type(actual.as_ref().to_str().unwrap());

    if image_type1 == ImageType::UNKNOWN {
        return Err("Input file 1 is of an unsupported image format".to_string());
    }

    let mut image1 =
        VipsImage::new_from_file(actual.as_ref().to_str().unwrap()).map_err(|e| e.to_string())?;

    let image_type2 = determindetermine_image_type(expected.as_ref().to_str().unwrap());
    if image_type2 == ImageType::UNKNOWN {
        return Err("Input file 2 is of an unsupported image format".to_string());
    }

    let mut image2 =
        VipsImage::new_from_file(actual.as_ref().to_str().unwrap()).map_err(|e| e.to_string())?;

    if image1.get_bands() != image2.get_bands() {
        return Err("mismatchedBands".to_string());
    }

    if image1.get_width() != image2.get_width() || image1.get_height() != image2.get_height() {
        return Err("mismatchedDimensions".to_string());
    }

    if image1.image_hasalpha() {
        image1 = image1
            .premultiply()
            .map_err(|e| e.to_string())?
            .extract_band_with_opts(1, VOption::new().set("n", image1.get_bands() - 1))
            .map_err(|e| e.to_string())?;
    }

    if image2.image_hasalpha() {
        image2 = image2
            .premultiply()
            .map_err(|e| e.to_string())?
            .extract_band_with_opts(1, VOption::new().set("n", image2.get_bands() - 1))
            .map_err(|e| e.to_string())?;
    }

    let maxColourDistance =
        image1.dE00(&image2).map_err(|e| e.to_string())?.max().map_err(|e| e.to_string())?;

    Ok(maxColourDistance)
}

fn determindetermine_image_type(file: &str) -> ImageType {
    let filename =
        CString::new(file).map_err(|_| "Error initializing C string.".to_string()).unwrap();
    let load = unsafe { vips_foreign_find_load(filename.as_ptr() as _) };

    if load.is_null() {
        return ImageType::UNKNOWN;
    }

    let c_str = unsafe { CStr::from_ptr(load) };
    if let Ok(loader_name) = c_str.to_str() {
        loader_to_type().get(loader_name).cloned().unwrap_or(ImageType::UNKNOWN)
    } else {
        ImageType::UNKNOWN
    }
}

// Static loader-to-type map, initialized once
fn loader_to_type() -> &'static HashMap<&'static str, ImageType> {
    static MAP: OnceLock<HashMap<&'static str, ImageType>> = OnceLock::new();
    MAP.get_or_init(|| {
        use ImageType::*;
        HashMap::from([
            ("VipsForeignLoadJpegFile", Jpeg),
            ("VipsForeignLoadJpegBuffer", Jpeg),
            ("VipsForeignLoadPngFile", Png),
            ("VipsForeignLoadPngBuffer", Png),
            ("VipsForeignLoadWebpFile", Webp),
            ("VipsForeignLoadWebpBuffer", Webp),
            ("VipsForeignLoadTiffFile", Tiff),
            ("VipsForeignLoadTiffBuffer", Tiff),
            ("VipsForeignLoadGifFile", GIF),
            ("VipsForeignLoadGifBuffer", GIF),
            ("VipsForeignLoadNsgifFile", GIF),
            ("VipsForeignLoadNsgifBuffer", GIF),
            ("VipsForeignLoadJp2kFile", JP2),
            ("VipsForeignLoadJp2kBuffer", JP2),
            ("VipsForeignLoadSvgFile", SVG),
            ("VipsForeignLoadSvgBuffer", SVG),
            ("VipsForeignLoadHeifFile", HEIF),
            ("VipsForeignLoadHeifBuffer", HEIF),
            ("VipsForeignLoadPdfFile", PDF),
            ("VipsForeignLoadPdfBuffer", PDF),
            ("VipsForeignLoadMagickFile", MAGICK),
            ("VipsForeignLoadMagickBuffer", MAGICK),
            ("VipsForeignLoadMagick7File", MAGICK),
            ("VipsForeignLoadMagick7Buffer", MAGICK),
            ("VipsForeignLoadOpenslideFile", OPENSLIDE),
            ("VipsForeignLoadPpmFile", PPM),
            ("VipsForeignLoadFitsFile", FITS),
            ("VipsForeignLoadOpenexr", EXR),
            ("VipsForeignLoadJxlFile", JXL),
            ("VipsForeignLoadJxlBuffer", JXL),
            ("VipsForeignLoadRadFile", RAD),
            ("VipsForeignLoadRadBuffer", RAD),
            ("VipsForeignLoadVips", VIPS),
            ("VipsForeignLoadVipsFile", VIPS),
            ("VipsForeignLoadRaw", RAW),
        ])
    })
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum ImageType {
    Jpeg,
    Png,
    Webp,
    JP2,
    Tiff,
    GIF,
    SVG,
    HEIF,
    PDF,
    MAGICK,
    OPENSLIDE,
    PPM,
    FITS,
    EXR,
    JXL,
    RAD,
    VIPS,
    RAW,
    UNKNOWN,
    MISSING,
}
