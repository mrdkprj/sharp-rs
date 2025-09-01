#![allow(non_snake_case)]
use sharp::{
    input::{Create, Inputs, RotateOptions, SharpOptions},
    output::{AvifOptions, PngOptions, WriteableMetadata},
    resize::{Fit, Region, ResizeOptions},
    Colour, Sharp,
};
mod fixtures;

#[test]
fn rotate() {
    ["autoOrient", "constructor"].into_iter().for_each(|rotateMethod| {
        let options = if rotateMethod == "constructor" {
            SharpOptions {
                auto_orient: Some(true),
                ..Default::default()
            }
        } else {
            SharpOptions::default()
        };

        ["Landscape", "Portrait"].into_iter().for_each(|orientation| {
            [1, 2, 3, 4, 5, 6, 7, 8].into_iter().for_each(|exifTag| {
                let input = fixtures::input(&format!("inputJpgWith{orientation}Exif{exifTag}"));
                let expected = fixtures::expected(&format!("{orientation}_{exifTag}-out.jpg"));
                //{orientation} image with EXIF Orientation ${exifTag}: Auto-rotate
                let (expectedWidth, expectedHeight) = if orientation == "Landscape" {
                    (600, 450)
                } else {
                    (450, 600)
                };

                let img = Sharp::new_from_file_with_opts(input.clone(), options.clone()).unwrap();
                let img = if rotateMethod == "autoOrient" {
                    img.auto_orient().unwrap()
                } else {
                    img
                };
                let (data, info) = img.to_buffer_with_info().unwrap();
                assert_eq!(info.width, expectedWidth);
                assert_eq!(info.height, expectedHeight);
                assert_similar!(expected.clone(), data, None);

                // ${orientation} image with EXIF Orientation ${exifTag}: Auto-rotate then resize
                let (expectedWidth, expectedHeight) = if orientation == "Landscape" {
                    (320, 240)
                } else {
                    (320, 427)
                };

                let img = Sharp::new_from_file_with_opts(input.clone(), options.clone()).unwrap();
                let img = if rotateMethod == "autoOrient" {
                    img.auto_orient().unwrap()
                } else {
                    img
                };
                let (data, info) = img
                    .resize_with_opts(ResizeOptions {
                        width: Some(320),
                        ..Default::default()
                    })
                    .unwrap()
                    .to_buffer_with_info()
                    .unwrap();
                assert_eq!(info.width, expectedWidth);
                assert_eq!(info.height, expectedHeight);
                assert_similar!(expected.clone(), data, None);

                // ${orientation} image with EXIF Orientation ${exifTag}: Resize then auto-rotate
                if rotateMethod != "constructor" {
                    let (expectedWidth, expectedHeight) = if orientation == "Landscape" {
                        (320, 240)
                    } else {
                        (320, 427)
                    };
                    let img = Sharp::new_from_file_with_opts(input.clone(), options.clone())
                        .unwrap()
                        .resize_with_opts(ResizeOptions {
                            width: Some(320),
                            ..Default::default()
                        })
                        .unwrap();
                    let img = if rotateMethod == "autoOrient" {
                        img.auto_orient().unwrap()
                    } else {
                        img
                    };
                    let (data, info) = img.to_buffer_with_info().unwrap();
                    assert_eq!(info.width, expectedWidth);
                    assert_eq!(info.height, expectedHeight);
                    assert_similar!(expected.clone(), data, None);
                }

                [true, false].into_iter().for_each(|doResize| {
                    [90, 180, 270, 45].into_iter().for_each(|angle| {
                        //${orientation} image with EXIF Orientation ${exifTag}: Auto-rotate then rotate ${angle} ${doResize ? 'and resize
                        let (inputWidth, inputHeight) = if orientation == "Landscape" {
                            (600, 450)
                        } else {
                            (450, 600)
                        };
                        let expected = fixtures::expected(&format!(
                            "{orientation}_{exifTag}_rotate{angle}-out.jpg"
                        ));
                        let (width, height) = if angle == 45 {
                            if doResize {
                                (
                                    (742.0_f64 / 1.875).floor() as i32,
                                    (742.0_f64 / 1.875).floor() as i32,
                                )
                            } else {
                                (742, 742)
                            }
                        } else if doResize {
                            (
                                (inputWidth as f64 / 1.875).floor() as i32,
                                (inputHeight as f64 / 1.875).floor() as i32,
                            )
                        } else {
                            (inputWidth, inputHeight)
                        };
                        let (expectedWidth, expectedHeight) = if angle % 180 == 0 {
                            (width, height)
                        } else {
                            (height, width)
                        };

                        let img =
                            Sharp::new_from_file_with_opts(input.clone(), options.clone()).unwrap();
                        let img = if rotateMethod == "autoOrient" {
                            img.auto_orient().unwrap()
                        } else {
                            img
                        };
                        let img = img.rotate(angle, None).unwrap();
                        let img = if doResize {
                            img.resize_with_opts(ResizeOptions {
                                width: Some(expectedWidth),
                                ..Default::default()
                            })
                            .unwrap()
                        } else {
                            img
                        };
                        let (data, info) = img.to_buffer_with_info().unwrap();
                        assert_eq!(info.width, expectedWidth);
                        assert_eq!(info.height, expectedHeight);
                        assert_similar!(expected.clone(), data, None);

                        [(true, true), (true, false), (false, true)].into_iter().for_each(
                            |(flip, flop)| {
                                let (inputWidth, inputHeight) = if orientation == "Landscape" {
                                    (600, 450)
                                } else {
                                    (450, 600)
                                };
                                let flipFlopFileName = if flip && !flop {
                                    "flip"
                                } else if !flip && flop {
                                    "flop"
                                } else {
                                    "flip_flop"
                                };

                                //${orientation} image with EXIF Orientation ${exifTag}: Auto-rotate then ${flipFlopTestName} ${doResize ? 'and resize' : ''}
                                let expected = fixtures::expected(&format!(
                                    "{orientation}_{exifTag}_{flipFlopFileName}-out.jpg"
                                ));
                                let img =
                                    Sharp::new_from_file_with_opts(input.clone(), options.clone())
                                        .unwrap();
                                let img = if rotateMethod == "autoOrient" {
                                    img.auto_orient().unwrap()
                                } else {
                                    img
                                };
                                let img = if flip {
                                    img.flip(true).unwrap()
                                } else {
                                    img
                                };
                                let img = if flop {
                                    img.flop(true).unwrap()
                                } else {
                                    img
                                };
                                let img = if doResize {
                                    img.resize_with_opts(ResizeOptions {
                                        width: if orientation == "Landscape" {
                                            Some(320)
                                        } else {
                                            Some(240)
                                        },
                                        ..Default::default()
                                    })
                                    .unwrap()
                                } else {
                                    img
                                };
                                let (data, info) = img.to_buffer_with_info().unwrap();
                                assert_eq!(
                                    info.width,
                                    (inputWidth as f64
                                        / if doResize {
                                            1.875
                                        } else {
                                            1.0
                                        }) as i32
                                );
                                assert_eq!(
                                    info.height,
                                    (inputHeight as f64
                                        / if doResize {
                                            1.875
                                        } else {
                                            1.0
                                        }) as i32
                                );
                                assert_similar!(expected, data, None);
                            },
                        );
                    })
                });
            });
        });
    });

    //Rotate by 30 degrees with semi-transparent background
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            ..Default::default()
        })
        .unwrap()
        .rotate(
            30,
            Some(RotateOptions {
                background: Colour::new(255, 0, 0, 0.5),
            }),
        )
        .unwrap()
        .png(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(408, info.width);
    assert_eq!(386, info.height);
    assert_similar!(fixtures::expected("rotate-transparent-bg.png"), data, None);

    //Rotate by 30 degrees with solid background
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            ..Default::default()
        })
        .unwrap()
        .rotate(
            30,
            Some(RotateOptions {
                background: Colour::new(255, 0, 0, 1.0),
            }),
        )
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(408, info.width);
    assert_eq!(386, info.height);
    assert_similar!(fixtures::expected("rotate-solid-bg.jpg"), data, None);

    //Rotate by 90 degrees, respecting output input size
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .rotate(90, None)
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);

    //Resize then rotate by 30 degrees, respecting output input size
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .rotate(30, None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(397, info.width);
    assert_eq!(368, info.height);

    //Rotate by any 90-multiple angle
    [-3690, -450, -90, 90, 450, 3690].into_iter().for_each(|angle| {
        let (_, info) = Sharp::new_from_file(fixtures::inputJpg320x240())
            .unwrap()
            .rotate(angle, None)
            .unwrap()
            .to_buffer_with_info()
            .unwrap();
        assert_eq!(240, info.width);
        assert_eq!(320, info.height);
    });

    //Rotate by any 30-multiple angle
    [-3750, -510, -150, 30, 390, 3630].into_iter().for_each(|angle| {
        let (_, info) = Sharp::new_from_file(fixtures::inputJpg320x240())
            .unwrap()
            .rotate(angle, None)
            .unwrap()
            .to_buffer_with_info()
            .unwrap();
        assert_eq!(397, info.width);
        assert_eq!(368, info.height);
    });

    //Rotate by any 180-multiple angle
    [-3780, -540, 0, 180, 540, 3780].into_iter().for_each(|angle| {
        let (_, info) = Sharp::new_from_file(fixtures::inputJpg320x240())
            .unwrap()
            .rotate(angle, None)
            .unwrap()
            .to_buffer_with_info()
            .unwrap();
        assert_eq!(320, info.width);
        assert_eq!(240, info.height);
    });

    //Rotate by 270 degrees, square output ignoring aspect ratio
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(240),
            height: Some(240),
            fit: Some(Fit::Fill),
            ..Default::default()
        })
        .unwrap()
        .rotate(270, None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(240, info.width);
    assert_eq!(240, info.height);
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(240, metadata.width);
    assert_eq!(240, metadata.height);

    //Rotate by 315 degrees, square output ignoring aspect ratio
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(240),
            height: Some(240),
            fit: Some(Fit::Fill),
            ..Default::default()
        })
        .unwrap()
        .rotate(315, None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(339, info.width);
    assert_eq!(339, info.height);
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(339, metadata.width);
    assert_eq!(339, metadata.height);

    //Rotate by 270 degrees, rectangular output ignoring aspect ratio
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .rotate(270, None)
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(240),
            fit: Some(Fit::Fill),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(320, metadata.width);
    assert_eq!(240, metadata.height);

    //Auto-rotate by 270 degrees, rectangular output ignoring aspect ratio
    let (data, info) = Sharp::new_from_file(fixtures::inputJpgWithLandscapeExif8())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(240),
            fit: Some(Fit::Fill),
            ..Default::default()
        })
        .unwrap()
        .auto_orient()
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(320, metadata.width);
    assert_eq!(240, metadata.height);

    //Rotate by 30 degrees, rectangular output ignoring aspect ratio
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(240),
            fit: Some(Fit::Fill),
            ..Default::default()
        })
        .unwrap()
        .rotate(30, None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(397, info.width);
    assert_eq!(368, info.height);
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(397, metadata.width);
    assert_eq!(368, metadata.height);

    //Input image has Orientation EXIF tag but do not rotate output
    let (data, info) = Sharp::new_from_file(fixtures::inputJpgWithExif())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            ..Default::default()
        })
        .unwrap()
        .with_metadata(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(427, info.height);
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(8, metadata.orientation);

    //Input image has Orientation EXIF tag value of 8 (270 degrees), auto-rotate
    let (data, info) = Sharp::new_from_file(fixtures::inputJpgWithExif())
        .unwrap()
        .auto_orient()
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("exif-8.jpg"), data, None);

    //Override EXIF Orientation tag metadata after auto-rotate
    let (data, info) = Sharp::new_from_file(fixtures::inputJpgWithExif())
        .unwrap()
        .auto_orient()
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            ..Default::default()
        })
        .unwrap()
        .with_metadata(Some(WriteableMetadata {
            orientation: Some(3),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    let metadata = Sharp::new_from_buffer(data.clone()).unwrap().metadata().unwrap();
    assert_eq!(3, metadata.orientation);
    assert_similar!(fixtures::expected("exif-8.jpg"), data, None);

    //Input image has Orientation EXIF tag value of 5 (270 degrees + flip), auto-rotate
    let (data, info) = Sharp::new_from_file(fixtures::inputJpgWithExifMirroring())
        .unwrap()
        .auto_orient()
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            ..Default::default()
        })
        .unwrap()
        .with_metadata(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    let metadata = Sharp::new_from_buffer(data.clone()).unwrap().metadata().unwrap();
    assert_eq!(1, metadata.orientation);
    assert_similar!(fixtures::expected("exif-5.jpg"), data, None);

    //Attempt to auto-rotate using image that has no EXIF
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .auto_orient()
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(261, info.height);

    //Attempt to auto-rotate image format without EXIF support
    let (_, info) = Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .auto_orient()
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            ..Default::default()
        })
        .unwrap()
        .jpeg(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(236, info.height);

    //Animated image rotate 180
    Sharp::new_from_file_with_opts(
        fixtures::inputGifAnimated(),
        SharpOptions {
            animated: Some(true),
            ..Default::default()
        },
    )
    .unwrap()
    .rotate(180, None)
    .unwrap()
    .to_buffer()
    .unwrap();

    //Multiple rotate: last one wins (cardinal)
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .rotate(45, None)
        .unwrap()
        .rotate(90, None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(2225, info.width);
    assert_eq!(2725, info.height);

    //Multiple rotate: last one wins (non cardinal)
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .rotate(90, None)
        .unwrap()
        .rotate(45, None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(3500, info.width);
    assert_eq!(3500, info.height);

    //Flip - vertical
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            ..Default::default()
        })
        .unwrap()
        .flip(true)
        .unwrap()
        .with_metadata(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(261, info.height);
    let metadata = Sharp::new_from_buffer(data.clone()).unwrap().metadata().unwrap();
    assert_eq!(1, metadata.orientation);
    assert_similar!(fixtures::expected("flip.jpg"), data, None);

    //Flop - horizontal
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            ..Default::default()
        })
        .unwrap()
        .flop(true)
        .unwrap()
        .with_metadata(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(261, info.height);
    let metadata = Sharp::new_from_buffer(data.clone()).unwrap().metadata().unwrap();
    assert_eq!(1, metadata.orientation);
    assert_similar!(fixtures::expected("flop.jpg"), data, None);

    //Flip and flop
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            ..Default::default()
        })
        .unwrap()
        .flip(true)
        .unwrap()
        .flop(true)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(261, info.height);
    assert_similar!(fixtures::expected("flip-and-flop.jpg"), data, None);

    //Neither flip nor flop
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            ..Default::default()
        })
        .unwrap()
        .flip(false)
        .unwrap()
        .flop(false)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(261, info.height);
    assert_similar!(fixtures::inputJpg(), data, None);

    //Auto-rotate and flip
    let (data, info) = Sharp::new_from_file(fixtures::inputJpgWithExif())
        .unwrap()
        .auto_orient()
        .unwrap()
        .flip(true)
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("rotate-and-flip.jpg"), data, None);

    //Auto-rotate and flop
    let (data, info) = Sharp::new_from_file(fixtures::inputJpgWithExif())
        .unwrap()
        .auto_orient()
        .unwrap()
        .flop(true)
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("rotate-and-flop.jpg"), data, None);

    //Auto-rotate and shrink-on-load
    let data = Sharp::new_from_file(fixtures::inputJpgWithLandscapeExif3())
        .unwrap()
        .auto_orient()
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(8),
            ..Default::default()
        })
        .unwrap()
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_eq!(data[0..3], vec![61, 74, 51]);

    //Flip and rotate ordering
    let data = Sharp::new_from_file(fixtures::inputJpgWithPortraitExif5())
        .unwrap()
        .flip(true)
        .unwrap()
        .rotate(90, None)
        .unwrap()
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_eq!(data[0..3], vec![55, 65, 31]);

    //Flip, rotate and resize ordering
    let data = Sharp::new_from_file(fixtures::inputJpgWithPortraitExif5())
        .unwrap()
        .flip(true)
        .unwrap()
        .rotate(90, None)
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(449),
            ..Default::default()
        })
        .unwrap()
        .raw(None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_eq!(data[0..3], vec![54, 64, 30]);

    //Resize after affine-based rotation does not overcompute
    Sharp::new(Inputs::new().create(Create {
        width: 4640,
        height: 2610,
        channels: 3,
        background: Colour::rgb(0, 0, 0),
        ..Default::default()
    }))
    .unwrap()
    .rotate(28, None)
    .unwrap()
    .resize(640, 360)
    .unwrap()
    .raw(None)
    .unwrap()
    .timeout(3)
    .to_buffer()
    .unwrap();

    //Rotate 90 then resize with inside fit
    let data = Sharp::new(Inputs::new().create(Create {
        width: 16,
        height: 8,
        channels: 3,
        background: Colour::rgb(255, 0, 0),
        ..Default::default()
    }))
    .unwrap()
    .rotate(90, None)
    .unwrap()
    .resize_with_opts(ResizeOptions {
        width: Some(6),
        fit: Some(Fit::Inside),
        ..Default::default()
    })
    .unwrap()
    .png(Some(PngOptions {
        compression_level: Some(0),
        ..Default::default()
    }))
    .unwrap()
    .to_buffer()
    .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(6, metadata.width);
    assert_eq!(12, metadata.height);

    //Resize with inside fit then rotate 90
    let data = Sharp::new(Inputs::new().create(Create {
        width: 16,
        height: 8,
        channels: 3,
        background: Colour::rgb(255, 0, 0),
        ..Default::default()
    }))
    .unwrap()
    .resize_with_opts(ResizeOptions {
        width: Some(6),
        fit: Some(Fit::Inside),
        ..Default::default()
    })
    .unwrap()
    .rotate(90, None)
    .unwrap()
    .png(Some(PngOptions {
        compression_level: Some(0),
        ..Default::default()
    }))
    .unwrap()
    .to_buffer()
    .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(3, metadata.width);
    assert_eq!(6, metadata.height);

    //Shrink-on-load with autoOrient
    let data = Sharp::new_from_file(fixtures::inputJpgWithLandscapeExif6())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(8),
            ..Default::default()
        })
        .unwrap()
        .auto_orient()
        .unwrap()
        .avif(Some(AvifOptions {
            effort: Some(0),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(8, metadata.width);
    assert_eq!(6, metadata.height);

    //Auto-orient and rotate 45
    let data = Sharp::new_from_file_with_opts(
        fixtures::inputJpgWithLandscapeExif2(),
        SharpOptions {
            auto_orient: Some(true),
            ..Default::default()
        },
    )
    .unwrap()
    .rotate(45, None)
    .unwrap()
    .to_buffer()
    .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(742, metadata.width);
    assert_eq!(742, metadata.height);

    //Auto-orient, extract and rotate 45
    let data = Sharp::new_from_file_with_opts(
        fixtures::inputJpgWithLandscapeExif2(),
        SharpOptions {
            auto_orient: Some(true),
            ..Default::default()
        },
    )
    .unwrap()
    .extract(Region {
        left: 20,
        top: 20,
        width: 200,
        height: 100,
    })
    .unwrap()
    .rotate(45, None)
    .unwrap()
    .to_buffer()
    .unwrap();
    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(212, metadata.width);
    assert_eq!(212, metadata.height);

    rs_vips::Vips::shutdown();
}
