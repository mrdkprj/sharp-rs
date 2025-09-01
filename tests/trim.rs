use sharp::{
    input::{Create, Inputs},
    output::PngOptions,
    resize::{ExtendOptions, ResizeOptions, TrimOptions},
    Colour, Sharp,
};
mod fixtures;

#[test]
fn trim() {
    //Skip shrink-on-load
    let expected = fixtures::expected("alpha-layer-2-trim-resize.jpg");
    let (data, info) = Sharp::new_from_file(fixtures::inputJpgOverlayLayer2())
        .unwrap()
        .trim(None)
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(300),
            height: Some(300),
            fast_shrink_on_load: Some(false),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(300, info.width);
    assert!(info.trim_offset_left >= -873 && info.trim_offset_left <= -870);
    assert_eq!(-554, info.trim_offset_top);
    assert_similar!(expected, data, None);

    //Single colour PNG where alpha channel provides the image
    let (_, info) = Sharp::new_from_file(fixtures::inputPngImageInAlpha())
        .unwrap()
        .trim(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(916, info.width);
    assert_eq!(137, info.height);
    assert_eq!(4, info.channels);
    assert_eq!(-6, info.trim_offset_left);
    assert_eq!(-20, info.trim_offset_top);

    //16-bit PNG with alpha channel
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithTransparency16bit())
        .unwrap()
        .resize(32, 32)
        .unwrap()
        .trim(Some(TrimOptions {
            threshold: Some(20.0),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(32, info.width);
    assert_eq!(32, info.height);
    assert_eq!(4, info.channels);
    assert_eq!(-2, info.trim_offset_left);
    assert_eq!(-2, info.trim_offset_top);
    assert_similar!(fixtures::expected("trim-16bit-rgba.png"), data, None);

    //Should rotate before trim
    let rotated30 = Sharp::new(Inputs::new().create(Create {
        width: 20,
        height: 30,
        channels: 3,
        background: Colour::new(255, 255, 255, 1.0),
        ..Default::default()
    }))
    .unwrap()
    .rotate(30, None)
    .unwrap()
    .png(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    let (_, info) = Sharp::new_from_buffer(rotated30)
        .unwrap()
        .rotate(-30, None)
        .unwrap()
        .trim(Some(TrimOptions {
            threshold: Some(128.0),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(20, info.width);
    assert_eq!(31, info.height);
    assert_eq!(-8, info.trim_offset_top);
    assert_eq!(-13, info.trim_offset_left);

    //Ensure trim uses bounding box of alpha and non-alpha channels
    let (_, info) = Sharp::new_from_file(fixtures::inputPngTrimIncludeAlpha())
        .unwrap()
        .trim(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(info.width, 179);
    assert_eq!(info.height, 123);
    assert_eq!(info.trim_offset_top, -44);
    assert_eq!(info.trim_offset_left, -13);

    //Ensure greyscale image can be trimmed
    let greyscale = Sharp::new(Inputs::new().create(Create {
        width: 16,
        height: 8,
        channels: 3,
        background: Colour::from_hex(0xc0c0c0),
        ..Default::default()
    }))
    .unwrap()
    .extend(ExtendOptions {
        left: Some(12),
        right: Some(24),
        background: Some(Colour::from_hex(0x808080)),
        ..Default::default()
    })
    .unwrap()
    .to_colourspace(sharp::Interpretation::BW)
    .png(Some(PngOptions {
        compression_level: Some(0),
        ..Default::default()
    }))
    .unwrap()
    .to_buffer()
    .unwrap();

    let (_, info) = Sharp::new_from_buffer(greyscale)
        .unwrap()
        .trim(None)
        .unwrap()
        .raw(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(info.width, 16);
    assert_eq!(info.height, 8);
    assert_eq!(info.trim_offset_top, 0);
    assert_eq!(info.trim_offset_left, -12);

    //Ensure cmyk image can be trimmed
    let cmyk = Sharp::new(Inputs::new().create(Create {
        width: 16,
        height: 8,
        channels: 3,
        background: Colour::from_hex(0xff0000),
        ..Default::default()
    }))
    .unwrap()
    .extend(ExtendOptions {
        left: Some(12),
        right: Some(24),
        background: Some(Colour::from_hex(0x0000ff)),
        ..Default::default()
    })
    .unwrap()
    .to_colourspace(sharp::Interpretation::Cmyk)
    .jpeg(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    let (_, info) = Sharp::new_from_buffer(cmyk)
        .unwrap()
        .trim(None)
        .unwrap()
        .raw(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(info.width, 16);
    assert_eq!(info.height, 8);
    assert_eq!(info.trim_offset_top, 0);
    assert_eq!(info.trim_offset_left, -12);

    //Ensure trim of image with all pixels same is no-op
    let (_, info) = Sharp::new(Inputs::new().create(Create {
        width: 5,
        height: 5,
        channels: 3,
        background: Colour::from_hex(0xff0000),
        ..Default::default()
    }))
    .unwrap()
    .trim(None)
    .unwrap()
    .to_buffer_with_info()
    .unwrap();
    assert_eq!(info.width, 5);
    assert_eq!(info.height, 5);
    assert_eq!(info.trim_offset_top, 0);
    assert_eq!(info.trim_offset_left, 0);

    //Works with line-art
    let (_, info) = Sharp::new_from_file(fixtures::inputJpgOverlayLayer2())
        .unwrap()
        .trim(Some(TrimOptions {
            line_art: Some(true),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(info.trim_offset_top, -552);

    //Specific background colour
    let (_, info) = Sharp::new_from_file(fixtures::inputPngTrimSpecificColour())
        .unwrap()
        .trim(Some(TrimOptions {
            background: Some(Colour::from_hex(0xffff00)),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(info.width, 900);
    assert_eq!(info.height, 600);
    assert_eq!(info.trim_offset_top, 0);
    assert_eq!(info.trim_offset_left, 0);

    //Only trims the bottom
    let (_, info) = Sharp::new_from_file(fixtures::inputPngTrimSpecificColour())
        .unwrap()
        .trim(Some(TrimOptions {
            background: Some(Colour::from_hex(0x21468B)),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(info.width, 900);
    assert_eq!(info.height, 401);
    assert_eq!(info.trim_offset_top, 0);
    assert_eq!(info.trim_offset_left, 0);

    //Only trims the bottom, in 16-bit
    let (_, info) = Sharp::new_from_file(fixtures::inputPngTrimSpecificColour16bit())
        .unwrap()
        .trim(Some(TrimOptions {
            background: Some(Colour::from_hex(0x21468B)),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(info.width, 900);
    assert_eq!(info.height, 401);
    assert_eq!(info.trim_offset_top, 0);
    assert_eq!(info.trim_offset_left, 0);

    //Only trims the bottom, including alpha
    let (_, info) = Sharp::new_from_file(fixtures::inputPngTrimSpecificColourIncludeAlpha())
        .unwrap()
        .trim(Some(TrimOptions {
            background: Some(Colour::from_hex(0x21468B80)),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(info.width, 900);
    assert_eq!(info.height, 401);
    assert_eq!(info.trim_offset_top, 0);
    assert_eq!(info.trim_offset_left, 0);

    rs_vips::Vips::shutdown();
}
