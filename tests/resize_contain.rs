use sharp::{
    input::{Create, Input, Inputs, SharpOptions},
    output::TiffOptions,
    resize::{Fit, Position, ResizeOptions},
    Colour, ForeignTiffCompression, Sharp,
};
mod fixtures;

#[test]
fn resize_contain() {
    //JPEG within PNG, no alpha channel
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(240),
            fit: Some(Fit::Contain),
            ..Default::default()
        })
        .unwrap()
        .png(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_eq!(3, info.channels);
    assert_similar!(fixtures::expected("embed-3-into-3.png"), data, None);

    //JPEG within WebP, to include alpha channel
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(240),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            ..Default::default()
        })
        .unwrap()
        .webp(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("webp", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("embed-3-into-4.webp"), data, None);

    //PNG with alpha channel
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(50),
            height: Some(50),
            fit: Some(Fit::Contain),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(50, info.width);
    assert_eq!(50, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("embed-4-into-4.png"), data, None);

    //16-bit PNG with alpha channel
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithTransparency16bit())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(32),
            height: Some(16),
            fit: Some(Fit::Contain),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(32, info.width);
    assert_eq!(16, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("embed-16bit.png"), data, None);

    //16-bit PNG with alpha channel onto RGBA
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithTransparency16bit())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(32),
            height: Some(16),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(32, info.width);
    assert_eq!(16, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("embed-16bit-rgba.png"), data, None);

    //PNG with 2 channels
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(32),
            height: Some(16),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(32, info.width);
    assert_eq!(16, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("embed-2channel.png"), data, None);

    //TIFF in LAB colourspace onto RGBA background
    let (data, info) = Sharp::new_from_file(fixtures::inputTiffCielab())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(64),
            height: Some(128),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(255, 102, 0, 0.5)),
            ..Default::default()
        })
        .unwrap()
        .png(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(64, info.width);
    assert_eq!(128, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("embed-lab-into-rgba.png"), data, None);

    //Enlarge
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithOneColor())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(240),
            fit: Some(Fit::Contain),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_eq!(3, info.channels);
    assert_similar!(fixtures::expected("embed-enlarge.png"), data, None);

    //Animated WebP, Width only
    let (data, info) = Sharp::new_from_file_with_opts(
        fixtures::inputWebPAnimated(),
        SharpOptions {
            pages: Some(-1),
            ..Default::default()
        },
    )
    .unwrap()
    .resize_with_opts(ResizeOptions {
        width: Some(320),
        height: Some(240),
        fit: Some(Fit::Contain),
        background: Some(Colour::new(255, 0, 0, 1.0)),
        ..Default::default()
    })
    .unwrap()
    .to_buffer_with_info()
    .unwrap();
    assert!(!data.is_empty());
    assert_eq!("webp", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240 * 9, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("embed-animated-width.webp"), data, None);

    //Animated WebP, Height only
    let (data, info) = Sharp::new_from_file_with_opts(
        fixtures::inputWebPAnimated(),
        SharpOptions {
            pages: Some(-1),
            ..Default::default()
        },
    )
    .unwrap()
    .resize_with_opts(ResizeOptions {
        width: Some(240),
        height: Some(320),
        fit: Some(Fit::Contain),
        background: Some(Colour::new(255, 0, 0, 1.0)),
        ..Default::default()
    })
    .unwrap()
    .to_buffer_with_info()
    .unwrap();
    assert!(!data.is_empty());
    assert_eq!("webp", info.format);
    assert_eq!(240, info.width);
    assert_eq!(320 * 9, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("embed-animated-height.webp"), data, None);

    // Position horizontal top
    let (data, info) = Sharp::new_from_file(fixtures::inputPngEmbed())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(200),
            height: Some(100),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            position: Some(Position::Top),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(200, info.width);
    assert_eq!(100, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("./embedgravitybird/a2-n.png"), data, None);

    //Position horizontal right top
    let (data, info) = Sharp::new_from_file(fixtures::inputPngEmbed())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(200),
            height: Some(100),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            position: Some(Position::RightTop),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(200, info.width);
    assert_eq!(100, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("./embedgravitybird/a3-ne.png"), data, None);

    //Position horizontal right
    let (data, info) = Sharp::new_from_file(fixtures::inputPngEmbed())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(200),
            height: Some(100),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            position: Some(Position::Right),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(200, info.width);
    assert_eq!(100, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("./embedgravitybird/a4-e.png"), data, None);

    //Position horizontal right bottom
    let (data, info) = Sharp::new_from_file(fixtures::inputPngEmbed())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(200),
            height: Some(100),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            position: Some(Position::RightBottom),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(200, info.width);
    assert_eq!(100, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("./embedgravitybird/a5-se.png"), data, None);

    //Position horizontal bottom
    let (data, info) = Sharp::new_from_file(fixtures::inputPngEmbed())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(200),
            height: Some(100),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            position: Some(Position::Bottom),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(200, info.width);
    assert_eq!(100, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("./embedgravitybird/a6-s.png"), data, None);

    //Position horizontal left bottom
    let (data, info) = Sharp::new_from_file(fixtures::inputPngEmbed())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(200),
            height: Some(100),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            position: Some(Position::LeftBottom),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(200, info.width);
    assert_eq!(100, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("./embedgravitybird/a7-sw.png"), data, None);

    //Position horizontal left
    let (data, info) = Sharp::new_from_file(fixtures::inputPngEmbed())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(200),
            height: Some(100),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            position: Some(Position::Left),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(200, info.width);
    assert_eq!(100, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("./embedgravitybird/a8-w.png"), data, None);
    //Position horizontal left top
    let (data, info) = Sharp::new_from_file(fixtures::inputPngEmbed())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(200),
            height: Some(100),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            position: Some(Position::LeftTop),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(200, info.width);
    assert_eq!(100, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("./embedgravitybird/a1-nw.png"), data, None);

    //Position vertical top
    let (data, info) = Sharp::new_from_file(fixtures::inputPngEmbed())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(200),
            height: Some(200),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            position: Some(Position::Top),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(200, info.width);
    assert_eq!(200, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("./embedgravitybird/2-n.png"), data, None);

    //Position vertical right top
    let (data, info) = Sharp::new_from_file(fixtures::inputPngEmbed())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(200),
            height: Some(200),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            position: Some(Position::RightTop),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(200, info.width);
    assert_eq!(200, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("./embedgravitybird/3-ne.png"), data, None);

    //Position vertical right
    let (data, info) = Sharp::new_from_file(fixtures::inputPngEmbed())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(200),
            height: Some(200),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            position: Some(Position::Right),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(200, info.width);
    assert_eq!(200, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("./embedgravitybird/4-e.png"), data, None);

    //Position vertical right bottom
    let (data, info) = Sharp::new_from_file(fixtures::inputPngEmbed())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(200),
            height: Some(200),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            position: Some(Position::RightBottom),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(200, info.width);
    assert_eq!(200, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("./embedgravitybird/5-se.png"), data, None);

    //Position vertical bottom
    let (data, info) = Sharp::new_from_file(fixtures::inputPngEmbed())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(200),
            height: Some(200),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            position: Some(Position::Bottom),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(200, info.width);
    assert_eq!(200, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("./embedgravitybird/6-s.png"), data, None);

    //Position vertical left bottom
    let (data, info) = Sharp::new_from_file(fixtures::inputPngEmbed())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(200),
            height: Some(200),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            position: Some(Position::LeftBottom),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(200, info.width);
    assert_eq!(200, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("./embedgravitybird/7-sw.png"), data, None);

    //Position vertical left
    let (data, info) = Sharp::new_from_file(fixtures::inputPngEmbed())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(200),
            height: Some(200),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            position: Some(Position::Left),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(200, info.width);
    assert_eq!(200, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("./embedgravitybird/8-w.png"), data, None);

    //Position vertical left top
    let (data, info) = Sharp::new_from_file(fixtures::inputPngEmbed())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(200),
            height: Some(200),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            position: Some(Position::LeftTop),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!data.is_empty());
    assert_eq!("png", info.format);
    assert_eq!(200, info.width);
    assert_eq!(200, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("./embedgravitybird/1-nw.png"), data, None);

    //multiple alpha channels
    let create = Create {
        width: 20,
        height: 12,
        channels: 4,
        background: Colour::new(0, 255, 0, 1.0),
        ..Default::default()
    };

    let multi = Sharp::new(Inputs::new().create(create.clone()))
        .unwrap()
        .join_channel(&[Input::create(create.clone())], None)
        .unwrap()
        .tiff(Some(TiffOptions {
            compression: Some(ForeignTiffCompression::Deflate),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    let data = Sharp::new_from_buffer(multi)
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(8),
            height: Some(8),
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 255, 1.0)),
            ..Default::default()
        })
        .unwrap()
        .tiff(Some(TiffOptions {
            compression: Some(ForeignTiffCompression::Deflate),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(metadata.format, "tiff");
    assert_eq!(metadata.width, 8);
    assert_eq!(metadata.height, 8);
    assert_eq!(metadata.space, "srgb");
    assert_eq!(metadata.channels, 8);

    rs_vips::Vips::shutdown();
}
