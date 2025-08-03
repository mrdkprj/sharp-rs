mod fixtures;
use sharp::{Colour, Sharp};

#[test]
pub fn tint() {
    //tints rgb image red
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tint(Colour::from_hex(0xFF0000))
        .to_buffer()
        .unwrap();

    //tints rgb image green
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tint(Colour::from_hex(0x00FF00))
        .to_buffer()
        .unwrap();

    //tints rgb image blue
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tint(Colour::from_hex(0x0000FF))
        .to_buffer()
        .unwrap();

    //tints rgb image with sepia tone
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .tint(Colour::from_hex(0x704214))
        .to_buffer()
        .unwrap();

    //tints rgb image with sepia tone with rgb colour
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .tint(Colour::new(112, 66, 20, 0.0))
        .to_buffer()
        .unwrap();

    //tints rgb image with alpha channel
    Sharp::new_from_file(fixtures::inputPngRGBWithAlpha())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .tint(Colour::from_hex(0x704214))
        .to_buffer()
        .unwrap();

    //tints cmyk image red
    Sharp::new_from_file(fixtures::inputJpgWithCmykProfile())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .tint(Colour::from_hex(0xFF0000))
        .to_buffer()
        .unwrap();
}
