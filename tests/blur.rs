mod fixtures;
use sharp::{operation::BlurOptions, Precision, Sharp};

#[test]
pub fn blur() {
    //'specific radius 1'
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .blur(Some(BlurOptions {
            sigma: 1.0,
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //'specific radius 10
    Sharp::new_from_file(fixtures::inputJpg())
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

    //'specific radius 0.3
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .blur(Some(BlurOptions {
            sigma: 0.3,
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //mild blur
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .blur(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //specific radius 10 and precision approximate
    Sharp::new_from_file(fixtures::inputJpg())
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

    //specific radius 10 and minAmplitude 0.01
    Sharp::new_from_file(fixtures::inputJpg())
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
}
