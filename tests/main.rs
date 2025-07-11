#![allow(dead_code)]
use sharp::{
    input::{Create, CreateRaw, CreateText, Noise, SharpOptions},
    FailOn, Sharp,
};

#[test]
fn run_all() {
    simple();
    overwrite().unwrap();
    create();
    gif();
    buf();
    rgb();
    text();
    text_rgba();
    metadata();
    icon();
    icon_meta();
}

#[test]
fn simple() {
    Sharp::new_from_file_with_opts(
        concat!(env!("CARGO_MANIFEST_DIR"), r"\tests\img\img.jpg"),
        SharpOptions {
            fail_on: Some(FailOn::None),
            ..Default::default()
        },
    )
    .unwrap()
    .with_metadata(None)
    .unwrap()
    .resize(800, 800)
    .unwrap()
    .with_metadata(None)
    .unwrap()
    .rotate(180, None)
    .unwrap()
    .jpeg(None)
    .unwrap()
    .to_file(concat!(env!("CARGO_MANIFEST_DIR"), r"\tests\img\img2.jpg"))
    .unwrap();
}

#[test]
fn overwrite() -> Result<(), String> {
    let src = concat!(env!("CARGO_MANIFEST_DIR"), r"\tests\img\img.jpg");
    let dest = concat!(env!("CARGO_MANIFEST_DIR"), r"\tests\img\img_rot.jpg");
    std::fs::copy(src, dest).unwrap();
    let x = Sharp::new_from_file(dest)?.with_metadata(None)?.rotate(180, None)?.with_metadata(None)?.to_buffer()?;
    std::fs::write(dest, x).unwrap();
    Ok(())
}

// Create a blank 300x200 PNG image of semi-translucent red pixels
#[test]
fn create() {
    Sharp::new(SharpOptions {
        fail_on: Some(FailOn::None),
        create: Some(Create {
            width: 300,
            height: 200,
            channels: 4,
            background: sharp::Colour::new(255, 0, 0, 0.5),
            noise: None,
        }),
        ..Default::default()
    })
    .unwrap()
    .png(None)
    .unwrap()
    .to_file(concat!(env!("CARGO_MANIFEST_DIR"), r"\tests\img\img2.png"))
    .unwrap();
    println!("done");
}

// Convert an animated GIF to an animated WebP
#[test]
fn gif() {
    Sharp::new_from_file_with_opts(
        concat!(env!("CARGO_MANIFEST_DIR"), r"\tests\img\sample.gif"),
        SharpOptions {
            animated: Some(true),
            ..Default::default()
        },
    )
    .unwrap()
    .to_file(concat!(env!("CARGO_MANIFEST_DIR"), r"\tests\img\file_out.webp"))
    .unwrap();
}

// Read a raw array of pixels and save it to a png
#[test]
fn buf() {
    Sharp::new_from_buffer_with_opts(
        vec![255, 255, 255, 0, 0, 0],
        SharpOptions {
            raw: Some(CreateRaw {
                width: 2,
                height: 1,
                channels: 3,
                premultiplied: false,
            }),
            ..Default::default()
        },
    )
    .unwrap()
    .to_file(concat!(env!("CARGO_MANIFEST_DIR"), r"\tests\img\my-two-pixels.png"))
    .unwrap();
}

// Generate RGB Gaussian noise
#[test]
fn rgb() {
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 300,
            height: 200,
            channels: 3,
            background: sharp::Colour::from_hex(0),
            noise: Some(Noise {
                gaussian: Some(true),
                mean: Some(128.0),
                sigma: Some(30.0),
            }),
        }),
        ..Default::default()
    })
    .unwrap()
    .to_file(concat!(env!("CARGO_MANIFEST_DIR"), r"\tests\img\noise.png"))
    .unwrap();
}

// Generate an image from text
#[test]
fn text() {
    Sharp::new(SharpOptions {
        text: Some(CreateText {
            text: "Hellow, World!".to_string(),
            width: Some(400),
            height: Some(300),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .to_file(concat!(env!("CARGO_MANIFEST_DIR"), r"\tests\img\text_bw.png"))
    .unwrap();
}

// Generate an rgba image from text using pango markup and font
#[test]
fn text_rgba() {
    Sharp::new(SharpOptions {
        text: Some(CreateText {
            text: r#"<span foreground="red">Red!</span><span background="cyan">blue</span>"#.to_string(),
            font: Some("sans".to_string()),
            rgba: Some(true),
            dpi: Some(300),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .to_file(concat!(env!("CARGO_MANIFEST_DIR"), r"\tests\img\text_rgba.png"))
    .unwrap();
}

#[test]
fn metadata() {
    let _ = Sharp::new_from_file_with_opts(
        concat!(env!("CARGO_MANIFEST_DIR"), r"\tests\img\img.jpg"),
        SharpOptions {
            fail_on: Some(FailOn::None),
            ..Default::default()
        },
    )
    .unwrap()
    .metadata()
    .unwrap();
    println!("done3");
    let data = Sharp::new_from_file_with_opts(
        concat!(env!("CARGO_MANIFEST_DIR"), r"\tests\img\img.jpg"),
        SharpOptions {
            fail_on: Some(FailOn::None),
            ..Default::default()
        },
    )
    .unwrap()
    .metadata()
    .unwrap();
    println!("{:?}", data);
}

#[test]
fn icon() {
    Sharp::new_from_file_with_opts(
        concat!(env!("CARGO_MANIFEST_DIR"), r"\tests\img\icon.png"),
        SharpOptions {
            fail_on: Some(FailOn::None),
            ..Default::default()
        },
    )
    .unwrap()
    .to_icon(concat!(env!("CARGO_MANIFEST_DIR"), r"\tests\img\icon.ico"))
    .unwrap();
}

#[test]
fn icon_meta() {
    let x = Sharp::from_icon_file(concat!(env!("CARGO_MANIFEST_DIR"), r"\tests\img\icon.ico")).unwrap().metadata().unwrap();
    println!("icon:{:?}", x);
}
/*
 * @example
 * // Join four input images as a 2x2 grid with a 4 pixel gutter
 * const data = await sharp(
 *  [image1, image2, image3, image4],
 *  { join: { across: 2, shim: 4 } }
 * ).toBuffer();
 *
 * @example
 * // Generate a two-frame animated image from emoji
 * const images = ['😀', '😛'].map(text => ({
 *   text: { text, width: 64, height: 64, channels: 4, rgba: true }
 * }));
 */
