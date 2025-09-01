mod fixtures;
use sharp::{
    input::{Create, Inputs},
    operation::NegateOptions,
    Colour, Sharp,
};

#[test]
pub fn negate() {
    //negate (jpeg)
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(true, None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("negate.jpg"), data, None);

    //negate (png)
    let (data, info) = Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(true, None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("negate.png"), data, None);

    //negate (png, trans)
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(true, None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("negate-trans.png"), data, None);

    //negate (png, alpha)
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(true, None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("negate-alpha.png"), data, None);

    //negate (webp)"
    let (data, info) = Sharp::new_from_file(fixtures::inputWebP())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(true, None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("webp", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("negate.webp"), data, None);

    //negate (webp, trans)
    let (data, info) = Sharp::new_from_file(fixtures::inputWebPWithTransparency())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(true, None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("webp", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("negate-trans.webp"), data, None);

    //negate (true)
    let (data, info) = Sharp::new_from_file(fixtures::inputJpgWithLowContrast())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(true, None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("negate.jpg"), data, None);

    //negate (false)
    let output = fixtures::path("output.unmodified-by-negate.png");
    Sharp::new_from_file(fixtures::inputJpgWithLowContrast())
        .unwrap()
        .negate(false, None)
        .unwrap()
        .to_file(output.clone())
        .unwrap();
    assert_max_colour_distance!(output, fixtures::inputJpgWithLowContrast(), 0.0);

    //negate ({alpha: true})
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(
            true,
            Some(NegateOptions {
                alpha: Some(true),
            }),
        )
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("negate.jpg"), data, None);

    //negate non-alpha channels (png)
    let (data, info) = Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(
            true,
            Some(NegateOptions {
                alpha: Some(false),
            }),
        )
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("negate-preserve-alpha.png"), data, None);

    //negate non-alpha channels (png, trans)
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithTransparency())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(
            true,
            Some(NegateOptions {
                alpha: Some(false),
            }),
        )
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("negate-preserve-alpha-trans.png"), data, None);

    //negate non-alpha channels (png, alpha)
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(
            true,
            Some(NegateOptions {
                alpha: Some(false),
            }),
        )
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("negate-preserve-alpha-grey.png"), data, None);

    //negate non-alpha channels (webp)
    let (data, info) = Sharp::new_from_file(fixtures::inputWebP())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(
            true,
            Some(NegateOptions {
                alpha: Some(false),
            }),
        )
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("webp", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("negate-preserve-alpha.webp"), data, None);

    //negate non-alpha channels (webp, trans)
    let (data, info) = Sharp::new_from_file(fixtures::inputWebPWithTransparency())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .negate(
            true,
            Some(NegateOptions {
                alpha: Some(false),
            }),
        )
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("webp", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("negate-preserve-alpha-trans.webp"), data, None);

    //negate create
    let data = Sharp::new(Inputs::new().create(Create {
        width: 1,
        height: 1,
        channels: 3,
        background: Colour::new(10, 20, 30, 1.0),
        ..Default::default()
    }))
    .unwrap()
    .negate(true, None)
    .unwrap()
    .raw(None)
    .unwrap()
    .to_buffer()
    .unwrap();
    assert_eq!(data[0..3], vec![245, 235, 225]);

    rs_vips::Vips::shutdown();
}
