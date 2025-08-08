mod fixtures;
use sharp::{
    input::{Create, HorizontalAlignment, Join, SharpOptions, VerticalAlignment},
    Colour, Sharp,
};

#[test]
pub fn join() {
    //Join two images horizontally
    let buf = std::fs::read(fixtures::inputPngPalette()).unwrap();
    let buf2 = Sharp::new(SharpOptions {
        create: Some(Create {
            width: 68,
            height: 68,
            channels: 3,
            background: Colour::new(0, 255, 0, 1.0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .png(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    let data = Sharp::new_from_buffers_with_opts(
        vec![buf.clone(), buf2],
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

    let meat = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    println!("{:?}", meat);

    //Join two images vertically with shim and alpha channel
    let buf2 = Sharp::new(SharpOptions {
        create: Some(Create {
            width: 68,
            height: 68,
            channels: 4,
            background: Colour::new(0, 255, 0, 1.0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .png(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    Sharp::new_from_buffers_with_opts(
        vec![buf.clone(), buf2],
        SharpOptions {
            join: Some(Join {
                across: Some(1),
                // shim: Some(8),
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .unwrap()
    .to_buffer()
    .unwrap();

    //Join four images in 2x2 grid, with centre alignment
    let buf2 = Sharp::new(SharpOptions {
        create: Some(Create {
            width: 128,
            height: 128,
            channels: 3,
            background: Colour::new(0, 255, 0, 1.0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .png(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    let buf3 = Sharp::new(SharpOptions {
        create: Some(Create {
            width: 128,
            height: 128,
            channels: 3,
            background: Colour::new(255, 0, 0, 1.0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .png(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    let buf4 = std::fs::read(fixtures::inputPngPalette()).unwrap();

    Sharp::new_from_buffers_with_opts(
        vec![buf.clone(), buf2, buf3, buf4.clone()],
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
    .to_buffer()
    .unwrap();

    //Join two images as animation
    let buf2 = Sharp::new(SharpOptions {
        create: Some(Create {
            width: 68,
            height: 68,
            channels: 3,
            background: Colour::new(0, 255, 0, 1.0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .png(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    Sharp::new_from_buffers_with_opts(
        vec![buf.clone(), buf2],
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
}
