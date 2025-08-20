mod fixtures;
use crate::fixtures::clean_up;
use sharp::{Colour, Sharp};

const MAX_DISTANCE: f64 = 6.0;

#[test]
pub fn tint() {
    clean_up();

    //tints rgb image red
    let output = fixtures::output("output.tint-red.jpg");
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tint(Colour::from_hex(0xFF0000))
        .to_file(output.clone())
        .unwrap();
    assert_max_colour_distance!(output, fixtures::expected("tint-red.jpg"), MAX_DISTANCE);

    //tints rgb image green
    let output = fixtures::output("output.tint-green.jpg");
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tint(Colour::from_hex(0x00FF00))
        .to_file(output.clone())
        .unwrap();
    assert_max_colour_distance!(output, fixtures::expected("tint-green.jpg"), MAX_DISTANCE);

    //tints rgb image blue
    let output = fixtures::output("output.tint-blue.jpg");
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tint(Colour::from_hex(0x0000FF))
        .to_file(output.clone())
        .unwrap();
    assert_max_colour_distance!(output, fixtures::expected("tint-blue.jpg"), MAX_DISTANCE);

    //tints rgb image with sepia tone
    let output = fixtures::output("output.tint-sepia-hex.jpg");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .tint(Colour::from_hex(0x704214))
        .to_file_with_info(output.clone())
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_max_colour_distance!(output, fixtures::expected("tint-sepia.jpg"), MAX_DISTANCE);

    //tints rgb image with sepia tone with rgb colour
    let output = fixtures::output("output.tint-sepia-rgb.jpg");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .tint(Colour::new(112, 66, 20, 0.0))
        .to_file_with_info(output.clone())
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_max_colour_distance!(output, fixtures::expected("tint-sepia.jpg"), MAX_DISTANCE);

    //tints rgb image with alpha channel
    let output = fixtures::output("output.tint-alpha.png");
    let info = Sharp::new_from_file(fixtures::inputPngRGBWithAlpha())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .tint(Colour::from_hex(0x704214))
        .to_file_with_info(output.clone())
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_max_colour_distance!(output, fixtures::expected("tint-alpha.png"), MAX_DISTANCE);

    //tints cmyk image red
    let output = fixtures::output("output.tint-cmyk.jpg");
    Sharp::new_from_file(fixtures::inputJpgWithCmykProfile())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .tint(Colour::from_hex(0xFF0000))
        .to_file_with_info(output.clone())
        .unwrap();
    assert_max_colour_distance!(output, fixtures::expected("tint-cmyk.jpg"), MAX_DISTANCE);

    clean_up();
}
