use sharp::{
    input::{CreateRaw, SharpOptions},
    Sharp,
};
mod fixtures;

#[test]
fn recomb() {
    let sepia =
        vec![vec![0.3588, 0.7044, 0.1368], vec![0.299, 0.587, 0.114], vec![0.2392, 0.4696, 0.0912]];
    //applies a sepia filter using recomb
    Sharp::new_from_file(fixtures::inputJpgWithLandscapeExif1())
        .unwrap()
        .recomb(sepia.clone())
        .unwrap()
        .to_buffer()
        .unwrap();

    //applies a sepia filter using recomb to an PNG with Alpha
    Sharp::new_from_file(fixtures::inputPngAlphaPremultiplicationSmall())
        .unwrap()
        .recomb(sepia.clone())
        .unwrap()
        .to_buffer()
        .unwrap();

    //recomb with a single channel input
    let mut buf = vec![0; 64];
    buf.fill(0u8);
    Sharp::new_from_buffer_with_opts(
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
    .to_buffer()
    .unwrap();

    //applies a different sepia filter using recomb
    Sharp::new_from_file(fixtures::inputJpgWithLandscapeExif1())
        .unwrap()
        .recomb(vec![
            vec![0.393, 0.769, 0.189],
            vec![0.349, 0.686, 0.168],
            vec![0.272, 0.534, 0.131],
        ])
        .unwrap()
        .to_buffer()
        .unwrap();

    //increases the saturation of the image
    let saturation_level = 1.0;
    Sharp::new_from_file(fixtures::inputJpgWithLandscapeExif1())
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
        .to_buffer()
        .unwrap();

    //applies opacity 30% to the image
    Sharp::new_from_file(fixtures::inputPngWithTransparent())
        .unwrap()
        .recomb(vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.3],
        ])
        .unwrap()
        .to_buffer()
        .unwrap();
}
