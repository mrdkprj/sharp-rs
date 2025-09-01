mod fixtures;
use sharp::{
    composite::OverlayOptions,
    input::{Create, CreateRaw, Input, Inputs, Noise, SharpOptions},
    Interpretation, Sharp,
};

#[test]
pub fn noise() {
    //generate single-channel gaussian noise
    let buf = Sharp::new(Inputs::new().create(Create {
        width: 1024,
        height: 768,
        channels: 1,
        noise: Some(Noise {
            gaussian: Some(true),
            mean: Some(128.0),
            sigma: Some(30.0),
        }),
        ..Default::default()
    }))
    .unwrap()
    .to_colourspace(Interpretation::BW)
    .png(None)
    .unwrap()
    .to_buffer()
    .unwrap();
    let metadata = Sharp::new_from_buffer(buf).unwrap().metadata().unwrap();
    assert_eq!("png", metadata.format);
    assert_eq!(1024, metadata.width);
    assert_eq!(768, metadata.height);
    assert_eq!(1, metadata.channels);
    assert_eq!("b-w", metadata.space);
    assert_eq!("uchar", metadata.depth);

    //generate 3-channels gaussian noise
    let buf = Sharp::new(Inputs::new().create(Create {
        width: 1024,
        height: 768,
        channels: 3,
        noise: Some(Noise {
            gaussian: Some(true),
            mean: Some(128.0),
            sigma: Some(30.0),
        }),
        ..Default::default()
    }))
    .unwrap()
    .png(None)
    .unwrap()
    .to_buffer()
    .unwrap();
    let metadata = Sharp::new_from_buffer(buf).unwrap().metadata().unwrap();
    assert_eq!("png", metadata.format);
    assert_eq!(1024, metadata.width);
    assert_eq!(768, metadata.height);
    assert_eq!(3, metadata.channels);
    assert_eq!("srgb", metadata.space);
    assert_eq!("uchar", metadata.depth);

    //overlay 3-channels gaussian noise over image"
    let (data, info) = Sharp::new(Inputs::new().create(Create {
        width: 320,
        height: 240,
        channels: 3,
        noise: Some(Noise {
            gaussian: Some(true),
            mean: Some(0.0),
            sigma: Some(5.0),
        }),
        ..Default::default()
    }))
    .unwrap()
    .to_buffer_with_info()
    .unwrap();
    let output = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .composite(&[OverlayOptions {
            input: Input::buffer(data),
            blend: Some(sharp::BlendMode::Exclusion),
            raw: Some(CreateRaw {
                width: info.width,
                height: info.height,
                channels: info.channels,
                ..Default::default()
            }),
            ..Default::default()
        }])
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::inputJpg(), output, None);

    //overlay strong single-channel (sRGB) gaussian noise with 25% transparency over transparent png image
    let width = 320;
    let height = 240;
    let raw_data = CreateRaw {
        width,
        height,
        channels: 1,
        ..Default::default()
    };
    let noise = Sharp::new(Inputs::new().create(Create {
        width,
        height,
        channels: 1,
        noise: Some(Noise {
            gaussian: Some(true),
            mean: Some(200.0),
            sigma: Some(30.0),
        }),
        ..Default::default()
    }))
    .unwrap();

    let (data, info1) = noise.to_colourspace(Interpretation::BW).to_buffer_with_info().unwrap();
    assert_eq!(1, info1.channels);

    let buffer = vec![64u8; (width * height) as _];
    let (data2, info2) = Sharp::new_from_buffer_with_opts(
        data.clone(),
        SharpOptions {
            raw: Some(raw_data.clone()),
            ..Default::default()
        },
    )
    .unwrap()
    .join_channel(
        &[Input::buffer(data.clone())],
        Some(SharpOptions {
            raw: Some(raw_data.clone()),
            ..Default::default()
        }),
    )
    .unwrap()
    .join_channel(
        &[Input::buffer(data.clone())],
        Some(SharpOptions {
            raw: Some(raw_data.clone()),
            ..Default::default()
        }),
    )
    .unwrap()
    .join_channel(
        &[Input::buffer(buffer)],
        Some(SharpOptions {
            raw: Some(raw_data.clone()),
            ..Default::default()
        }),
    )
    .unwrap()
    .to_buffer_with_info()
    .unwrap();
    assert_eq!(4, info2.channels);

    let (output, info) = Sharp::new_from_file(fixtures::inputPngRGBWithAlpha())
        .unwrap()
        .resize(width, height)
        .unwrap()
        .composite(&[OverlayOptions {
            input: Input::buffer(data2),
            blend: Some(sharp::BlendMode::Exclusion),
            raw: Some(CreateRaw {
                width: info2.width,
                height: info2.height,
                channels: info2.channels,
                ..Default::default()
            }),
            ..Default::default()
        }])
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(width, info.width);
    assert_eq!(height, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::inputPngRGBWithAlpha(), output, Some(10));

    //animated noise
    let data = Sharp::new(Inputs::new().create(Create {
        width: 16,
        height: 64,
        page_height: Some(16),
        channels: 3,
        noise: Some(Noise {
            gaussian: Some(true),
            ..Default::default()
        }),
        ..Default::default()
    }))
    .unwrap()
    .gif(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(metadata.width, 16);
    assert_eq!(metadata.height, 16);
    assert_eq!(metadata.pages, 4);
    assert_eq!(metadata.delay.len(), 4);

    rs_vips::Vips::shutdown();
}
