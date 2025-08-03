mod fixtures;
use sharp::{
    input::{Create, SharpOptions},
    operation::ModulateOptions,
    output::PngOptions,
    Colour, Sharp,
};

#[test]
pub fn modulate() {
    //should be able to hue-rotate
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 1,
            height: 1,
            channels: 3,
            background: Colour::new(153, 68, 68, 0.0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .modulate(Some(ModulateOptions {
        hue: Some(120),
        ..Default::default()
    }))
    .unwrap()
    .raw(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    //should be able to brighten
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 1,
            height: 1,
            channels: 3,
            background: Colour::new(153, 68, 68, 0.0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .modulate(Some(ModulateOptions {
        brightness: Some(2.0),
        ..Default::default()
    }))
    .unwrap()
    .raw(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    //should be able to darken
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 1,
            height: 1,
            channels: 3,
            background: Colour::new(153, 68, 68, 0.0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .modulate(Some(ModulateOptions {
        brightness: Some(0.5),
        ..Default::default()
    }))
    .unwrap()
    .raw(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    //should be able to saturate
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 1,
            height: 1,
            channels: 3,
            background: Colour::new(153, 68, 68, 0.0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .modulate(Some(ModulateOptions {
        saturation: Some(2.0),
        ..Default::default()
    }))
    .unwrap()
    .raw(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    //should be able to desaturate
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 1,
            height: 1,
            channels: 3,
            background: Colour::new(153, 68, 68, 0.0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .modulate(Some(ModulateOptions {
        saturation: Some(0.5),
        ..Default::default()
    }))
    .unwrap()
    .raw(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    //should be able to lighten
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 1,
            height: 1,
            channels: 3,
            background: Colour::new(153, 68, 68, 0.0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .modulate(Some(ModulateOptions {
        lightness: Some(10.0),
        ..Default::default()
    }))
    .unwrap()
    .raw(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    //should be able to modulate all channels
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 1,
            height: 1,
            channels: 3,
            background: Colour::new(153, 68, 68, 0.0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .modulate(Some(ModulateOptions {
        brightness: Some(2.0),
        saturation: Some(0.5),
        hue: Some(180),
        ..Default::default()
    }))
    .unwrap()
    .raw(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    //'should be able to use linear and modulate together
    let contrast = 1.5;
    let brightness = 0.5;
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 1,
            height: 1,
            channels: 3,
            background: Colour::new(153, 68, 68, 0.0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .linear(Some(vec![contrast]), Some(vec![128.0 * contrast + 128.0]))
    .unwrap()
    .modulate(Some(ModulateOptions {
        brightness: Some(brightness),
        ..Default::default()
    }))
    .unwrap()
    .raw(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    Sharp::cache(true);
    //hue-rotate
    [30, 60, 90, 120, 150, 180, 210, 240, 270, 300, 330, 360].iter().for_each(|angle| {
        let _ = Sharp::new_from_file(fixtures::testPattern())
            .unwrap()
            .resize(320, 320)
            .unwrap()
            .modulate(Some(ModulateOptions {
                hue: Some(*angle),
                ..Default::default()
            }))
            .unwrap()
            .png(Some(PngOptions {
                compression_level: Some(0),
                ..Default::default()
            }))
            .unwrap()
            .to_buffer()
            .unwrap();
    });
}
