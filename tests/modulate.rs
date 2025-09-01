mod fixtures;
use sharp::{
    input::{Create, Inputs},
    operation::ModulateOptions,
    output::PngOptions,
    Colour, Sharp,
};

#[test]
pub fn modulate() {
    //should be able to hue-rotate
    let data = Sharp::new(Inputs::new().create(Create {
        width: 1,
        height: 1,
        channels: 3,
        background: Colour::new(153, 68, 68, 0.0),
        ..Default::default()
    }))
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
    assert_eq!(vec![41, 107, 57], data[0..3]);

    //should be able to brighten
    let data = Sharp::new(Inputs::new().create(Create {
        width: 1,
        height: 1,
        channels: 3,
        background: Colour::new(153, 68, 68, 1.0),
        ..Default::default()
    }))
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
    assert_eq!(vec![255, 173, 168], data[0..3]);

    //should be able to darken
    let data = Sharp::new(Inputs::new().create(Create {
        width: 1,
        height: 1,
        channels: 3,
        background: Colour::new(153, 68, 68, 1.0),
        ..Default::default()
    }))
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
    assert_eq!(vec![97, 17, 25], data[0..3]);

    //should be able to saturate
    let data = Sharp::new(Inputs::new().create(Create {
        width: 1,
        height: 1,
        channels: 3,
        background: Colour::new(153, 68, 68, 1.0),
        ..Default::default()
    }))
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
    assert_eq!(vec![198, 0, 43], data[0..3]);

    //should be able to desaturate
    let data = Sharp::new(Inputs::new().create(Create {
        width: 1,
        height: 1,
        channels: 3,
        background: Colour::new(153, 68, 68, 1.0),
        ..Default::default()
    }))
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
    assert_eq!(vec![127, 83, 81], data[0..3]);

    //should be able to lighten
    let data = Sharp::new(Inputs::new().create(Create {
        width: 1,
        height: 1,
        channels: 3,
        background: Colour::new(153, 68, 68, 1.0),
        ..Default::default()
    }))
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
    assert_eq!(vec![182, 93, 92], data[0..3]);

    //should be able to modulate all channels
    let data = Sharp::new(Inputs::new().create(Create {
        width: 1,
        height: 1,
        channels: 3,
        background: Colour::new(153, 68, 68, 1.0),
        ..Default::default()
    }))
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
    assert_eq!(vec![149, 209, 214], data[0..3]);

    //'should be able to use linear and modulate together
    let contrast = 1.5;
    let brightness = 0.5;
    let data = Sharp::new(Inputs::new().create(Create {
        width: 1,
        height: 1,
        channels: 3,
        background: Colour::new(153, 68, 68, 1.0),
        ..Default::default()
    }))
    .unwrap()
    .linear(Some(vec![contrast]), Some(vec![-(128.0 * contrast) + 128.0]))
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
    assert_eq!(vec![81, 0, 0], data[0..3]);

    Sharp::cache(true);
    //hue-rotate
    [30, 60, 90, 120, 150, 180, 210, 240, 270, 300, 330, 360].iter().for_each(|angle| {
        let base = format!("modulate-hue-angle-{:?}.png", angle);
        let actual = fixtures::output(&format!("output.{}", base));
        let expected = fixtures::expected(&base);
        Sharp::new_from_file(fixtures::testPattern())
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
            .to_file(actual.clone())
            .unwrap();
        assert_max_colour_distance!(actual, expected, 3.0);
    });

    fixtures::clean_up();
    rs_vips::Vips::shutdown();
}
