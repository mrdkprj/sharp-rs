mod fixtures;
use sharp::{
    composite::OverlayOptions,
    input::{Create, Input, Inputs},
    resize::{Gravity, ResizeOptions},
    BlendMode, Colour, Interpretation, Sharp,
};

#[test]
pub fn composite() {
    [BlendMode::Over, BlendMode::Xor, BlendMode::Saturate, BlendMode::DestOver].iter().for_each(
        |b| {
            let filename = format!("composite.blend.{}.png", blend_to_string(*b));
            let expected = fixtures::expected(&filename);
            let actual = fixtures::output(&format!("ouput.{}", filename));
            Sharp::new(Inputs::new().create(Create {
                width: 80,
                height: 60,
                channels: 4,
                background: Colour::new(255, 0, 0, 0.5),
                ..Default::default()
            }))
            .unwrap()
            .composite(&[OverlayOptions {
                input: Input::create(Create {
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
            .to_file(actual.clone())
            .unwrap();
            assert_max_colour_distance!(actual, expected, 1.0);
        },
    );

    //premultiplied true'
    let filename = "composite.premultiplied.png";
    let below = fixtures::path(&format!("input.below.{}", filename));
    let above = fixtures::path(&format!("input.above.{}", filename));
    let actual = fixtures::output(&format!("input.true.{}", filename));
    let expected = fixtures::expected(&format!("expected.true.{}", filename));
    Sharp::new_from_file(below)
        .unwrap()
        .composite(&[OverlayOptions {
            input: Input::path(above),
            blend: Some(BlendMode::ColourBurn),
            top: Some(0),
            left: Some(0),
            premultiplied: Some(true),
            ..Default::default()
        }])
        .unwrap()
        .to_file(actual.clone())
        .unwrap();
    assert_max_colour_distance!(actual, expected, 1.0);

    //'premultiplied false
    let filename = "composite.premultiplied.png";
    let below = fixtures::path(&format!("input.below.{}", filename));
    let above = fixtures::path(&format!("input.above.{}", filename));
    let actual = fixtures::output(&format!("input.false.{}", filename));
    let expected = fixtures::expected(&format!("expected.false.{}", filename));
    Sharp::new_from_file(below)
        .unwrap()
        .composite(&[OverlayOptions {
            input: Input::path(above),
            blend: Some(BlendMode::ColourBurn),
            top: Some(0),
            left: Some(0),
            premultiplied: Some(false),
            ..Default::default()
        }])
        .unwrap()
        .to_file(actual.clone())
        .unwrap();
    assert_max_colour_distance!(actual, expected, 1.0);

    //premultiplied absent
    let filename = "composite.premultiplied.png";
    let below = fixtures::path(&format!("input.below.{}", filename));
    let above = fixtures::path(&format!("input.above.{}", filename));
    let actual = fixtures::output(&format!("input.absent.{}", filename));
    let expected = fixtures::expected(&format!("expected.absent.{}", filename));
    Sharp::new_from_file(below)
        .unwrap()
        .composite(&[OverlayOptions {
            input: Input::path(above),
            blend: Some(BlendMode::ColourBurn),
            top: Some(0),
            left: Some(0),
            ..Default::default()
        }])
        .unwrap()
        .to_file(actual.clone())
        .unwrap();
    assert_max_colour_distance!(actual, expected, 1.0);

    //scrgb pipeline
    let filename = "composite-red-scrgb.png";
    let actual = fixtures::output(&format!("output.{}", filename));
    let expected = fixtures::expected(filename);
    Sharp::new(Inputs::new().create(Create {
        width: 32,
        height: 32,
        channels: 4,
        background: Colour::new(255, 0, 0, 0.5),
        ..Default::default()
    }))
    .unwrap()
    .pipeline_colourspace(Interpretation::Scrgb)
    .composite(&[OverlayOptions {
        input: Input::path(fixtures::inputPngWithTransparency16bit()),
        blend: Some(BlendMode::ColourBurn),
        ..Default::default()
    }])
    .unwrap()
    .to_file(actual.clone())
    .unwrap();
    assert_max_colour_distance!(actual, expected, 1.0);

    //multiple
    let filename = "composite-multiple.png";
    let actual = fixtures::output(&format!("output.{}", filename));
    let expected = fixtures::expected(filename);
    Sharp::new(Inputs::new().create(Create {
        width: 80,
        height: 60,
        channels: 4,
        background: Colour::new(255, 0, 0, 0.5),
        ..Default::default()
    }))
    .unwrap()
    .pipeline_colourspace(Interpretation::Scrgb)
    .composite(&[
        OverlayOptions {
            input: Input::create(Create {
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
            input: Input::create(Create {
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
    .to_file(actual.clone())
    .unwrap();
    assert_max_colour_distance!(actual, expected, 1.0);

    //autoOrient
    let data = Sharp::new(Inputs::new().create(Create {
        width: 600,
        height: 600,
        channels: 4,
        background: Colour::new(255, 0, 0, 1.0),
        ..Default::default()
    }))
    .unwrap()
    .composite(&[OverlayOptions {
        input: Input::path(fixtures::inputJpgWithExif()),
        auto_orient: Some(true),
        ..Default::default()
    }])
    .unwrap()
    .jpeg(None)
    .unwrap()
    .to_buffer()
    .unwrap();
    assert_similar!(fixtures::expected("composite-autoOrient.jpg"), data, None);

    //zero offset
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .composite(&[OverlayOptions {
            input: Input::path(fixtures::inputPngWithTransparency16bit()),
            top: Some(0),
            left: Some(0),
            ..Default::default()
        }])
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("overlay-offset-0.jpg"), data, None);

    //offset and gravity
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(80),
            ..Default::default()
        })
        .unwrap()
        .composite(&[OverlayOptions {
            input: Input::path(fixtures::inputPngWithTransparency16bit()),
            top: Some(10),
            left: Some(10),
            gravity: Some(Gravity::West),
            ..Default::default()
        }])
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("overlay-offset-with-gravity.jpg"), data, None);

    //negative offset and gravity
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(400),
            ..Default::default()
        })
        .unwrap()
        .composite(&[OverlayOptions {
            input: Input::path(fixtures::inputPngWithTransparency16bit()),
            top: Some(-10),
            left: Some(-10),
            gravity: Some(Gravity::West),
            ..Default::default()
        }])
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("overlay-negative-offset-with-gravity.jpg"), data, None);

    //offset, gravity and tile'
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(80),
            ..Default::default()
        })
        .unwrap()
        .composite(&[OverlayOptions {
            input: Input::path(fixtures::inputPngWithTransparency16bit()),
            top: Some(10),
            left: Some(10),
            gravity: Some(Gravity::West),
            tile: Some(true),
            ..Default::default()
        }])
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("overlay-offset-with-gravity-tile.jpg"), data, None);

    //offset and tile
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(80),
            ..Default::default()
        })
        .unwrap()
        .composite(&[OverlayOptions {
            input: Input::path(fixtures::inputPngWithTransparency16bit()),
            top: Some(10),
            left: Some(10),
            tile: Some(true),
            ..Default::default()
        }])
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("overlay-offset-with-tile.jpg"), data, None);

    //centre gravity should replicate correct number of tiles
    let buf = Sharp::new(Inputs::new().create(Create {
        width: 40,
        height: 40,
        channels: 4,
        background: Colour::new(255, 0, 0, 1.0),
        ..Default::default()
    }))
    .unwrap()
    .composite(&[OverlayOptions {
        input: Input::path(fixtures::inputPngWithTransparency16bit()),
        gravity: Some(Gravity::Centre),
        tile: Some(true),
        ..Default::default()
    }])
    .unwrap()
    .raw(None)
    .unwrap()
    .to_buffer()
    .unwrap();
    assert_eq!(buf[0..3], vec![255, 0, 0]);

    [
        Gravity::Centre,
        Gravity::East,
        Gravity::North,
        Gravity::Northeast,
        Gravity::Northwest,
        Gravity::South,
        Gravity::Southeast,
        Gravity::Southwest,
        Gravity::West,
    ]
    .iter()
    .for_each(|g| {
        //gravity
        let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
            .unwrap()
            .resize_with_opts(ResizeOptions {
                width: Some(80),
                ..Default::default()
            })
            .unwrap()
            .composite(&[OverlayOptions {
                input: Input::path(fixtures::inputPngWithTransparency16bit()),
                gravity: Some(g.clone()),
                ..Default::default()
            }])
            .unwrap()
            .to_buffer_with_info()
            .unwrap();
        assert_eq!("jpeg".to_string(), info.format);
        assert_eq!(80, info.width);
        assert_eq!(65, info.height);
        assert_eq!(3, info.channels);
        assert_similar!(
            fixtures::expected(&format!("overlay-gravity-{}.jpg", gravity_to_string(g))),
            data,
            None
        );

        //tile and gravity
        let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
            .unwrap()
            .resize_with_opts(ResizeOptions {
                width: Some(80),
                ..Default::default()
            })
            .unwrap()
            .composite(&[OverlayOptions {
                input: Input::path(fixtures::inputPngWithTransparency16bit()),
                gravity: Some(g.clone()),
                tile: Some(true),
                ..Default::default()
            }])
            .unwrap()
            .to_buffer_with_info()
            .unwrap();
        assert_eq!("jpeg".to_string(), info.format);
        assert_eq!(80, info.width);
        assert_eq!(65, info.height);
        assert_eq!(3, info.channels);
        assert_similar!(
            fixtures::expected(&format!("overlay-tile-gravity-{}.jpg", gravity_to_string(g))),
            data,
            None
        );
    });

    fixtures::clean_up();
    rs_vips::Vips::shutdown();
}

fn blend_to_string(blend: BlendMode) -> String {
    match blend {
        BlendMode::Over => "over",
        BlendMode::Xor => "xor",
        BlendMode::Saturate => "saturate",
        BlendMode::DestOver => "dest-over",
        _ => "",
    }
    .to_string()
}

fn gravity_to_string(gravity: &Gravity) -> String {
    match gravity {
        Gravity::Centre => "centre",
        Gravity::East => "east",
        Gravity::North => "north",
        Gravity::Northeast => "northeast",
        Gravity::Northwest => "northwest",
        Gravity::South => "south",
        Gravity::Southeast => "southeast",
        Gravity::Southwest => "southwest",
        Gravity::West => "west",
    }
    .to_string()
}
