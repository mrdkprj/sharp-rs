mod fixtures;
use sharp::{
    input::RotateOptions,
    operation::{AffineOptions, Interpolators},
    resize::Region,
    Colour, Sharp,
};

#[test]
pub fn affine() {
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .affine(vec![vec![1.0, 0.0], vec![0.0, 1.0]], None)
        .unwrap()
        .to_buffer()
        .unwrap();

    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .affine(vec![vec![0.2, 0.0], vec![0.0, 1.5]], None)
        .unwrap()
        .to_buffer()
        .unwrap();

    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(500, 500)
        .unwrap()
        .affine(vec![vec![0.5, 1.0], vec![0.0, 1.0]], None)
        .unwrap()
        .to_buffer()
        .unwrap();

    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .extract(Region {
            left: 300,
            top: 300,
            width: 600,
            height: 600,
        })
        .unwrap()
        .affine(vec![vec![0.3, 0.0], vec![-0.5, 1.3]], None)
        .unwrap()
        .to_buffer()
        .unwrap();

    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .rotate(90, None)
        .unwrap()
        .affine(vec![vec![-1.2, 0.0], vec![0.0, -1.2]], None)
        .unwrap()
        .to_buffer()
        .unwrap();

    Sharp::new_from_file(fixtures::inputJpg())
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
                // blue
                background: Colour::from_hex(0),
            }),
        )
        .unwrap()
        .affine(
            vec![vec![2.0, 1.0], vec![2.0, -0.5]],
            Some(AffineOptions {
                // red
                background: Some(Colour::from_hex(0)),
                ..Default::default()
            }),
        )
        .unwrap()
        .to_buffer()
        .unwrap();

    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .rotate(180, None)
        .unwrap()
        .affine(
            vec![vec![-2.0, 1.5], vec![-1.0, -2.0]],
            Some(AffineOptions {
                // yellow
                background: Some(Colour::from_hex(0)),
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

    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .affine(
            vec![vec![-2.0, 1.5], vec![-1.0, -2.0]],
            Some(AffineOptions {
                interpolator: Some(Interpolators::Nearest),
                ..Default::default()
            }),
        )
        .unwrap()
        .to_buffer()
        .unwrap();
}
