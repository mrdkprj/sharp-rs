#![allow(clippy::all)]
#![allow(unused_imports)]
use icc_profile::DecodedICCProfile;
use num_derive::ToPrimitive;
use sharp::{
    input::{Create, Inputs, SharpOptions},
    metadata::BackgroundColor,
    operation::BlurOptions,
    output::{
        Exif, JpegOptions, PngOptions, WebpOptions, WithIccProfileOptions, WriteableMetadata,
    },
    resize::ResizeOptions,
    Colour, Sharp,
};
use std::{
    collections::HashMap,
    str::{from_utf8, from_utf8_unchecked},
};
mod fixtures;

#[test]
fn metadata() {
    let create = Create {
        width: 1,
        height: 1,
        channels: 3,
        background: Colour::rgb(255, 0, 0),
        ..Default::default()
    };

    //JPEG
    let metadata = Sharp::new_from_file(fixtures::inputJpg()).unwrap().metadata().unwrap();
    assert_eq!("jpeg", metadata.format);
    assert_eq!(2725, metadata.width);
    assert_eq!(2225, metadata.height);
    assert_eq!("srgb", metadata.space);
    assert_eq!(3, metadata.channels);
    assert_eq!("uchar", metadata.depth);
    assert_eq!("4:2:0", metadata.chroma_subsampling);
    assert!(!metadata.is_progressive);
    assert!(!metadata.has_profile);
    assert!(!metadata.has_alpha);

    //JPEG with EXIF/ICC
    let metadata = Sharp::new_from_file(fixtures::inputJpgWithExif()).unwrap().metadata().unwrap();
    assert_eq!("jpeg", metadata.format);
    assert_eq!(450, metadata.width);
    assert_eq!(600, metadata.height);
    assert_eq!("srgb", metadata.space);
    assert_eq!(3, metadata.channels);
    assert_eq!("uchar", metadata.depth);
    assert_eq!(72, metadata.density);
    assert_eq!("4:2:0", metadata.chroma_subsampling);
    assert_eq!(false, metadata.is_progressive);
    assert_eq!(true, metadata.has_profile);
    assert_eq!(false, metadata.has_alpha);
    assert_eq!(8, metadata.orientation);
    let profile = DecodedICCProfile::new(&metadata.icc).unwrap();
    assert_eq!("Generic RGB Profile", profile.tags.get("desc").unwrap().as_string(0));

    //JPEG with IPTC/XMP
    let metadata =
        Sharp::new_from_file(fixtures::inputJpgWithIptcAndXmp()).unwrap().metadata().unwrap();
    assert_eq!(18250, metadata.iptc.len());
    assert_eq!(
        unsafe { from_utf8_unchecked(&metadata.iptc) }.find("Photoshop").unwrap_or_default(),
        0
    );
    assert_eq!(12466, metadata.xmp.len());
    assert_eq!(
        from_utf8(&metadata.xmp).unwrap().find(r#"<?xpacket begin=""#).unwrap_or_default(),
        0
    );
    assert!(from_utf8(&metadata.xmp)
        .unwrap()
        .starts_with(r#"<?xpacket begin="﻿" id="W5M0MpCehiHzreSzNTczkc9d"?>"#));

    //TIFF
    let metadata = Sharp::new_from_file(fixtures::inputTiff()).unwrap().metadata().unwrap();
    assert_eq!("tiff", metadata.format);
    assert_eq!(2464, metadata.width);
    assert_eq!(3248, metadata.height);
    assert_eq!("b-w", metadata.space);
    assert_eq!(1, metadata.channels);
    assert_eq!("uchar", metadata.depth);
    assert_eq!(300, metadata.density);
    assert_eq!(false, metadata.is_progressive);
    assert_eq!(false, metadata.has_profile);
    assert_eq!(false, metadata.has_alpha);
    assert_eq!(1, metadata.orientation);
    assert_eq!(2464, metadata.auto_orient.width);
    assert_eq!(3248, metadata.auto_orient.height);
    assert_eq!("inch", metadata.resolution_unit);

    //Multipage TIFF
    let metadata =
        Sharp::new_from_file(fixtures::inputTiffMultipage()).unwrap().metadata().unwrap();
    assert_eq!("tiff", metadata.format);
    assert_eq!(2464, metadata.width);
    assert_eq!(3248, metadata.height);
    assert_eq!("b-w", metadata.space);
    assert_eq!(1, metadata.channels);
    assert_eq!("uchar", metadata.depth);
    assert_eq!(300, metadata.density);
    assert_eq!(false, metadata.is_progressive);
    assert_eq!(2, metadata.pages);
    assert_eq!(false, metadata.has_alpha);
    assert_eq!(false, metadata.has_alpha);
    assert_eq!(1, metadata.orientation);

    //PNG
    let metadata = Sharp::new_from_file(fixtures::inputPng()).unwrap().metadata().unwrap();
    assert_eq!("png", metadata.format);
    assert_eq!(2809, metadata.width);
    assert_eq!(2074, metadata.height);
    assert_eq!("b-w", metadata.space);
    assert_eq!(1, metadata.channels);
    assert_eq!("uchar", metadata.depth);
    assert_eq!(300, metadata.density);
    assert_eq!(false, metadata.is_progressive);
    assert_eq!(false, metadata.has_profile);
    assert_eq!(false, metadata.has_alpha);
    assert_eq!(2809, metadata.auto_orient.width);
    assert_eq!(2074, metadata.auto_orient.height);

    //PNG with comment
    let metadata =
        Sharp::new_from_file(fixtures::inputPngTestJoinChannel()).unwrap().metadata().unwrap();
    assert_eq!("png", metadata.format);
    assert_eq!(320, metadata.width);
    assert_eq!(240, metadata.height);
    assert_eq!("b-w", metadata.space);
    assert_eq!(1, metadata.channels);
    assert_eq!("uchar", metadata.depth);
    assert_eq!(72, metadata.density);
    assert_eq!(false, metadata.is_progressive);
    assert_eq!(false, metadata.has_profile);
    assert_eq!(false, metadata.has_alpha);
    assert_eq!(1, metadata.comments.len());
    assert!(metadata.comments.contains_key("Comment"));
    assert_eq!("Created with GIMP", metadata.comments.get("Comment").unwrap_or(&String::new()));

    //Transparent PNG
    let metadata =
        Sharp::new_from_file(fixtures::inputPngWithTransparency()).unwrap().metadata().unwrap();
    assert_eq!("png", metadata.format);
    assert_eq!(2048, metadata.width);
    assert_eq!(1536, metadata.height);
    assert_eq!("srgb", metadata.space);
    assert_eq!(4, metadata.channels);
    assert_eq!("uchar", metadata.depth);
    assert_eq!(72, metadata.density);
    assert_eq!(false, metadata.is_progressive);
    assert_eq!(false, metadata.has_profile);
    assert_eq!(true, metadata.has_alpha);

    //PNG with greyscale bKGD chunk - 8 bit
    let metadata =
        Sharp::new_from_file(fixtures::inputPng8BitGreyBackground()).unwrap().metadata().unwrap();
    assert_eq!(0, metadata.background.gray as i32);
    assert_eq!(8, metadata.bits_per_sample);
    assert_eq!(2, metadata.channels);
    assert_eq!(72, metadata.density);
    assert_eq!("uchar", metadata.depth);
    assert_eq!("png", metadata.format);
    assert_eq!(true, metadata.has_alpha);
    assert_eq!(false, metadata.has_profile);
    assert_eq!(32, metadata.height);
    assert_eq!(false, metadata.is_palette);
    assert_eq!(false, metadata.is_progressive);
    assert_eq!("b-w", metadata.space);
    assert_eq!(32, metadata.width);
    assert_eq!(32, metadata.auto_orient.width);
    assert_eq!(32, metadata.auto_orient.height);

    //PNG with greyscale bKGD chunk - 16 bit
    let metadata =
        Sharp::new_from_file(fixtures::inputPng16BitGreyBackground()).unwrap().metadata().unwrap();
    assert_eq!(67, metadata.background.gray as i32);
    assert_eq!(16, metadata.bits_per_sample);
    assert_eq!(2, metadata.channels);
    assert_eq!(72, metadata.density);
    assert_eq!("ushort", metadata.depth);
    assert_eq!("png", metadata.format);
    assert_eq!(true, metadata.has_alpha);
    assert_eq!(false, metadata.has_profile);
    assert_eq!(32, metadata.height);
    assert_eq!(false, metadata.is_palette);
    assert_eq!(false, metadata.is_progressive);
    assert_eq!("grey16", metadata.space);
    assert_eq!(32, metadata.width);
    assert_eq!(32, metadata.auto_orient.width);
    assert_eq!(32, metadata.auto_orient.height);

    //WebP
    let metadata = Sharp::new_from_file(fixtures::inputWebP()).unwrap().metadata().unwrap();
    assert_eq!("webp", metadata.format);
    assert_eq!(1024, metadata.width);
    assert_eq!(772, metadata.height);
    assert_eq!("srgb", metadata.space);
    assert_eq!(3, metadata.channels);
    assert_eq!("uchar", metadata.depth);
    assert_eq!(false, metadata.is_progressive);
    assert_eq!(false, metadata.has_profile);
    assert_eq!(false, metadata.has_alpha);

    //Animated WebP
    let metadata = Sharp::new_from_file(fixtures::inputWebPAnimated()).unwrap().metadata().unwrap();
    assert_eq!(metadata.format, "webp");
    assert_eq!(metadata.width, 80);
    assert_eq!(metadata.height, 80);
    assert_eq!(metadata.space, "srgb");
    assert_eq!(metadata.channels, 4);
    assert_eq!(metadata.depth, "uchar");
    assert_eq!(metadata.is_progressive, false);
    assert_eq!(metadata.pages, 9);
    assert_eq!(metadata.loop_, 0);
    assert_eq!(metadata.delay, [120, 120, 90, 120, 120, 90, 120, 90, 30]);
    assert_eq!(metadata.has_profile, false);
    assert_eq!(metadata.has_alpha, true);

    //Animated WebP with all pages
    let metadata = Sharp::new_from_file_with_opts(
        fixtures::inputWebPAnimated(),
        SharpOptions {
            pages: Some(-1),
            ..Default::default()
        },
    )
    .unwrap()
    .metadata()
    .unwrap();
    assert_eq!(metadata.format, "webp");
    assert_eq!(metadata.width, 80);
    assert_eq!(metadata.height, 720);
    assert_eq!(metadata.space, "srgb");
    assert_eq!(metadata.channels, 4);
    assert_eq!(metadata.depth, "uchar");
    assert_eq!(metadata.is_progressive, false);
    assert_eq!(metadata.pages, 9);
    assert_eq!(metadata.page_height, 80);
    assert_eq!(metadata.loop_, 0);
    assert_eq!(metadata.delay, [120, 120, 90, 120, 120, 90, 120, 90, 30]);
    assert_eq!(metadata.has_profile, false);
    assert_eq!(metadata.has_alpha, true);

    //Animated WebP with limited looping
    let metadata =
        Sharp::new_from_file(fixtures::inputWebPAnimatedLoop3()).unwrap().metadata().unwrap();
    assert_eq!(metadata.format, "webp");
    assert_eq!(metadata.width, 370);
    assert_eq!(metadata.height, 285);
    assert_eq!(metadata.space, "srgb");
    assert_eq!(metadata.channels, 4);
    assert_eq!(metadata.depth, "uchar");
    assert_eq!(metadata.is_progressive, false);
    assert_eq!(metadata.pages, 10);
    assert_eq!(metadata.loop_, 3);
    let mut delay = vec![3000; 9];
    delay.push(15000);
    assert_eq!(metadata.delay, delay);
    assert_eq!(metadata.has_profile, false);
    assert_eq!(metadata.has_alpha, true);

    //GIF
    let metadata = Sharp::new_from_file(fixtures::inputGif()).unwrap().metadata().unwrap();
    assert_eq!("gif", metadata.format);
    assert_eq!(800, metadata.width);
    assert_eq!(533, metadata.height);
    assert_eq!(3, metadata.channels);
    assert_eq!("uchar", metadata.depth);
    assert_eq!(false, metadata.is_progressive);
    assert_eq!(false, metadata.has_profile);
    assert_eq!(
        metadata.background,
        BackgroundColor {
            r: 138.0,
            g: 148.0,
            b: 102.0,
            gray: 0.0
        }
    );

    //GIF grey+alpha
    let metadata =
        Sharp::new_from_file(fixtures::inputGifGreyPlusAlpha()).unwrap().metadata().unwrap();
    assert_eq!("gif", metadata.format);
    assert_eq!(2, metadata.width);
    assert_eq!(1, metadata.height);
    assert_eq!(4, metadata.channels);
    assert_eq!("uchar", metadata.depth);
    assert_eq!(false, metadata.is_progressive);
    assert_eq!(false, metadata.has_profile);

    //Animated GIF
    let metadata = Sharp::new_from_file(fixtures::inputGifAnimated()).unwrap().metadata().unwrap();
    assert_eq!(metadata.format, "gif");
    assert_eq!(metadata.width, 80);
    assert_eq!(metadata.height, 80);
    assert_eq!(metadata.space, "srgb");
    assert_eq!(metadata.channels, 4);
    assert_eq!(metadata.depth, "uchar");
    assert_eq!(metadata.is_progressive, false);
    assert_eq!(metadata.pages, 30);
    assert_eq!(metadata.loop_, 0);
    assert_eq!(metadata.delay, vec![30; 30]);
    assert_eq!(
        metadata.background,
        BackgroundColor {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            gray: 0.0
        }
    );
    assert_eq!(metadata.has_profile, false);
    assert_eq!(metadata.has_alpha, true);

    //Animated GIF with limited looping
    let metadata =
        Sharp::new_from_file(fixtures::inputGifAnimatedLoop3()).unwrap().metadata().unwrap();
    assert_eq!(metadata.format, "gif");
    assert_eq!(metadata.width, 370);
    assert_eq!(metadata.height, 285);
    assert_eq!(metadata.space, "srgb");
    assert_eq!(metadata.channels, 4);
    assert_eq!(metadata.depth, "uchar");
    assert_eq!(metadata.is_progressive, false);
    assert_eq!(metadata.pages, 10);
    assert_eq!(metadata.loop_, 3);
    let mut delay = vec![3000; 9];
    delay.push(15000);
    assert_eq!(metadata.delay, delay);
    assert_eq!(metadata.has_profile, false);
    assert_eq!(metadata.has_alpha, true);

    //vips
    let metadata = Sharp::new_from_file(fixtures::inputV()).unwrap().metadata().unwrap();
    assert_eq!("vips", metadata.format);
    assert_eq!(70, metadata.width);
    assert_eq!(60, metadata.height);
    assert_eq!(3, metadata.channels);
    assert_eq!("uchar", metadata.depth);
    assert_eq!(72, metadata.density);
    assert_eq!(false, metadata.is_progressive);
    assert_eq!(false, metadata.has_profile);
    assert_eq!(false, metadata.has_alpha);

    //Resize to half width using metadata
    let image = Sharp::new_from_file(fixtures::inputJpg()).unwrap();
    let metadata = image.metadata().unwrap();
    assert_eq!("jpeg", metadata.format);
    assert_eq!(2725, metadata.width);
    assert_eq!(2225, metadata.height);
    assert_eq!("srgb", metadata.space);
    assert_eq!(3, metadata.channels);
    assert_eq!("uchar", metadata.depth);
    assert_eq!("4:2:0", metadata.chroma_subsampling);
    assert_eq!(false, metadata.is_progressive);
    assert_eq!(false, metadata.has_profile);
    assert_eq!(false, metadata.has_alpha);
    let (data, info) = image
        .resize_with_opts(ResizeOptions {
            width: Some(metadata.width / 2),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!(1362, info.width);
    assert_eq!(1112, info.height);

    // Keep EXIF metadata and add sRGB profile after a resize
    let data = Sharp::new_from_file(fixtures::inputJpgWithExif())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .with_metadata(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(true, metadata.has_profile);
    assert_eq!(8, metadata.orientation);
    assert_eq!(320, metadata.width);
    assert_eq!(240, metadata.height);
    assert_eq!(240, metadata.auto_orient.width);
    assert_eq!(320, metadata.auto_orient.height);
    assert!(!metadata.exif.is_empty());
    assert!(!metadata.icc.is_empty());
    let profile = DecodedICCProfile::new(&metadata.icc).unwrap();
    assert_eq!(ColorSpaceSignature::RgbData as u32, profile.color_space);
    assert_eq!(Intent::Perceptual as u32, profile.rendering_intent);
    assert_eq!(ProfileClassSignature::DisplayClass as u32, profile.device_class);

    //keep existing ICC profile
    let data = Sharp::new_from_file(fixtures::inputJpgWithExif())
        .unwrap()
        .keep_icc_profile()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    if !metadata.icc.is_empty() {
        let profile = DecodedICCProfile::new(&metadata.icc).unwrap();
        assert_eq!(profile.tags.get("desc").unwrap().as_string(0), "Generic RGB Profile");
    }

    //keep existing ICC profile, ignore colourspace conversion
    let data = Sharp::new_from_file(fixtures::inputJpgWithExif())
        .unwrap()
        .keep_icc_profile()
        .to_colourspace(sharp::Interpretation::Cmyk)
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    if !metadata.icc.is_empty() {
        let profile = DecodedICCProfile::new(&metadata.icc).unwrap();
        assert_eq!(profile.tags.get("desc").unwrap().as_string(0), "Generic RGB Profile");
    }

    //keep existing ICC profile, avoid colour transform
    let data = Sharp::new_from_file(fixtures::inputPngWithProPhotoProfile())
        .unwrap()
        .keep_icc_profile()
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_eq!(data, vec![131, 141, 192]);

    //keep existing CMYK ICC profile
    let data = Sharp::new_from_file(fixtures::inputJpgWithCmykProfile())
        .unwrap()
        .pipeline_colourspace(sharp::Interpretation::Cmyk)
        .to_colourspace(sharp::Interpretation::Cmyk)
        .keep_icc_profile()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    if !metadata.icc.is_empty() {
        let profile = DecodedICCProfile::new(&metadata.icc).unwrap();
        assert_eq!(profile.tags.get("desc").unwrap().as_string(0), "U.S. Web Coated (SWOP) v2");
    }

    //transform to ICC profile and attach
    let data = Sharp::new(Inputs::new().create(create.clone()))
        .unwrap()
        .png(None)
        .unwrap()
        .with_icc_profile(
            "p3",
            Some(WithIccProfileOptions {
                attach: Some(true),
            }),
        )
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert!(!metadata.icc.is_empty());
    let profile = DecodedICCProfile::new(&metadata.icc).unwrap();
    assert!(profile.tags.get("desc").unwrap().as_string(0).contains("sP3C"));

    //transform to ICC profile but do not attach
    let data = Sharp::new(Inputs::new().create(create.clone()))
        .unwrap()
        .png(None)
        .unwrap()
        .with_icc_profile(
            "p3",
            Some(WithIccProfileOptions {
                attach: Some(false),
            }),
        )
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(3, metadata.channels);
    assert!(metadata.icc.is_empty());

    //Apply CMYK output ICC profile
    let output = fixtures::output("output.icc-cmyk.jpg");
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(64),
            ..Default::default()
        })
        .unwrap()
        .with_icc_profile("cmyk", None)
        .to_file(output.clone())
        .unwrap();
    let metadata = Sharp::new_from_file(output.clone()).unwrap().metadata().unwrap();
    assert_eq!(true, metadata.has_profile);
    assert_eq!("cmyk", metadata.space);
    assert_eq!(4, metadata.channels);
    assert!(!metadata.icc.is_empty());
    let profile = DecodedICCProfile::new(&metadata.icc).unwrap();
    assert_eq!(ColorSpaceSignature::CmykData as u32, profile.color_space);
    assert_eq!(Intent::RelativeColorimetric as u32, profile.rendering_intent);
    assert_eq!(ProfileClassSignature::OutputClass as u32, profile.device_class);
    assert_similar!(fixtures::expected("icc-cmyk.jpg"), std::fs::read(output).unwrap(), Some(1));

    //Apply custom output ICC profile
    let output = fixtures::output("output.hilutite.jpg");
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(64),
            ..Default::default()
        })
        .unwrap()
        .with_icc_profile(fixtures::path("hilutite.icm").to_str().unwrap(), None)
        .to_file(output.clone())
        .unwrap();
    assert_max_colour_distance!(output, fixtures::expected("hilutite.jpg"), 9.0);

    //Remove EXIF metadata after a resize
    let data = Sharp::new_from_file(fixtures::inputJpgWithExif())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert!(!metadata.has_profile);
    assert!(metadata.exif.is_empty());
    assert!(metadata.icc.is_empty());

    //Remove metadata from PNG output
    let data = Sharp::new_from_file(fixtures::inputJpgWithExif())
        .unwrap()
        .png(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert!(!metadata.has_profile);
    assert!(metadata.exif.is_empty());
    assert!(metadata.icc.is_empty());

    //Set density of JPEG
    let data = Sharp::new(Inputs::new().create(create.clone()))
        .unwrap()
        .with_metadata(Some(WriteableMetadata {
            density: Some(300.0),
            ..Default::default()
        }))
        .unwrap()
        .jpeg(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(metadata.density, 300);

    //Set density of PNG
    let data = Sharp::new(Inputs::new().create(create.clone()))
        .unwrap()
        .with_metadata(Some(WriteableMetadata {
            density: Some(96.0),
            ..Default::default()
        }))
        .unwrap()
        .png(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(metadata.density, 96);

    //chromaSubsampling 4:4:4:4 CMYK JPEG
    let metadata =
        Sharp::new_from_file(fixtures::inputJpgWithCmykProfile()).unwrap().metadata().unwrap();
    assert_eq!("4:4:4:4", metadata.chroma_subsampling);

    //chromaSubsampling 4:4:4 RGB JPEG
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(10, 10)
        .unwrap()
        .jpeg(Some(JpegOptions {
            chroma_subsampling: Some("4:4:4".to_string()),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!("4:4:4", metadata.chroma_subsampling);

    //isProgressive JPEG
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(10, 10)
        .unwrap()
        .jpeg(Some(JpegOptions {
            progressive: Some(true),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert!(metadata.is_progressive);

    //isProgressive PNG
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(10, 10)
        .unwrap()
        .png(Some(PngOptions {
            progressive: Some(true),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert!(metadata.is_progressive);

    //16-bit TIFF with TIFFTAG_PHOTOSHOP metadata
    let metadata =
        Sharp::new_from_file(fixtures::inputTifftagPhotoshop()).unwrap().metadata().unwrap();
    assert_eq!(metadata.format, "tiff");
    assert_eq!(metadata.width, 317);
    assert_eq!(metadata.height, 211);
    assert_eq!(metadata.space, "rgb16");
    assert_eq!(metadata.channels, 3);
    assert!(!metadata.tifftag_photoshop.is_empty());
    assert_eq!(metadata.tifftag_photoshop.len(), 6634);

    //AVIF
    let metadata = Sharp::new_from_file(fixtures::inputAvif()).unwrap().metadata().unwrap();
    assert_eq!(metadata.format, "heif");
    assert_eq!(metadata.width, 2048);
    assert_eq!(metadata.height, 858);
    assert_eq!(metadata.space, "srgb");
    assert_eq!(metadata.channels, 3);
    assert_eq!(metadata.depth, "uchar");
    assert_eq!(metadata.is_progressive, false);
    assert_eq!(metadata.is_palette, false);
    assert_eq!(metadata.bits_per_sample, 8);
    assert_eq!(metadata.pages, 1);
    assert_eq!(metadata.page_primary, 0);
    assert_eq!(metadata.compression, "av1");
    assert_eq!(metadata.has_profile, false);
    assert_eq!(metadata.has_alpha, false);
    assert_eq!(metadata.auto_orient.width, 2048);
    assert_eq!(metadata.auto_orient.height, 858);

    //withMetadata adds default sRGB profile
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(32, 24)
        .unwrap()
        .with_metadata(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    let profile = DecodedICCProfile::new(&metadata.icc).unwrap();
    assert_eq!(profile.color_space, ColorSpaceSignature::RgbData as u32);
    assert_eq!(profile.device_class, ProfileClassSignature::DisplayClass as u32);
    assert_eq!(profile.rendering_intent, Intent::Perceptual as u32);

    //withMetadata adds default sRGB profile to RGB16
    let data = Sharp::new(Inputs::new().create(create.clone()))
        .unwrap()
        .to_colourspace(sharp::Interpretation::Rgb16)
        .png(None)
        .unwrap()
        .with_metadata(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(metadata.depth, "ushort");
    let profile = DecodedICCProfile::new(&metadata.icc).unwrap();
    assert!(profile.tags.get("desc").unwrap().as_string(0).contains("sRGB"));

    //keepExif maintains all EXIF metadata
    let data1 = Sharp::new(Inputs::new().create(create.clone()))
        .unwrap()
        .with_exif(Exif {
            ifd0: Some(HashMap::from([
                ("Copyright".to_string(), "Test 1".to_string()),
                ("Software".to_string(), "sharp".to_string()),
            ])),
            ..Default::default()
        })
        .jpeg(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    let data2 = Sharp::new_from_buffer(data1).unwrap().keep_exif().to_buffer().unwrap();
    let exif = rexif::parse_buffer(&data2).unwrap();
    let mut result: HashMap<&'static str, String> = HashMap::new();
    exif.entries.iter().for_each(|e| match e.tag {
        rexif::ExifTag::Copyright => {
            let _ = result.insert("Copyright", e.value_more_readable.to_string());
        }
        rexif::ExifTag::Software => {
            let _ = result.insert("Software", e.value_more_readable.to_string());
        }
        _ => {}
    });
    assert_eq!(result.get("Copyright").unwrap(), "Test 1");
    assert_eq!(result.get("Software").unwrap(), "sharp");

    //withExif replaces all EXIF metadata
    let data1 = Sharp::new(Inputs::new().create(create.clone()))
        .unwrap()
        .with_exif(Exif {
            ifd0: Some(HashMap::from([
                ("Copyright".to_string(), "Test 1".to_string()),
                ("Software".to_string(), "sharp".to_string()),
            ])),
            ..Default::default()
        })
        .jpeg(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    let exif = rexif::parse_buffer(&data1).unwrap();
    let mut result: HashMap<&'static str, String> = HashMap::new();
    exif.entries.iter().for_each(|e| match e.tag {
        rexif::ExifTag::Copyright => {
            let _ = result.insert("Copyright", e.value_more_readable.to_string());
        }
        rexif::ExifTag::Software => {
            let _ = result.insert("Software", e.value_more_readable.to_string());
        }
        _ => {}
    });
    assert_eq!(result.get("Copyright").unwrap(), "Test 1");
    assert_eq!(result.get("Software").unwrap(), "sharp");

    let data2 = Sharp::new_from_buffer(data1)
        .unwrap()
        .with_exif(Exif {
            ifd0: Some(HashMap::from([("Copyright".to_string(), "Test 2".to_string())])),
            ..Default::default()
        })
        .to_buffer()
        .unwrap();

    let exif = rexif::parse_buffer(&data2).unwrap();
    let mut result: HashMap<&'static str, String> = HashMap::new();
    exif.entries.iter().for_each(|e| match e.tag {
        rexif::ExifTag::Copyright => {
            let _ = result.insert("Copyright", e.value_more_readable.to_string());
        }
        rexif::ExifTag::Software => {
            let _ = result.insert("Software", e.value_more_readable.to_string());
        }
        _ => {}
    });
    assert_eq!(result.get("Copyright").unwrap(), "Test 2");
    assert!(!result.contains_key("Software"));

    //withExifMerge merges all EXIF metadata
    let data1 = Sharp::new(Inputs::new().create(create.clone()))
        .unwrap()
        .with_exif(Exif {
            ifd0: Some(HashMap::from([("Copyright".to_string(), "Test 1".to_string())])),
            ..Default::default()
        })
        .jpeg(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    let exif = rexif::parse_buffer(&data1).unwrap();
    let mut result: HashMap<&'static str, String> = HashMap::new();
    exif.entries.iter().for_each(|e| match e.tag {
        rexif::ExifTag::Copyright => {
            let _ = result.insert("Copyright", e.value_more_readable.to_string());
        }
        rexif::ExifTag::Software => {
            let _ = result.insert("Software", e.value_more_readable.to_string());
        }
        _ => {}
    });
    assert_eq!(result.get("Copyright").unwrap(), "Test 1");
    assert!(!result.contains_key("Software"));

    let data2 = Sharp::new_from_buffer(data1)
        .unwrap()
        .with_exif(Exif {
            ifd0: Some(HashMap::from([
                ("Copyright".to_string(), "Test 2".to_string()),
                ("Software".to_string(), "sharp".to_string()),
            ])),
            ..Default::default()
        })
        .to_buffer()
        .unwrap();

    let exif = rexif::parse_buffer(&data2).unwrap();
    let mut result: HashMap<&'static str, String> = HashMap::new();
    exif.entries.iter().for_each(|e| match e.tag {
        rexif::ExifTag::Copyright => {
            let _ = result.insert("Copyright", e.value_more_readable.to_string());
        }
        rexif::ExifTag::Software => {
            let _ = result.insert("Software", e.value_more_readable.to_string());
        }
        _ => {}
    });
    assert_eq!(result.get("Copyright").unwrap(), "Test 2");
    assert_eq!(result.get("Software").unwrap(), "sharp");

    //withMetadata preserves existing XMP metadata from input
    let data = Sharp::new_from_file(fixtures::inputJpgWithIptcAndXmp())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .with_metadata(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert!(from_utf8(&metadata.xmp).unwrap().starts_with(r#"<?xpacket begin=""#));

    //keepXmp preserves existing XMP metadata from input
    let data = Sharp::new_from_file(fixtures::inputJpgWithIptcAndXmp())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .keep_xmp()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert!(from_utf8(&metadata.xmp).unwrap().starts_with(r#"<?xpacket begin=""#));

    //withXmp with custom XMP replaces existing XMP
    let xmp = r#"<?xml version="1.0"?><x:xmpmeta xmlns:x="adobe:ns:meta/"><rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"><rdf:Description rdf:about="" xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:creator><rdf:Seq><rdf:li>Test Creator</rdf:li></rdf:Seq></dc:creator><dc:title><rdf:Alt><rdf:li xml:lang="x-default">Test Title</rdf:li></rdf:Alt></dc:title></rdf:Description></rdf:RDF></x:xmpmeta>"#;
    let data = Sharp::new_from_file(fixtures::inputJpgWithIptcAndXmp())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .with_xmp(xmp)
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert!(from_utf8(&metadata.xmp).unwrap().contains("Test Creator"));
    assert!(from_utf8(&metadata.xmp).unwrap().contains("Test Title"));

    //withXmp with custom XMP buffer on image without existing XMP
    let xmp = r#"<?xml version="1.0"?><x:xmpmeta xmlns:x="adobe:ns:meta/"><rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"><rdf:Description rdf:about="" xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:description><rdf:Alt><rdf:li xml:lang="x-default">Added via Sharp</rdf:li></rdf:Alt></dc:description></rdf:Description></rdf:RDF></x:xmpmeta>"#;
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .with_xmp(xmp)
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert!(from_utf8(&metadata.xmp).unwrap().contains("Added via Sharp"));

    //withXmp with valid XMP metadata for different image formats
    let xmp = r#"<?xml version="1.0"?><x:xmpmeta xmlns:x="adobe:ns:meta/"><rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"><rdf:Description rdf:about="" xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:subject><rdf:Bag><rdf:li>test</rdf:li><rdf:li>metadata</rdf:li></rdf:Bag></dc:subject></rdf:Description></rdf:RDF></x:xmpmeta>"#;
    // Test with JPEG output
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .with_xmp(xmp)
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert!(from_utf8(&metadata.xmp).unwrap().contains("test"));
    // Test with PNG output (PNG should also support XMP metadata)
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(100, 100)
        .unwrap()
        .png(None)
        .unwrap()
        .with_xmp(xmp)
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert!(from_utf8(&metadata.xmp).unwrap().contains("test"));
    // Test with WebP output (WebP should also support XMP metadata)
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(100, 100)
        .unwrap()
        .webp(None)
        .unwrap()
        .with_xmp(xmp)
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert!(from_utf8(&metadata.xmp).unwrap().contains("test"));

    //XMP metadata persists through multiple operations
    let xmp = r#"<?xml version="1.0"?><x:xmpmeta xmlns:x="adobe:ns:meta/"><rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"><rdf:Description rdf:about="" xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:identifier>persistent-test</dc:identifier></rdf:Description></rdf:RDF></x:xmpmeta>"#;
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .with_xmp(xmp)
        .rotate(90, None)
        .unwrap()
        .blur(Some(BlurOptions {
            sigma: 1.0,
            ..Default::default()
        }))
        .unwrap()
        .sharpen(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert!(from_utf8(&metadata.xmp).unwrap().contains("persistent-test"));

    //withXmp XMP works with WebP format specifically
    let xmp = r#"<?xml version="1.0"?><x:xmpmeta xmlns:x="adobe:ns:meta/"><rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"><rdf:Description rdf:about="" xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:creator><rdf:Seq><rdf:li>WebP Creator</rdf:li></rdf:Seq></dc:creator><dc:format>image/webp</dc:format></rdf:Description></rdf:RDF></x:xmpmeta>"#;
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(120, 80)
        .unwrap()
        .webp(Some(WebpOptions {
            quality: Some(80),
            ..Default::default()
        }))
        .unwrap()
        .with_xmp(xmp)
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert!(from_utf8(&metadata.xmp).unwrap().contains("WebP Creator"));
    assert!(from_utf8(&metadata.xmp).unwrap().contains("image/webp"));

    fixtures::clean_up();
    rs_vips::Vips::shutdown();
}

#[derive(ToPrimitive)]
pub enum ColorSpaceSignature {
    XYZData = 1_482_250_784,
    LabData = 1_281_450_528,
    LuvData = 1_282_766_368,
    YCbCrData = 1_497_588_338,
    YxyData = 1_501_067_552,
    RgbData = 1_380_401_696,
    GrayData = 1_196_573_017,
    HsvData = 1_213_421_088,
    HlsData = 1_212_961_568,
    CmykData = 1_129_142_603,
    CmyData = 1_129_142_560,
    MCH1Data = 1_296_255_025,
    MCH2Data = 1_296_255_026,
    MCH3Data = 1_296_255_027,
    MCH4Data = 1_296_255_028,
    MCH5Data = 1_296_255_029,
    MCH6Data = 1_296_255_030,
    MCH7Data = 1_296_255_031,
    MCH8Data = 1_296_255_032,
    MCH9Data = 1_296_255_033,
    MCHAData = 1_296_255_041,
    MCHBData = 1_296_255_042,
    MCHCData = 1_296_255_043,
    MCHDData = 1_296_255_044,
    MCHEData = 1_296_255_045,
    MCHFData = 1_296_255_046,
    NamedData = 1_852_662_636,
    Sig1colorData = 826_494_034,
    Sig2colorData = 843_271_250,
    Sig3colorData = 860_048_466,
    Sig4colorData = 876_825_682,
    Sig5colorData = 893_602_898,
    Sig6colorData = 910_380_114,
    Sig7colorData = 927_157_330,
    Sig8colorData = 943_934_546,
    Sig9colorData = 960_711_762,
    Sig10colorData = 1_094_929_490,
    Sig11colorData = 1_111_706_706,
    Sig12colorData = 1_128_483_922,
    Sig13colorData = 1_145_261_138,
    Sig14colorData = 1_162_038_354,
    Sig15colorData = 1_178_815_570,
    LuvKData = 1_282_766_411,
}

#[derive(ToPrimitive)]
pub enum Intent {
    Perceptual = 0,
    RelativeColorimetric = 1,
    Saturation = 2,
    AbsoluteColorimetric = 3,
    PreserveKOnlyPerceptual = 10,
    PreserveKOnlyRelativeColorimetric = 11,
    PreserveKOnlySaturation = 12,
    PreserveKPlanePerceptual = 13,
    PreserveKPlaneRelativeColorimetric = 14,
    PreserveKPlaneSaturation = 15,
}

#[derive(ToPrimitive)]
pub enum ProfileClassSignature {
    InputClass = 1_935_896_178,
    DisplayClass = 1_835_955_314,
    OutputClass = 1_886_549_106,
    LinkClass = 1_818_848_875,
    AbstractClass = 1_633_842_036,
    ColorSpaceClass = 1_936_744_803,
    NamedColorClass = 1_852_662_636,
}
