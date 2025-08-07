use sharp::{
    resize::{Fit, Position, ResizeOptions},
    Colour, Sharp,
};
mod fixtures;

#[test]
fn resize() {
    //Allows specifying the position as a string
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: 320,
            height: 240,
            fit: Some(Fit::Contain),
            position: Some(Position::Centre),
            ..Default::default()
        })
        .unwrap()
        .png(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //JPEG within PNG, no alpha channel
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: 320,
            height: 240,
            fit: Some(Fit::Contain),
            ..Default::default()
        })
        .unwrap()
        .png(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //JPEG within WebP, to include alpha channel
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: 320,
            height: 240,
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            ..Default::default()
        })
        .unwrap()
        .webp(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //16-bit PNG with alpha channel onto RGBA
    Sharp::new_from_file(fixtures::inputPngWithTransparency16bit())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: 32,
            height: 16,
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            ..Default::default()
        })
        .unwrap()
        .to_buffer()
        .unwrap();

    //PNG with 2 channels
    Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: 32,
            height: 16,
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            ..Default::default()
        })
        .unwrap()
        .to_buffer()
        .unwrap();

    //TIFF in LAB colourspace onto RGBA background
    Sharp::new_from_file(fixtures::inputTiffCielab())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: 64,
            height: 128,
            fit: Some(Fit::Contain),
            background: Some(Colour::new(255, 102, 0, 0.5)),
            ..Default::default()
        })
        .unwrap()
        .png(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    // Position horizontal top
    Sharp::new_from_file(fixtures::inputPngEmbed())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: 200,
            height: 100,
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            position: Some(Position::Top),
            ..Default::default()
        })
        .unwrap()
        .to_buffer()
        .unwrap();

    //Position horizontal right top
    Sharp::new_from_file(fixtures::inputPngEmbed())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: 200,
            height: 100,
            fit: Some(Fit::Contain),
            background: Some(Colour::new(0, 0, 0, 0.0)),
            position: Some(Position::RightTop),
            ..Default::default()
        })
        .unwrap()
        .to_buffer()
        .unwrap();

    //Resize fit=cover
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: 320,
            height: 80,
            position: Some(Position::Top),
            ..Default::default()
        })
        .unwrap()
        .to_buffer()
        .unwrap();
}
