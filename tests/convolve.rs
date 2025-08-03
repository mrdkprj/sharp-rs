mod fixtures;
use sharp::{operation::KernelOptions, Sharp};

#[test]
pub fn convolve() {
    //specific convolution kernel 1
    Sharp::new_from_file(fixtures::inputPngStripesV())
        .unwrap()
        .convolve(KernelOptions {
            width: 3,
            height: 3,
            scale: Some(50.0),
            offset: Some(0.0),
            kernel: vec![10.0, 20.0, 10.0, 0.0, 0.0, 0.0, 10.0, 20.0, 10.0],
        })
        .unwrap()
        .to_buffer()
        .unwrap();

    //specific convolution kernel 2
    Sharp::new_from_file(fixtures::inputPngStripesV())
        .unwrap()
        .convolve(KernelOptions {
            width: 3,
            height: 3,
            scale: Some(50.0),
            offset: Some(0.0),
            kernel: vec![1.0, 0.0, 1.0, 2.0, 0.0, 2.0, 1.0, 0.0, 1.0],
        })
        .unwrap()
        .to_buffer()
        .unwrap();

    //horizontal Sobel operator
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .convolve(KernelOptions {
            width: 3,
            height: 3,
            kernel: vec![-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0],
            ..Default::default()
        })
        .unwrap()
        .to_buffer()
        .unwrap();
}
