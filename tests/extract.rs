mod fixtures;
use sharp::{
    input::{Create, SharpOptions},
    resize::{Region, ResizeOptions},
    Colour, Interpretation, Sharp,
};

#[test]
pub fn extract() {
    //Partial image extraction
    // jpeg
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .extract(Region {
            left: 2,
            top: 2,
            width: 20,
            height: 20,
        })
        .unwrap()
        .to_buffer()
        .unwrap();

    // png
    Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .extract(Region {
            left: 200,
            top: 300,
            width: 400,
            height: 200,
        })
        .unwrap()
        .to_buffer()
        .unwrap();

    // webp
    Sharp::new_from_file(fixtures::inputWebP())
        .unwrap()
        .extract(Region {
            left: 100,
            top: 50,
            width: 125,
            height: 200,
        })
        .unwrap()
        .to_buffer()
        .unwrap();

    //Before resize
    Sharp::new_from_file_with_opts(
        fixtures::inputWebPAnimated(),
        SharpOptions {
            pages: Some(-1),
            ..Default::default()
        },
    )
    .unwrap()
    .extract(Region {
        left: 0,
        top: 30,
        width: 80,
        height: 20,
    })
    .unwrap()
    .resize(320, 80)
    .unwrap()
    .to_buffer()
    .unwrap();

    //After resize
    Sharp::new_from_file_with_opts(
        fixtures::inputWebPAnimated(),
        SharpOptions {
            pages: Some(-1),
            ..Default::default()
        },
    )
    .unwrap()
    .resize(320, 320)
    .unwrap()
    .extract(Region {
        left: 0,
        top: 120,
        width: 320,
        height: 80,
    })
    .unwrap()
    .to_buffer()
    .unwrap();

    // tiff
    Sharp::new_from_file(fixtures::inputTiff())
        .unwrap()
        .extract(Region {
            left: 34,
            top: 63,
            width: 341,
            height: 529,
        })
        .unwrap()
        .to_buffer()
        .unwrap();

    //Before resize
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .extract(Region {
            left: 10,
            top: 10,
            width: 10,
            height: 500,
        })
        .unwrap()
        .resize(100, 100)
        .unwrap()
        .to_buffer()
        .unwrap();

    //After resize
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(500, 500)
        .unwrap()
        .extract(Region {
            left: 10,
            top: 10,
            width: 100,
            height: 100,
        })
        .unwrap()
        .to_buffer()
        .unwrap();

    //Before and after resize and crop
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .extract(Region {
            left: 0,
            top: 0,
            width: 700,
            height: 700,
        })
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: 500,
            height: 500,
            position: Some(sharp::resize::Position::Top),
            ..Default::default()
        })
        .unwrap()
        .extract(Region {
            left: 10,
            top: 10,
            width: 100,
            height: 100,
        })
        .unwrap()
        .to_buffer()
        .unwrap();

    //Image channel extraction

    //Red channel
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .extract_channel(0)
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .to_buffer()
        .unwrap();

    //Green channel
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .extract_channel(1)
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .to_buffer()
        .unwrap();

    //Blue channel
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .extract_channel(2)
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .to_buffer()
        .unwrap();

    //With colorspace conversion
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 1,
            height: 1,
            channels: 3,
            background: Colour::new(255, 0, 0, 1.0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .to_colourspace(Interpretation::Lch)
    .extract_channel(1)
    .unwrap()
    .to_buffer()
    .unwrap();

    //Alpha from 16-bit PNG
    Sharp::new_from_file(fixtures::inputPngWithTransparency16bit())
        .unwrap()
        .resize(16, 16)
        .unwrap()
        .extract_channel(3)
        .unwrap()
        .to_buffer()
        .unwrap();

    //Alpha from 2-channel input
    Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .extract_channel(3)
        .unwrap()
        .to_colourspace(Interpretation::BW)
        .to_buffer()
        .unwrap();
}
