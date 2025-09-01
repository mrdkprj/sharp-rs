mod fixtures;
use sharp::{
    input::{CreateRaw, Input, SharpOptions},
    Interpretation, Sharp,
};

#[test]
pub fn join_channel() {
    //Grayscale to RGB, buffer
    let (data, info) = Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .join_channel(
            &[
                Input::path(fixtures::inputPngTestJoinChannel()),
                Input::path(fixtures::inputPngStripesH()),
            ],
            None,
        )
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_eq!(3, info.channels);
    assert_similar!(fixtures::expected("joinChannel-rgb.jpg"), data, None);

    //Grayscale to RGB, file
    let data = Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .join_channel(
            &[
                Input::buffer(std::fs::read(fixtures::inputPngTestJoinChannel()).unwrap()),
                Input::buffer(std::fs::read(fixtures::inputPngStripesH()).unwrap()),
            ],
            None,
        )
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("joinChannel-rgb.jpg"), data, None);

    //Grayscale to RGBA, buffer
    let (data, info) = Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .join_channel(
            &[
                Input::buffer(std::fs::read(fixtures::inputPngTestJoinChannel()).unwrap()),
                Input::buffer(std::fs::read(fixtures::inputPngStripesH()).unwrap()),
                Input::buffer(std::fs::read(fixtures::inputPngStripesV()).unwrap()),
            ],
            None,
        )
        .unwrap()
        .to_colourspace(Interpretation::Srgb)
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("joinChannel-rgba.png"), data, None);

    //Grayscale to RGBA, file
    let (data, info) = Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .join_channel(
            &[
                Input::path(fixtures::inputPngTestJoinChannel()),
                Input::path(fixtures::inputPngStripesH()),
                Input::path(fixtures::inputPngStripesV()),
            ],
            None,
        )
        .unwrap()
        .to_colourspace(Interpretation::Srgb)
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("joinChannel-rgba.png"), data, None);

    //Grayscale to CMYK, buffers
    let data = Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .join_channel(
            &[
                Input::buffer(std::fs::read(fixtures::inputPngTestJoinChannel()).unwrap()),
                Input::buffer(std::fs::read(fixtures::inputPngStripesH()).unwrap()),
                Input::buffer(std::fs::read(fixtures::inputPngStripesV()).unwrap()),
            ],
            None,
        )
        .unwrap()
        .to_colourspace(Interpretation::Cmyk)
        .to_format(sharp::output::FormatEnum::Jpeg, None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("joinChannel-cmyk.jpg"), data, None);

    //join raw buffers to RGB
    let buf1 = Sharp::new_from_file(fixtures::inputPngTestJoinChannel())
        .unwrap()
        .to_colourspace(Interpretation::BW)
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    let buf2 = Sharp::new_from_file(fixtures::inputPngStripesH())
        .unwrap()
        .to_colourspace(Interpretation::BW)
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    let (data, info) = Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .join_channel(
            &[Input::buffer(buf1), Input::buffer(buf2)],
            Some(SharpOptions {
                raw: Some(CreateRaw {
                    width: 320,
                    height: 240,
                    channels: 1,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_eq!(3, info.channels);
    assert_similar!(fixtures::expected("joinChannel-rgb.jpg"), data, None);

    //Grayscale to RGBA, files, two arrays
    let (data, info) = Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .join_channel(
            &[Input::buffer(std::fs::read(fixtures::inputPngTestJoinChannel()).unwrap())],
            None,
        )
        .unwrap()
        .join_channel(
            &[
                Input::buffer(std::fs::read(fixtures::inputPngStripesH()).unwrap()),
                Input::buffer(std::fs::read(fixtures::inputPngStripesV()).unwrap()),
            ],
            None,
        )
        .unwrap()
        .to_colourspace(Interpretation::Srgb)
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_eq!(4, info.channels);
    assert_similar!(fixtures::expected("joinChannel-rgba.png"), data, None);

    rs_vips::Vips::shutdown();
}
