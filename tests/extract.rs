mod fixtures;
use sharp::{
    input::SharpOptions,
    resize::{Region, ResizeOptions},
    Sharp,
};

#[test]
pub fn extract() {
    //Partial image extraction
    // jpeg
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .extract(Region {
            left: 2,
            top: 2,
            width: 20,
            height: 20,
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(20, info.width);
    assert_eq!(20, info.height);
    assert_similar!(fixtures::expected("extract.jpg"), data, None);

    // png
    let (data, info) = Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .extract(Region {
            left: 200,
            top: 300,
            width: 400,
            height: 200,
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(400, info.width);
    assert_eq!(200, info.height);
    assert_similar!(fixtures::expected("extract.png"), data, None);

    // webp
    let (data, info) = Sharp::new_from_file(fixtures::inputWebP())
        .unwrap()
        .extract(Region {
            left: 100,
            top: 50,
            width: 125,
            height: 200,
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(125, info.width);
    assert_eq!(200, info.height);
    assert_similar!(fixtures::expected("extract.webp"), data, None);

    //animated webp before resize
    let (data, info) = Sharp::new_from_file_with_opts(
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
    .to_buffer_with_info()
    .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(80 * 9, info.height);
    assert_similar!(fixtures::expected("gravity-center-height.webp"), data, None);

    //animated webp after resize
    let (data, info) = Sharp::new_from_file_with_opts(
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
    .to_buffer_with_info()
    .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(80 * 9, info.height);
    assert_similar!(fixtures::expected("gravity-center-height.webp"), data, None);

    // tiff
    let (data, info) = Sharp::new_from_file(fixtures::inputTiff())
        .unwrap()
        .extract(Region {
            left: 34,
            top: 63,
            width: 341,
            height: 529,
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(341, info.width);
    assert_eq!(529, info.height);
    assert_similar!(fixtures::expected("extract.tiff"), data, None);

    //Before resize
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
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
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(100, info.width);
    assert_eq!(100, info.height);
    assert_similar!(fixtures::expected("extract-resize.jpg"), data, None);

    //After resize and crop
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(500),
            height: Some(500),
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
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(100, info.width);
    assert_eq!(100, info.height);
    assert_similar!(fixtures::expected("resize-crop-extract.jpg"), data, None);

    //Before and after resize and crop
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .extract(Region {
            left: 0,
            top: 0,
            width: 700,
            height: 700,
        })
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(500),
            height: Some(500),
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
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(100, info.width);
    assert_eq!(100, info.height);
    assert_similar!(fixtures::expected("extract-resize-crop-extract.jpg"), data, None);

    //Extract then rotate
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .extract(Region {
            left: 20,
            top: 10,
            width: 380,
            height: 280,
        })
        .unwrap()
        .rotate(90, None)
        .unwrap()
        .jpeg(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(280, info.width);
    assert_eq!(380, info.height);
    assert_similar!(fixtures::expected("extract-rotate.jpg"), data, None);

    //Rotate then extract
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .rotate(90, None)
        .unwrap()
        .extract(Region {
            left: 20,
            top: 10,
            width: 280,
            height: 380,
        })
        .unwrap()
        .jpeg(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(280, info.width);
    assert_eq!(380, info.height);
    assert_similar!(fixtures::expected("rotate-extract.jpg"), data, None);

    //Extract then rotate then extract
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .extract(Region {
            left: 20,
            top: 10,
            width: 180,
            height: 280,
        })
        .unwrap()
        .rotate(90, None)
        .unwrap()
        .extract(Region {
            left: 20,
            top: 10,
            width: 200,
            height: 100,
        })
        .unwrap()
        .jpeg(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(200, info.width);
    assert_eq!(100, info.height);
    assert_similar!(fixtures::expected("extract-rotate-extract.jpg"), data, None);

    //Extract then rotate non-90 anagle
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .extract(Region {
            left: 20,
            top: 10,
            width: 380,
            height: 280,
        })
        .unwrap()
        .rotate(45, None)
        .unwrap()
        .jpeg(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(467, info.width);
    assert_eq!(467, info.height);
    assert_similar!(fixtures::expected("extract-rotate-45.jpg"), data, None);

    //Rotate then extract non-90 angle
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithGreyAlpha())
        .unwrap()
        .rotate(45, None)
        .unwrap()
        .extract(Region {
            left: 20,
            top: 10,
            width: 380,
            height: 280,
        })
        .unwrap()
        .jpeg(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(380, info.width);
    assert_eq!(280, info.height);
    assert_similar!(fixtures::expected("rotate-extract-45.jpg"), data, None);

    //Apply exif orientation and mirroring then extract
    [
        fixtures::inputJpgWithLandscapeExif1(),
        fixtures::inputJpgWithLandscapeExif2(),
        fixtures::inputJpgWithLandscapeExif3(),
        fixtures::inputJpgWithLandscapeExif4(),
        fixtures::inputJpgWithLandscapeExif5(),
        fixtures::inputJpgWithLandscapeExif6(),
        fixtures::inputJpgWithLandscapeExif7(),
        fixtures::inputJpgWithLandscapeExif8(),
    ]
    .iter()
    .for_each(|inp| {
        let data = Sharp::new_from_file(inp)
            .unwrap()
            .auto_orient()
            .unwrap()
            .extract(Region {
                left: 0,
                top: 208,
                width: 60,
                height: 40,
            })
            .unwrap()
            .to_buffer()
            .unwrap();
        assert_similar!(fixtures::expected("rotate-mirror-extract.jpg"), data, None);
    });
}
