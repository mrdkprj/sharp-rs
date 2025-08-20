use sharp::{
    input::{CreateRaw, SharpOptions},
    Sharp,
};
use std::fs;
mod fixtures;

#[test]
fn recomb() {
    let sepia =
        vec![vec![0.3588, 0.7044, 0.1368], vec![0.299, 0.587, 0.114], vec![0.2392, 0.4696, 0.0912]];
    //applies a sepia filter using recomb
    let output = fixtures::output("output.recomb-sepia.jpg");
    let (data, info) = Sharp::new_from_file(fixtures::inputJpgWithLandscapeExif1())
        .unwrap()
        .recomb(sepia.clone())
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(600, info.width);
    assert_eq!(450, info.height);
    fs::write(output.clone(), data).unwrap();
    assert_max_colour_distance!(output, fixtures::expected("Landscape_1-recomb-sepia.jpg"), 17.0);

    //applies a sepia filter using recomb to an PNG with Alpha
    let output = fixtures::output("output.recomb-sepia.png");
    let (data, info) = Sharp::new_from_file(fixtures::inputPngAlphaPremultiplicationSmall())
        .unwrap()
        .recomb(sepia.clone())
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(1024, info.width);
    assert_eq!(768, info.height);
    fs::write(output.clone(), data).unwrap();
    assert_max_colour_distance!(output, fixtures::expected("alpha-recomb-sepia.png"), 17.0);

    //recomb with a single channel input
    let buf = vec![0u8; 64];
    let (_, info) = Sharp::new_from_buffer_with_opts(
        buf,
        SharpOptions {
            raw: Some(CreateRaw {
                width: 8,
                height: 8,
                channels: 1,
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .unwrap()
    .recomb(sepia.clone())
    .unwrap()
    .to_buffer_with_info()
    .unwrap();
    assert_eq!(3, info.channels);

    //applies a different sepia filter using recomb
    let output = fixtures::output("output.recomb-sepia2.jpg");
    let (data, info) = Sharp::new_from_file(fixtures::inputJpgWithLandscapeExif1())
        .unwrap()
        .recomb(vec![
            vec![0.393, 0.769, 0.189],
            vec![0.349, 0.686, 0.168],
            vec![0.272, 0.534, 0.131],
        ])
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(600, info.width);
    assert_eq!(450, info.height);
    fs::write(output.clone(), data).unwrap();
    assert_max_colour_distance!(output, fixtures::expected("Landscape_1-recomb-sepia2.jpg"), 17.0);

    //increases the saturation of the image
    let saturation_level = 1.0;
    let output = fixtures::output("output.recomb-saturation.jpg");
    let (data, info) = Sharp::new_from_file(fixtures::inputJpgWithLandscapeExif1())
        .unwrap()
        .recomb(vec![
            vec![
                saturation_level + 1.0 - 0.2989,
                -0.587 * saturation_level,
                -0.114 * saturation_level,
            ],
            vec![
                -0.2989 * saturation_level,
                saturation_level + 1.0 - 0.587,
                -0.114 * saturation_level,
            ],
            vec![
                -0.2989 * saturation_level,
                -0.587 * saturation_level,
                saturation_level + 1.0 - 0.114,
            ],
        ])
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(600, info.width);
    assert_eq!(450, info.height);
    fs::write(output.clone(), data).unwrap();
    assert_max_colour_distance!(
        output,
        fixtures::expected("Landscape_1-recomb-saturation.jpg"),
        37.0
    );

    //applies opacity 30% to the image
    let output = fixtures::output("output.recomb-opacity.png");
    let (data, info) = Sharp::new_from_file(fixtures::inputPngWithTransparent())
        .unwrap()
        .recomb(vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.3],
        ])
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(48, info.width);
    assert_eq!(48, info.height);
    fs::write(output.clone(), data).unwrap();
    assert_max_colour_distance!(output, fixtures::expected("d-opacity-30.png"), 17.0);
}
