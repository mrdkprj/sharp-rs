use sharp::{
    input::SharpOptions,
    resize::{Fit, Position, ResizeOptions},
    Sharp,
};
mod fixtures;

struct X {
    width: i32,
    height: i32,
    gravity: Position,
    fixture: &'static str,
}

#[test]
fn resize_cover() {
    [
        X {
            width: 320,
            height: 80,
            gravity: Position::Top,
            fixture: "gravity-north.jpg",
        },
        X {
            width: 80,
            height: 320,
            gravity: Position::Right,
            fixture: "gravity-east.jpg",
        },
        X {
            width: 320,
            height: 80,
            gravity: Position::Bottom,
            fixture: "gravity-south.jpg",
        },
        X {
            width: 80,
            height: 320,
            gravity: Position::Left,
            fixture: "gravity-west.jpg",
        },
        X {
            width: 320,
            height: 80,
            gravity: Position::RightTop,
            fixture: "gravity-north.jpg",
        },
        X {
            width: 80,
            height: 320,
            gravity: Position::RightTop,
            fixture: "gravity-east.jpg",
        },
        X {
            width: 320,
            height: 80,
            gravity: Position::RightBottom,
            fixture: "gravity-south.jpg",
        },
        X {
            width: 80,
            height: 320,
            gravity: Position::RightBottom,
            fixture: "gravity-east.jpg",
        },
        X {
            width: 320,
            height: 80,
            gravity: Position::LeftBottom,
            fixture: "gravity-south.jpg",
        },
        X {
            width: 80,
            height: 320,
            gravity: Position::LeftBottom,
            fixture: "gravity-west.jpg",
        },
        X {
            width: 320,
            height: 80,
            gravity: Position::LeftTop,
            fixture: "gravity-north.jpg",
        },
        X {
            width: 80,
            height: 320,
            gravity: Position::LeftTop,
            fixture: "gravity-west.jpg",
        },
    ]
    .iter()
    .for_each(|x| {
        let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
            .unwrap()
            .resize_with_opts(ResizeOptions {
                width: Some(x.width),
                height: Some(x.height),
                fit: Some(Fit::Cover),
                position: Some(x.gravity.clone()),
                ..Default::default()
            })
            .unwrap()
            .to_buffer_with_info()
            .unwrap();
        assert_eq!(x.width, info.width);
        assert_eq!(x.height, info.height);
        assert_similar!(fixtures::expected(x.fixture), data, None);
    });

    //Skip crop when post-resize dimensions are at target
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(1600, 1200)
        .unwrap()
        .to_buffer()
        .unwrap();
    let (_, info) = Sharp::new_from_buffer(data)
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(1110),
            fit: Some(Fit::Cover),
            position: Some(Position::Attention),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(1110, info.width);
    assert_eq!(832, info.height);
    assert_eq!(0, info.crop_offset_left);
    assert_eq!(0, info.crop_offset_top);

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
        width: Some(80),
        height: Some(320),
        fit: Some(Fit::Cover),

        ..Default::default()
    })
    .unwrap()
    .to_buffer_with_info()
    .unwrap();
    assert_eq!(80, info.width);
    assert_eq!(320 * 9, info.height);
    assert_similar!(fixtures::expected("gravity-center-width.webp"), data, None);

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
        width: Some(320),
        height: Some(80),
        fit: Some(Fit::Cover),

        ..Default::default()
    })
    .unwrap()
    .to_buffer_with_info()
    .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(80 * 9, info.height);
    assert_similar!(fixtures::expected("gravity-center-height.webp"), data, None);

    //Entropy-based strategy
    //JPEG
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(80),
            height: Some(320),
            fit: Some(Fit::Cover),
            position: Some(Position::Entropy),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(3, info.channels);
    assert_eq!(80, info.width);
    assert_eq!(320, info.height);
    assert_eq!(-117, info.crop_offset_left);
    assert_eq!(0, info.crop_offset_top);
    assert_similar!(fixtures::expected("crop-strategy-entropy.jpg"), data, None);

    //PNG
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(80),
            fit: Some(Fit::Cover),
            position: Some(Position::Entropy),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(4, info.channels);
    assert_eq!(320, info.width);
    assert_eq!(80, info.height);
    assert_eq!(0, info.crop_offset_left);
    assert_eq!(-80, info.crop_offset_top);
    assert_similar!(fixtures::expected("crop-strategy.png"), data, None);

    //Attention strategy
    //JPEG
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(80),
            height: Some(320),
            fit: Some(Fit::Cover),
            position: Some(Position::Attention),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(3, info.channels);
    assert_eq!(80, info.width);
    assert_eq!(320, info.height);
    assert_eq!(-107, info.crop_offset_left);
    assert_eq!(0, info.crop_offset_top);
    assert_eq!(588, info.attention_x);
    assert_eq!(640, info.attention_y);
    assert_similar!(fixtures::expected("crop-strategy-attention.jpg"), data, None);

    //PNG
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(80),
            fit: Some(Fit::Cover),
            position: Some(Position::Attention),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(4, info.channels);
    assert_eq!(320, info.width);
    assert_eq!(80, info.height);
    assert_eq!(0, info.crop_offset_left);
    assert_eq!(0, info.crop_offset_top);
    assert_eq!(0, info.attention_x);
    assert_eq!(0, info.attention_y);
    assert_similar!(fixtures::expected("crop-strategy.png"), data, None);

    //Webp
    let (data, info) = Sharp::new_from_file(fixtures::inputWebP())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(80),
            fit: Some(Fit::Cover),
            position: Some(Position::Attention),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("webp", info.format);
    assert_eq!(3, info.channels);
    assert_eq!(320, info.width);
    assert_eq!(80, info.height);
    assert_eq!(0, info.crop_offset_left);
    assert_eq!(-161, info.crop_offset_top);
    assert_eq!(288, info.attention_x);
    assert_eq!(745, info.attention_y);
    assert_similar!(fixtures::expected("crop-strategy.webp"), data, None);
}
