use sharp::{
    input::{Create, Noise, SharpOptions},
    Interpretation, Sharp,
};

#[test]
pub fn noise() {
    //generate single-channel gaussian noise
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 1024,
            height: 768,
            channels: 1,
            noise: Some(Noise {
                gaussian: Some(true),
                mean: Some(128.0),
                sigma: Some(30.0),
            }),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .to_colourspace(Interpretation::BW)
    .to_buffer()
    .unwrap();

    //generate 3-channels gaussian noise
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 1024,
            height: 768,
            channels: 3,
            noise: Some(Noise {
                gaussian: Some(true),
                mean: Some(128.0),
                sigma: Some(30.0),
            }),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .to_buffer()
    .unwrap();

    //overlay 3-channels gaussian noise over image'
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 320,
            height: 240,
            channels: 3,
            noise: Some(Noise {
                gaussian: Some(true),
                mean: Some(0.0),
                sigma: Some(5.0),
            }),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .to_buffer()
    .unwrap();

    //animated noise
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 16,
            height: 64,
            page_height: Some(16),
            channels: 3,
            noise: Some(Noise {
                gaussian: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .gif(None)
    .unwrap()
    .to_buffer()
    .unwrap();
}
