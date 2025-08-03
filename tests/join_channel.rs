mod fixtures;
use sharp::{
    input::{CreateRaw, SharpOptions},
    Interpretation, Sharp,
};

#[test]
pub fn join_channel() {
    //Grayscale to RGB, buffer
    Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .join_channel(&[fixtures::inputPngTestJoinChannel(), fixtures::inputPngStripesH()], None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //Grayscale to RGB, file
    Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .join_channel_buffers(
            &[
                std::fs::read(fixtures::inputPngTestJoinChannel()).unwrap(),
                std::fs::read(fixtures::inputPngStripesH()).unwrap(),
            ],
            None,
        )
        .unwrap()
        .to_buffer()
        .unwrap();

    //Grayscale to RGBA, buffer
    Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .join_channel(
            &[
                fixtures::inputPngTestJoinChannel(),
                fixtures::inputPngStripesH(),
                fixtures::inputPngStripesV(),
            ],
            None,
        )
        .unwrap()
        .to_colourspace(Interpretation::Srgb)
        .to_buffer()
        .unwrap();

    //Grayscale to RGBA, file
    Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .join_channel_buffers(
            &[
                std::fs::read(fixtures::inputPngTestJoinChannel()).unwrap(),
                std::fs::read(fixtures::inputPngStripesH()).unwrap(),
                std::fs::read(fixtures::inputPngStripesV()).unwrap(),
            ],
            None,
        )
        .unwrap()
        .to_colourspace(Interpretation::Srgb)
        .to_buffer()
        .unwrap();

    //Grayscale to CMYK, buffers
    Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .join_channel_buffers(
            &[
                std::fs::read(fixtures::inputPngTestJoinChannel()).unwrap(),
                std::fs::read(fixtures::inputPngStripesH()).unwrap(),
                std::fs::read(fixtures::inputPngStripesV()).unwrap(),
            ],
            None,
        )
        .unwrap()
        .to_colourspace(Interpretation::Cmyk)
        .to_format(sharp::output::FormatEnum::Jpeg, None)
        .unwrap()
        .to_buffer()
        .unwrap();

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

    Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .join_channel_buffers(
            &[buf1, buf2],
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
        .to_buffer()
        .unwrap();
}
