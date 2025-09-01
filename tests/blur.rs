mod fixtures;
use sharp::{operation::BlurOptions, Precision, Sharp};

#[test]
pub fn blur() {
    //'specific radius 1'
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .blur(Some(BlurOptions {
            sigma: 1.0,
            ..Default::default()
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg".to_string(), info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("blur-1.jpg"), data, None);

    //'specific radius 10
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .blur(Some(BlurOptions {
            sigma: 10.0,
            ..Default::default()
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg".to_string(), info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("blur-10.jpg"), data, None);

    //'specific radius 0.3
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .blur(Some(BlurOptions {
            sigma: 0.3,
            ..Default::default()
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg".to_string(), info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("blur-0.3.jpg"), data, None);

    //mild blur
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .blur(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg".to_string(), info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);
    assert_similar!(fixtures::expected("blur-mild.jpg"), data, None);

    //blurred image is smaller than non-blurred
    let (not_blurred, not_blurred_info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .blur(Some(BlurOptions {
            sigma: 0.0,
            ..Default::default()
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!not_blurred.is_empty());
    assert_eq!("jpeg".to_string(), not_blurred_info.format);
    assert_eq!(320, not_blurred_info.width);
    assert_eq!(240, not_blurred_info.height);
    let (blurred, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .blur(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert!(!blurred.is_empty());
    assert!(blurred.len() < not_blurred.len());
    assert_eq!("jpeg".to_string(), info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);

    //specific radius 10 and precision approximate
    let approximate = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .blur(Some(BlurOptions {
            sigma: 10.0,
            precision: Some(Precision::Approximate),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    let integer = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .blur(Some(BlurOptions {
            sigma: 10.0,
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_ne!(approximate, integer);
    assert_similar!(fixtures::expected("blur-10.jpg"), approximate, None);

    //specific radius 10 and minAmplitude 0.01
    let min_amplitude_low = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .blur(Some(BlurOptions {
            sigma: 10.0,
            min_amplitude: Some(0.01),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    let min_amplitude_default = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .blur(Some(BlurOptions {
            sigma: 10.0,
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_ne!(min_amplitude_low, min_amplitude_default);
    assert_similar!(fixtures::expected("blur-10.jpg"), min_amplitude_low, None);

    rs_vips::Vips::shutdown();
}
