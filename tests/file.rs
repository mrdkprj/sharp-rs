use sharp::{
    input::{Create, CreateRaw, CreateText, Inputs, Noise, SharpOptions},
    FailOn, Sharp,
};
mod fixtures;

#[test]
pub fn test_output() {
    simple();
    overwrite();
    create();
    gif();
    buf();
    rgb();
    text();
    text_rgba();
    icon();
    icon_meta();
    stat();

    fixtures::clean_up();
    rs_vips::Vips::shutdown();
}

fn simple() {
    Sharp::new_from_file_with_opts(
        fixtures::path("img.jpg"),
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
    .to_file(fixtures::output("img2.jpg"))
    .unwrap();
}

fn overwrite() {
    let src = fixtures::path("img.jpg");
    let dest = fixtures::output("img_rot.jpg");
    std::fs::copy(&src, &dest).unwrap();
    let x = Sharp::new_from_file(&dest)
        .unwrap()
        .with_metadata(None)
        .unwrap()
        .rotate(180, None)
        .unwrap()
        .to_buffer()
        .unwrap();
    std::fs::write(&dest, x).unwrap();
}

// Create a blank 300x200 PNG image of semi-translucent red pixels
fn create() {
    Sharp::new_with_opts(
        Inputs::new().create(Create {
            width: 300,
            height: 200,
            channels: 4,
            background: sharp::Colour::new(255, 0, 0, 0.5),
            noise: None,
            ..Default::default()
        }),
        SharpOptions {
            fail_on: Some(FailOn::None),
            ..Default::default()
        },
    )
    .unwrap()
    .png(None)
    .unwrap()
    .to_file(fixtures::output("img2.png"))
    .unwrap();
}

// Convert an animated GIF to an animated WebP
fn gif() {
    Sharp::new_from_file_with_opts(
        fixtures::path("sample.gif"),
        SharpOptions {
            animated: Some(true),
            ..Default::default()
        },
    )
    .unwrap()
    .to_file(fixtures::output("file_out.webp"))
    .unwrap();
}

// Read a raw array of pixels and save it to a png
fn buf() {
    Sharp::new_from_buffer_with_opts(
        vec![255, 255, 255, 0, 0, 0],
        SharpOptions {
            raw: Some(CreateRaw {
                width: 2,
                height: 1,
                channels: 3,
                premultiplied: false,
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .unwrap()
    .to_file(fixtures::output("my-two-pixels.png"))
    .unwrap();
}

// Generate RGB Gaussian noise
fn rgb() {
    Sharp::new(Inputs::new().create(Create {
        width: 300,
        height: 200,
        channels: 3,
        background: sharp::Colour::from_hex(0),
        noise: Some(Noise {
            gaussian: Some(true),
            mean: Some(128.0),
            sigma: Some(30.0),
        }),
        ..Default::default()
    }))
    .unwrap()
    .to_file(fixtures::output("noise.png"))
    .unwrap();
}

// Generate an image from text
fn text() {
    Sharp::new(Inputs::new().text(CreateText {
        text: "Hellow, World!".to_string(),
        width: Some(400),
        height: Some(300),
        ..Default::default()
    }))
    .unwrap()
    .to_file(fixtures::output("text_bw.png"))
    .unwrap();
}

// Generate an rgba image from text using pango markup and font
fn text_rgba() {
    Sharp::new(Inputs::new().text(CreateText {
        text:
            r#"<span foreground="red">Red!</span><span background="cyan">blue</span>"#.to_string(),
        font: Some("sans".to_string()),
        rgba: Some(true),
        dpi: Some(300),
        ..Default::default()
    }))
    .unwrap()
    .to_file(fixtures::output("text_rgba.png"))
    .unwrap();
}

fn icon() {
    Sharp::new_from_file_with_opts(
        fixtures::path("icon.png"),
        SharpOptions {
            fail_on: Some(FailOn::None),
            ..Default::default()
        },
    )
    .unwrap()
    .to_icon(fixtures::output("icon.ico"), None)
    .unwrap();
}

fn icon_meta() {
    let x = Sharp::from_icon_file(fixtures::path("icon.ico")).unwrap().metadata().unwrap();
    println!("icon:{:?}", x);
}

fn stat() {
    Sharp::cache(true);
    let x = Sharp::new_from_file_with_opts(
        fixtures::path("img.jpg"),
        SharpOptions {
            fail_on: Some(FailOn::None),
            ..Default::default()
        },
    )
    .unwrap()
    .stats()
    .unwrap();
    println!("{:?}", x);
}
