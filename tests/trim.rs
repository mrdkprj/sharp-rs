use sharp::{
    input::{Create, SharpOptions},
    output::PngOptions,
    resize::{ExtendOptions, ResizeOptions, TrimOptions},
    Colour, Sharp,
};
mod fixtures;

#[test]
fn trim() {
    //Skip shrink-on-load
    Sharp::new_from_file(fixtures::inputJpgOverlayLayer2())
        .unwrap()
        .trim(None)
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: 300,
            height: 300,
            fast_shrink_on_load: Some(false),
            ..Default::default()
        })
        .unwrap()
        .to_buffer()
        .unwrap();

    //Single colour PNG where alpha channel provides the image
    Sharp::new_from_file(fixtures::inputPngImageInAlpha())
        .unwrap()
        .trim(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //16-bit PNG with alpha channel
    Sharp::new_from_file(fixtures::inputPngWithTransparency16bit())
        .unwrap()
        .resize(32, 32)
        .unwrap()
        .trim(Some(TrimOptions {
            threshold: Some(20.0),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Should rotate before trim
    let rotated30 = Sharp::new(SharpOptions {
        create: Some(Create {
            width: 20,
            height: 30,
            channels: 3,
            background: Colour::new(255, 255, 255, 1.0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .rotate(30, None)
    .unwrap()
    .png(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    Sharp::new_from_buffer(rotated30)
        .unwrap()
        .rotate(-30, None)
        .unwrap()
        .trim(Some(TrimOptions {
            threshold: Some(128.0),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Ensure greyscale image can be trimmed
    let greyscale = Sharp::new(SharpOptions {
        create: Some(Create {
            width: 16,
            height: 8,
            channels: 3,
            background: Colour::from_hex(0xc0c0c0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .extend(ExtendOptions {
        left: Some(12),
        right: Some(24),
        background: Some(Colour::from_hex(0x808080)),
        ..Default::default()
    })
    .unwrap()
    .to_colourspace(sharp::Interpretation::BW)
    .png(Some(PngOptions {
        compression_level: Some(0),
        ..Default::default()
    }))
    .unwrap()
    .to_buffer()
    .unwrap();

    Sharp::new_from_buffer(greyscale)
        .unwrap()
        .trim(None)
        .unwrap()
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //Ensure cmyk image can be trimmed
    let cmyk = Sharp::new(SharpOptions {
        create: Some(Create {
            width: 16,
            height: 8,
            channels: 3,
            background: Colour::from_hex(0xff0000),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .extend(ExtendOptions {
        left: Some(12),
        right: Some(24),
        background: Some(Colour::from_hex(0x0000ff)),
        ..Default::default()
    })
    .unwrap()
    .to_colourspace(sharp::Interpretation::Cmyk)
    .jpeg(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    Sharp::new_from_buffer(cmyk)
        .unwrap()
        .trim(None)
        .unwrap()
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //Ensure trim of image with all pixels same is no-op
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 5,
            height: 5,
            channels: 3,
            background: Colour::from_hex(0xff0000),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .trim(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    //Works with line-art
    Sharp::new_from_file(fixtures::inputJpgOverlayLayer2())
        .unwrap()
        .trim(Some(TrimOptions {
            line_art: Some(true),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Specific background colour
    Sharp::new_from_file(fixtures::inputPngTrimSpecificColour())
        .unwrap()
        .trim(Some(TrimOptions {
            background: Some(Colour::from_hex(0xffff00)),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Only trims the bottom
    Sharp::new_from_file(fixtures::inputPngTrimSpecificColour())
        .unwrap()
        .trim(Some(TrimOptions {
            background: Some(Colour::from_hex(0x21468B)),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Only trims the bottom, in 16-bit
    Sharp::new_from_file(fixtures::inputPngTrimSpecificColour16bit())
        .unwrap()
        .trim(Some(TrimOptions {
            background: Some(Colour::from_hex(0x21468B)),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Only trims the bottom, including alpha
    Sharp::new_from_file(fixtures::inputPngTrimSpecificColourIncludeAlpha())
        .unwrap()
        .trim(Some(TrimOptions {
            background: Some(Colour::from_hex(0x21468B80)),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
}
