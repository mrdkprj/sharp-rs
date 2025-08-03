mod fixtures;
use sharp::{
    input::{Create, SharpOptions},
    operation::NegateOptions,
    Colour, Sharp,
};

#[test]
pub fn negate() {
    //negate (jpeg)
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //negate (png)
    Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //negate (png, trans)
    Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //negate (png, alpha)
    Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //negate (webp)'
    Sharp::new_from_file(fixtures::inputWebP())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //negate (webp, trans)
    Sharp::new_from_file(fixtures::inputWebPWithTransparency())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //negate (true)
    Sharp::new_from_file(fixtures::inputJpgWithLowContrast())
        .unwrap()
        .negate(Some(NegateOptions {
            alpha: Some(false),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //negate ({alpha: true})
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .negate(Some(NegateOptions {
            alpha: Some(true),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //negate non-alpha channels (png)
    Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(Some(NegateOptions {
            alpha: Some(false),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //negate non-alpha channels (png, trans)
    Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(Some(NegateOptions {
            alpha: Some(false),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //negate non-alpha channels (png, alpha)
    Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(Some(NegateOptions {
            alpha: Some(false),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //negate non-alpha channels (webp)
    Sharp::new_from_file(fixtures::inputWebP())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(Some(NegateOptions {
            alpha: Some(false),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //negate non-alpha channels (webp, trans)
    Sharp::new_from_file(fixtures::inputWebPWithTransparency())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(Some(NegateOptions {
            alpha: Some(false),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //negate create
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 1,
            height: 1,
            channels: 3,
            background: Colour::new(10, 20, 30, 0.0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .negate(None)
    .unwrap()
    .raw(None)
    .unwrap()
    .to_buffer()
    .unwrap();
}
