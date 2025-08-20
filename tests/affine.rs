mod fixtures;
use sharp::{
    input::RotateOptions,
    operation::{AffineOptions, Interpolators},
    resize::Region,
    Colour, Sharp,
};

#[test]
pub fn affine() {
    //Applies identity matrix
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .affine(vec![vec![1.0, 0.0], vec![0.0, 1.0]], None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::inputJpg(), data, None);

    //Applies resize affine matrix'
    let input_width: f64 = 2725.0 * 0.2;
    let input_height: f64 = 2225.0 * 1.5;
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .affine(vec![vec![0.2, 0.0], vec![0.0, 1.5]], None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_similar!(fixtures::inputJpg(), data, None);
    assert!(info.width as f64 == input_width.ceil());
    assert!(info.height as f64 == input_height.ceil());

    //Resizes and applies affine transform
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(500, 500)
        .unwrap()
        .affine(vec![vec![0.5, 1.0], vec![1.0, 0.5]], None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("affine-resize-expected.jpg"), data, None);

    //Extracts and applies affine transform
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .extract(Region {
            left: 300,
            top: 300,
            width: 600,
            height: 600,
        })
        .unwrap()
        .affine(vec![vec![0.3, 0.0, -0.5, 0.3]], None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("affine-extract-expected.jpg"), data, None);

    //Rotates and applies affine transform
    let data = Sharp::new_from_file(fixtures::inputJpg320x240())
        .unwrap()
        .rotate(90, None)
        .unwrap()
        .affine(vec![vec![-1.2, 0.0], vec![0.0, -1.2]], None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("affine-rotate-expected.jpg"), data, None);

    //Extracts, rotates and applies affine transform
    let data = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .extract(Region {
            left: 1000,
            top: 1000,
            width: 200,
            height: 200,
        })
        .unwrap()
        .rotate(
            45,
            Some(RotateOptions {
                background: Colour::new(0, 0, 255, 1.0),
            }),
        )
        .unwrap()
        .affine(
            vec![vec![2.0, 1.0], vec![2.0, -0.5]],
            Some(AffineOptions {
                background: Some(Colour::new(255, 0, 0, 1.0)),
                ..Default::default()
            }),
        )
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("affine-extract-rotate-expected.jpg"), data, None);

    //Applies affine transform with background color
    let data = Sharp::new_from_file(fixtures::inputJpg320x240())
        .unwrap()
        .affine(
            vec![vec![-1.5, 1.2], vec![-1.0, 1.0]],
            Some(AffineOptions {
                background: Some(Colour::new(255, 0, 0, 1.0)),
                ..Default::default()
            }),
        )
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("affine-background-expected.jpg"), data, None);

    //Applies affine transform with background color and output offsets
    let data = Sharp::new_from_file(fixtures::inputJpg320x240())
        .unwrap()
        .rotate(180, None)
        .unwrap()
        .affine(
            vec![vec![-2.0, 1.5], vec![-1.0, 2.0]],
            Some(AffineOptions {
                background: Some(Colour::new(0, 0, 255, 1.0)),
                odx: Some(40.0),
                ody: Some(-100.0),
                ..Default::default()
            }),
        )
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(
        fixtures::expected("affine-background-output-offsets-expected.jpg"),
        data,
        None
    );

    //Applies affine transform with background color and all offsets
    let data = Sharp::new_from_file(fixtures::inputJpg320x240())
        .unwrap()
        .rotate(180, None)
        .unwrap()
        .affine(
            vec![vec![-1.2, 1.8], vec![-1.0, 2.0]],
            Some(AffineOptions {
                background: Some(Colour::from_hex(0xffff00)),
                idx: Some(10.0),
                idy: Some(-40.0),
                odx: Some(10.0),
                ody: Some(-50.0),
                ..Default::default()
            }),
        )
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("affine-background-all-offsets-expected.jpg"), data, None);

    //Performs 2x upscale
    let input_width = 320 * 2;
    let input_height = 240 * 2;
    [
        Interpolators::Bicubic,
        Interpolators::Bilinear,
        Interpolators::LocallyBoundedBicubic,
        Interpolators::Nearest,
        Interpolators::Nohalo,
        Interpolators::VertexSplitQuadraticBasisSpline,
    ]
    .into_iter()
    .for_each(|interp| {
        let (data, info) = Sharp::new_from_file(fixtures::inputJpg320x240())
            .unwrap()
            .affine(
                vec![vec![2.0, 0.0], vec![0.0, 2.0]],
                Some(AffineOptions {
                    interpolator: Some(interp.clone()),
                    ..Default::default()
                }),
            )
            .unwrap()
            .to_buffer_with_info()
            .unwrap();

        assert_eq!(info.width, input_width);
        assert_eq!(info.height, input_height);
        assert_similar!(
            fixtures::expected(&format!(
                "affine-{}-2x-upscale-expected.jpg",
                interp_to_string(interp)
            )),
            data,
            None
        );
    })
}

fn interp_to_string(interp: Interpolators) -> String {
    match interp {
        Interpolators::Bicubic => "bicubic",
        Interpolators::Bilinear => "bilinear",
        Interpolators::LocallyBoundedBicubic => "lbb",
        Interpolators::Nearest => "nearest",
        Interpolators::Nohalo => "nohalo",
        Interpolators::VertexSplitQuadraticBasisSpline => "vsqbs",
    }
    .to_string()
}
