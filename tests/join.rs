mod fixtures;
use sharp::{
    input::{Create, HorizontalAlignment, Inputs, Join, SharpOptions, VerticalAlignment},
    Colour, Sharp,
};

#[test]
pub fn join() {
    //Join two images horizontally
    let buf = Sharp::new_with_opts(
        Inputs::new().path(fixtures::inputPngPalette()).create(Create {
            width: 68,
            height: 68,
            channels: 3,
            background: Colour::new(0, 255, 0, 1.0),
            ..Default::default()
        }),
        SharpOptions {
            join: Some(Join {
                across: Some(2),
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .unwrap()
    .to_buffer()
    .unwrap();

    let metadata = Sharp::new_from_buffer(buf).unwrap().metadata().unwrap();
    assert_eq!(metadata.format, "png");
    assert_eq!(metadata.width, 136);
    assert_eq!(metadata.height, 68);
    assert_eq!(metadata.space, "srgb");
    assert_eq!(metadata.channels, 3);
    assert!(!metadata.has_alpha);

    //Join two images vertically with shim and alpha channel
    let buf = Sharp::new_with_opts(
        Inputs::new().path(fixtures::inputPngPalette()).create(Create {
            width: 68,
            height: 68,
            channels: 4,
            background: Colour::new(0, 255, 0, 1.0),
            ..Default::default()
        }),
        SharpOptions {
            join: Some(Join {
                across: Some(1),
                shim: Some(8),
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .unwrap()
    .to_buffer()
    .unwrap();

    let metadata = Sharp::new_from_buffer(buf).unwrap().metadata().unwrap();
    assert_eq!(metadata.format, "png");
    assert_eq!(metadata.width, 68);
    assert_eq!(metadata.height, 144);
    assert_eq!(metadata.space, "srgb");
    assert_eq!(metadata.channels, 4);
    assert!(metadata.has_alpha);

    //Join four images in 2x2 grid, with centre alignment
    let output = fixtures::output("output.join2x2.png");
    Sharp::new_with_opts(
        Inputs::new()
            .path(fixtures::inputPngPalette())
            .create(Create {
                width: 128,
                height: 128,
                channels: 3,
                background: Colour::new(0, 255, 0, 1.0),
                ..Default::default()
            })
            .create(Create {
                width: 128,
                height: 128,
                channels: 3,
                background: Colour::new(255, 0, 0, 1.0),
                ..Default::default()
            })
            .path(fixtures::inputPngPalette()),
        SharpOptions {
            join: Some(Join {
                across: Some(2),
                halign: Some(HorizontalAlignment::Centre),
                valign: Some(VerticalAlignment::Centre),
                background: Some(Colour::new(0, 0, 255, 1.0)),
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .unwrap()
    .to_file(output.clone())
    .unwrap();
    let info = Sharp::new_from_file(output.clone()).unwrap().metadata().unwrap();
    assert_eq!(info.format, "png");
    assert_eq!(info.width, 256);
    assert_eq!(info.height, 256);
    assert_eq!(info.channels, 3);
    assert_max_colour_distance!(output, fixtures::expected("join2x2.png"), 1.0);

    //Join two images as animation
    let buf = Sharp::new_with_opts(
        Inputs::new().path(fixtures::inputPngPalette()).create(Create {
            width: 68,
            height: 68,
            channels: 3,
            background: Colour::new(0, 255, 0, 1.0),
            ..Default::default()
        }),
        SharpOptions {
            join: Some(Join {
                animated: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .unwrap()
    .gif(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    let metadata = Sharp::new_from_buffer(buf).unwrap().metadata().unwrap();
    assert_eq!(metadata.format, "gif");
    assert_eq!(metadata.width, 68);
    assert_eq!(metadata.height, 68);
    assert_eq!(metadata.pages, 2);

    rs_vips::Vips::shutdown();
}
