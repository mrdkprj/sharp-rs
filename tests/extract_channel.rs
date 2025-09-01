use sharp::{
    input::{Create, Inputs},
    resize::ResizeOptions,
    Colour, Interpretation, Sharp,
};
mod fixtures;

#[test]
fn extract_channel() {
    //Red channel
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .extract_channel(0)
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("extract-red.jpg"), data, None);

    //Green channel
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .extract_channel(1)
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("extract-green.jpg"), data, None);

    //Blue channel
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .extract_channel(2)
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("extract-blue.jpg"), data, None);

    //With colorspace conversion
    let buf = Sharp::new(Inputs::new().create(Create {
        width: 1,
        height: 1,
        channels: 3,
        background: Colour::new(255, 0, 0, 1.0),
        ..Default::default()
    }))
    .unwrap()
    .to_colourspace(Interpretation::Lch)
    .extract_channel(1)
    .unwrap()
    .to_buffer()
    .unwrap();
    assert_eq!(buf, vec![104]);

    //Alpha from 16-bit PNG
    let output = fixtures::output("output.extract-alpha-16bit.png");
    Sharp::new_from_file(fixtures::inputPngWithTransparency16bit())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(16),
            ..Default::default()
        })
        .unwrap()
        .extract_channel(3)
        .unwrap()
        .to_file(output.clone())
        .unwrap();
    assert_max_colour_distance!(output, fixtures::expected("extract-alpha-16bit.png"), 1.0);

    //Alpha from 2-channel input
    let output = fixtures::output("output.extract-alpha-2-channel.png");
    Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .extract_channel(3)
        .unwrap()
        .to_colourspace(Interpretation::BW)
        .to_file(output.clone())
        .unwrap();
    assert_max_colour_distance!(output, fixtures::expected("extract-alpha-2-channel.png"), 1.0);

    fixtures::clean_up();
    rs_vips::Vips::shutdown();
}
