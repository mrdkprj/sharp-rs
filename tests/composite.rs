mod fixtures;
use sharp::{
    composite::{CompositeInput, OverlayOptions},
    input::{Create, SharpOptions},
    resize::Gravity,
    BlendMode, Colour, Interpretation, Sharp,
};

#[test]
pub fn composite() {
    [BlendMode::Over, BlendMode::Xor, BlendMode::Saturate, BlendMode::DestOver].iter().for_each(
        |b| {
            Sharp::new(SharpOptions {
                create: Some(Create {
                    width: 80,
                    height: 60,
                    channels: 4,
                    background: Colour::new(255, 0, 0, 0.5),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .unwrap()
            .composite(&[OverlayOptions {
                input: CompositeInput::Create(Create {
                    width: 60,
                    height: 40,
                    channels: 4,
                    background: Colour::new(0, 0, 255, 0.5),
                    ..Default::default()
                }),
                blend: Some(*b),
                ..Default::default()
            }])
            .unwrap()
            .to_buffer()
            .unwrap();
        },
    );

    //premultiplied true'
    Sharp::new_from_file(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("img")
            .join("input.below.composite.premultiplied.png"),
    )
    .unwrap()
    .composite(&[OverlayOptions {
        input: CompositeInput::Path(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("img")
                .join("input.above.composite.premultiplied.png")
                .to_string_lossy()
                .to_string(),
        ),
        blend: Some(BlendMode::ColourBurn),
        top: Some(0),
        left: Some(0),
        premultiplied: Some(true),
        ..Default::default()
    }])
    .unwrap()
    .to_buffer()
    .unwrap();

    //'premultiplied false
    Sharp::new_from_file(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("img")
            .join("input.below.composite.premultiplied.png"),
    )
    .unwrap()
    .composite(&[OverlayOptions {
        input: CompositeInput::Path(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("img")
                .join("input.above.composite.premultiplied.png")
                .to_string_lossy()
                .to_string(),
        ),
        blend: Some(BlendMode::ColourBurn),
        top: Some(0),
        left: Some(0),
        premultiplied: Some(false),
        ..Default::default()
    }])
    .unwrap()
    .to_buffer()
    .unwrap();

    //premultiplied absent
    Sharp::new_from_file(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("img")
            .join("input.below.composite.premultiplied.png"),
    )
    .unwrap()
    .composite(&[OverlayOptions {
        input: CompositeInput::Path(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("img")
                .join("input.above.composite.premultiplied.png")
                .to_string_lossy()
                .to_string(),
        ),
        blend: Some(BlendMode::ColourBurn),
        top: Some(0),
        left: Some(0),
        ..Default::default()
    }])
    .unwrap()
    .to_buffer()
    .unwrap();

    //scrgb pipeline
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 32,
            height: 32,
            channels: 4,
            background: Colour::new(255, 0, 0, 0.5),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .pipeline_colourspace(Interpretation::Scrgb)
    .composite(&[OverlayOptions {
        input: CompositeInput::Path(
            fixtures::inputPngWithTransparency16bit().to_string_lossy().to_string(),
        ),
        blend: Some(BlendMode::ColourBurn),
        ..Default::default()
    }])
    .unwrap()
    .to_buffer()
    .unwrap();

    //multiple
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 80,
            height: 60,
            channels: 4,
            background: Colour::new(255, 0, 0, 0.5),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .pipeline_colourspace(Interpretation::Scrgb)
    .composite(&[
        OverlayOptions {
            input: CompositeInput::Create(Create {
                width: 60,
                height: 40,
                channels: 4,
                background: Colour::new(0, 0, 255, 0.5),
                ..Default::default()
            }),
            gravity: Some(Gravity::Northeast),
            ..Default::default()
        },
        OverlayOptions {
            input: CompositeInput::Create(Create {
                width: 40,
                height: 40,
                channels: 4,
                background: Colour::new(0, 255, 0, 0.5),
                ..Default::default()
            }),
            gravity: Some(Gravity::Southwest),
            ..Default::default()
        },
    ])
    .unwrap()
    .to_buffer()
    .unwrap();

    //autoOrient
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 600,
            height: 600,
            channels: 4,
            background: Colour::new(255, 0, 0, 1.0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .composite(&[OverlayOptions {
        input: CompositeInput::Path(fixtures::inputJpgWithExif().to_string_lossy().to_string()),
        auto_orient: Some(true),
        ..Default::default()
    }])
    .unwrap()
    .to_buffer()
    .unwrap();

    //zero offset
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .composite(&[OverlayOptions {
            input: CompositeInput::Path(
                fixtures::inputPngWithTransparency16bit().to_string_lossy().to_string(),
            ),
            top: Some(0),
            left: Some(0),
            ..Default::default()
        }])
        .unwrap()
        .to_buffer()
        .unwrap();

    //offset and gravity
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(80, 80)
        .unwrap()
        .composite(&[OverlayOptions {
            input: CompositeInput::Path(
                fixtures::inputPngWithTransparency16bit().to_string_lossy().to_string(),
            ),
            top: Some(0),
            left: Some(0),
            gravity: Some(Gravity::West),
            ..Default::default()
        }])
        .unwrap()
        .to_buffer()
        .unwrap();

    //negative offset and gravity
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(400, 400)
        .unwrap()
        .composite(&[OverlayOptions {
            input: CompositeInput::Path(
                fixtures::inputPngWithTransparency16bit().to_string_lossy().to_string(),
            ),
            top: Some(-10),
            left: Some(-10),
            gravity: Some(Gravity::West),
            ..Default::default()
        }])
        .unwrap()
        .to_buffer()
        .unwrap();

    //offset, gravity and tile'
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(400, 400)
        .unwrap()
        .composite(&[OverlayOptions {
            input: CompositeInput::Path(
                fixtures::inputPngWithTransparency16bit().to_string_lossy().to_string(),
            ),
            top: Some(10),
            left: Some(10),
            gravity: Some(Gravity::West),
            tile: Some(true),
            ..Default::default()
        }])
        .unwrap()
        .to_buffer()
        .unwrap();

    //offset and tile
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(400, 400)
        .unwrap()
        .composite(&[OverlayOptions {
            input: CompositeInput::Path(
                fixtures::inputPngWithTransparency16bit().to_string_lossy().to_string(),
            ),
            top: Some(10),
            left: Some(10),
            tile: Some(true),
            ..Default::default()
        }])
        .unwrap()
        .to_buffer()
        .unwrap();

    //centre gravity should replicate correct number of tiles
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 40,
            height: 40,
            channels: 4,
            background: Colour::new(255, 0, 0, 0.5),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .composite(&[OverlayOptions {
        input: CompositeInput::Path(
            fixtures::inputPngWithTransparency16bit().to_string_lossy().to_string(),
        ),
        gravity: Some(Gravity::Centre),
        tile: Some(true),
        ..Default::default()
    }])
    .unwrap()
    .raw(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    //tile and gravity
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(80, 80)
        .unwrap()
        .composite(&[OverlayOptions {
            input: CompositeInput::Path(
                fixtures::inputPngWithTransparency16bit().to_string_lossy().to_string(),
            ),
            gravity: Some(Gravity::North),
            tile: Some(true),
            ..Default::default()
        }])
        .unwrap()
        .to_buffer()
        .unwrap();
}
