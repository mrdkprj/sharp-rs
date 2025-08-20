mod fixtures;
use sharp::{operation::KernelOptions, Sharp};

#[test]
pub fn convolve() {
    //specific convolution kernel 1
    let (data, info) = Sharp::new_from_file(fixtures::inputPngStripesV())
        .unwrap()
        .convolve(KernelOptions {
            width: 3,
            height: 3,
            scale: Some(50.0),
            offset: Some(0.0),
            kernel: vec![10.0, 20.0, 10.0, 0.0, 0.0, 0.0, 10.0, 20.0, 10.0],
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("conv-1.png"), data, None);

    //specific convolution kernel 2
    let (data, info) = Sharp::new_from_file(fixtures::inputPngStripesH())
        .unwrap()
        .convolve(KernelOptions {
            width: 3,
            height: 3,
            kernel: vec![1.0, 0.0, 1.0, 2.0, 0.0, 2.0, 1.0, 0.0, 1.0],
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("conv-2.png"), data, None);

    //horizontal Sobel operator
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
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
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("conv-sobel-horizontal.jpg"), data, None);
}
