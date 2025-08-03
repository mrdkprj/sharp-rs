mod fixtures;
use sharp::{
    composite::{CompositeInput, OverlayOptions},
    input::{Create, SharpOptions},
    output::PngOptions,
    resize::ExtendOptions,
    Colour, Extend, Sharp,
};

#[test]
pub fn extend() {
    //extend all sides equally via a single value
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(120, 120)
        .unwrap()
        .extend(ExtendOptions {
            top: Some(10),
            left: Some(10),
            bottom: Some(10),
            right: Some(10),
            ..Default::default()
        })
        .unwrap()
        .to_buffer()
        .unwrap();

    //extend all sides equally via a single value, Animated WebP
    Sharp::new_from_file_with_opts(
        fixtures::inputWebPAnimated(),
        SharpOptions {
            pages: Some(-1),
            ..Default::default()
        },
    )
    .unwrap()
    .resize(120, 120)
    .unwrap()
    .extend(ExtendOptions {
        top: Some(10),
        left: Some(10),
        bottom: Some(10),
        right: Some(10),
        ..Default::default()
    })
    .unwrap()
    .to_buffer()
    .unwrap();

    [Extend::Background, Extend::Copy, Extend::Mirror, Extend::Repeat].iter().for_each(|e| {
        //extends all sides with animated WebP
        Sharp::new_from_file_with_opts(
            fixtures::inputWebPAnimated(),
            SharpOptions {
                pages: Some(-1),
                ..Default::default()
            },
        )
        .unwrap()
        .resize(120, 120)
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
        .to_buffer()
        .unwrap();

        //extend all sides equally with RGB
        Sharp::new_from_file(fixtures::inputJpg())
            .unwrap()
            .resize(120, 120)
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
            .to_buffer()
            .unwrap();

        //extend sides unequally with RGBA
        Sharp::new_from_file(fixtures::inputPngWithTransparency16bit())
            .unwrap()
            .resize(120, 120)
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
            .to_buffer()
            .unwrap();

        //PNG with 2 channels
        Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
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
            .to_buffer()
            .unwrap();
    });

    //extend top with mirroring uses ordered read
    Sharp::new_from_file(fixtures::inputJpg())
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
        .to_buffer()
        .unwrap();

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

    Sharp::new_from_buffer_with_opts(
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
    .to_buffer()
    .unwrap();

    //should add alpha channel before extending with a transparent Background
    Sharp::new_from_file(fixtures::inputJpgWithLandscapeExif1())
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
        .to_buffer()
        .unwrap();

    //Premultiply background when compositing
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 1,
            height: 1,
            channels: 4,
            background: Colour::from_hex(0xfff0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .composite(&[OverlayOptions {
        input: CompositeInput::Create(Create {
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
}
