mod fixtures;
use sharp::{
    composite::OverlayOptions,
    input::{Create, Input, Inputs, SharpOptions},
    output::PngOptions,
    resize::{ExtendOptions, ResizeOptions},
    Colour, Extend, Sharp,
};

#[test]
pub fn extend() {
    //extend all sides equally via a single value
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(120),
            ..Default::default()
        })
        .unwrap()
        .extend(ExtendOptions {
            top: Some(10),
            left: Some(10),
            bottom: Some(10),
            right: Some(10),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(140, info.width);
    assert_eq!(118, info.height);
    assert_similar!(fixtures::expected("extend-equal-single.jpg"), data, None);

    //extend all sides equally via a single value, Animated WebP
    let (data, info) = Sharp::new_from_file_with_opts(
        fixtures::inputWebPAnimated(),
        SharpOptions {
            pages: Some(-1),
            ..Default::default()
        },
    )
    .unwrap()
    .resize_with_opts(ResizeOptions {
        width: Some(120),
        ..Default::default()
    })
    .unwrap()
    .extend(ExtendOptions {
        top: Some(10),
        left: Some(10),
        bottom: Some(10),
        right: Some(10),
        ..Default::default()
    })
    .unwrap()
    .to_buffer_with_info()
    .unwrap();
    assert_eq!(140, info.width);
    assert_eq!(140 * 9, info.height);
    assert_similar!(fixtures::expected("extend-equal-single.webp"), data, None);

    [Extend::Background, Extend::Copy, Extend::Mirror, Extend::Repeat].iter().for_each(|e| {
        //extends all sides with animated WebP
        let (data, info) = Sharp::new_from_file_with_opts(
            fixtures::inputWebPAnimated(),
            SharpOptions {
                pages: Some(-1),
                ..Default::default()
            },
        )
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(120),
            ..Default::default()
        })
        .unwrap()
        .extend(ExtendOptions {
            top: Some(40),
            left: Some(40),
            bottom: Some(40),
            right: Some(40),
            extend_with: Some(*e),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
        assert_eq!(200, info.width);
        assert_eq!(200 * 9, info.height);
        assert_similar!(
            fixtures::expected(&format!("extend-equal-{}.webp", extend_to_string(*e))),
            data,
            None
        );

        //extend all sides equally with RGB
        let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
            .unwrap()
            .resize_with_opts(ResizeOptions {
                width: Some(120),
                ..Default::default()
            })
            .unwrap()
            .extend(ExtendOptions {
                top: Some(10),
                left: Some(10),
                bottom: Some(10),
                right: Some(10),
                extend_with: Some(*e),
                background: Some(Colour::new(255, 0, 0, 1.0)),
            })
            .unwrap()
            .to_buffer_with_info()
            .unwrap();
        assert_eq!(140, info.width);
        assert_eq!(118, info.height);
        assert_similar!(
            fixtures::expected(&format!("extend-equal-{}.jpg", extend_to_string(*e))),
            data,
            None
        );

        //extend sides unequally with RGBA
        let (data, info) = Sharp::new_from_file(fixtures::inputPngWithTransparency16bit())
            .unwrap()
            .resize_with_opts(ResizeOptions {
                width: Some(120),
                ..Default::default()
            })
            .unwrap()
            .extend(ExtendOptions {
                top: Some(50),
                left: Some(10),
                right: Some(35),
                extend_with: Some(*e),
                background: Some(Colour::new(0, 0, 0, 0.0)),
                ..Default::default()
            })
            .unwrap()
            .to_buffer_with_info()
            .unwrap();
        assert_eq!(165, info.width);
        assert_eq!(170, info.height);
        assert_similar!(
            fixtures::expected(&format!("extend-unequal-{}.png", extend_to_string(*e))),
            data,
            None
        );

        //PNG with 2 channels
        let (data, info) = Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
            .unwrap()
            .extend(ExtendOptions {
                top: Some(50),
                left: Some(80),
                right: Some(80),
                bottom: Some(50),
                extend_with: Some(*e),
                background: Some(Colour::new(0, 0, 0, 0.0)),
            })
            .unwrap()
            .to_buffer_with_info()
            .unwrap();
        assert!(!data.is_empty());
        assert_eq!(560, info.width);
        assert_eq!(400, info.height);
        assert_eq!(4, info.channels);
        assert_similar!(
            fixtures::expected(&format!("extend-2channel-{}.png", extend_to_string(*e))),
            data,
            None
        );
    });

    //extend top with mirroring uses ordered read
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .extend(ExtendOptions {
            top: Some(1),
            extend_with: Some(Extend::Mirror),
            ..Default::default()
        })
        .unwrap()
        .png(Some(PngOptions {
            compression_level: Some(0),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2226, info.height);

    //multi-page extend uses ordered read
    let multi_page_tiff = Sharp::new_from_file_with_opts(
        fixtures::inputGifAnimated(),
        SharpOptions {
            animated: Some(true),
            ..Default::default()
        },
    )
    .unwrap()
    .resize(8, 48)
    .unwrap()
    .tiff(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    let (_, info) = Sharp::new_from_buffer_with_opts(
        multi_page_tiff,
        SharpOptions {
            pages: Some(-1),
            ..Default::default()
        },
    )
    .unwrap()
    .extend(ExtendOptions {
        background: Some(Colour::new(255, 0, 0, 1.0)),
        top: Some(1),
        ..Default::default()
    })
    .unwrap()
    .png(Some(PngOptions {
        compression_level: Some(0),
        ..Default::default()
    }))
    .unwrap()
    .to_buffer_with_info()
    .unwrap();
    assert_eq!(8, info.width);
    assert_eq!(1470, info.height);

    //should add alpha channel before extending with a transparent Background
    let (data, info) = Sharp::new_from_file(fixtures::inputJpgWithLandscapeExif1())
        .unwrap()
        .extend(ExtendOptions {
            bottom: Some(10),
            right: Some(10),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            ..Default::default()
        })
        .unwrap()
        .to_format(sharp::output::FormatEnum::Png, None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(610, info.width);
    assert_eq!(460, info.height);
    assert_similar!(fixtures::expected("addAlphaChanelBeforeExtend.png"), data, None);

    //Premultiply background when compositing
    let buf = Sharp::new(Inputs::new().create(Create {
        width: 1,
        height: 1,
        channels: 4,
        background: Colour::new(255, 255, 255, 0.0),
        ..Default::default()
    }))
    .unwrap()
    .composite(&[OverlayOptions {
        input: Input::create(Create {
            width: 1,
            height: 1,
            channels: 4,
            background: Colour::new(191, 25, 66, 0.8),
            ..Default::default()
        }),
        ..Default::default()
    }])
    .unwrap()
    .extend(ExtendOptions {
        left: Some(1),
        background: Some(Colour::new(191, 25, 66, 0.8)),
        ..Default::default()
    })
    .unwrap()
    .raw(None)
    .unwrap()
    .to_buffer()
    .unwrap();
    assert_eq!(buf, vec![191, 25, 66, 204, 191, 25, 66, 204]);

    rs_vips::Vips::shutdown();
}

fn extend_to_string(extend: Extend) -> String {
    match extend {
        Extend::Background => "background",
        Extend::Copy => "copy",
        Extend::Mirror => "mirror",
        Extend::Repeat => "repeat",
        _ => "",
    }
    .to_string()
}
