mod fixtures;
use sharp::{operation::ClaheOptions, Sharp};

#[test]
pub fn clahe() {
    //width 5 width 5 maxSlope 0
    Sharp::new_from_file(fixtures::inputJpgClahe())
        .unwrap()
        .clahe(Some(ClaheOptions {
            width: 5,
            height: 5,
            max_slope: Some(0),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //width 5 width 5 maxSlope 5
    Sharp::new_from_file(fixtures::inputJpgClahe())
        .unwrap()
        .clahe(Some(ClaheOptions {
            width: 5,
            height: 5,
            max_slope: Some(5),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //width 11 width 25 maxSlope 14
    Sharp::new_from_file(fixtures::inputJpgClahe())
        .unwrap()
        .clahe(Some(ClaheOptions {
            width: 11,
            height: 25,
            max_slope: Some(14),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //width 50 width 50 maxSlope 0
    Sharp::new_from_file(fixtures::inputJpgClahe())
        .unwrap()
        .clahe(Some(ClaheOptions {
            width: 50,
            height: 50,
            max_slope: Some(0),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //width 50 width 50 maxSlope 14
    Sharp::new_from_file(fixtures::inputJpgClahe())
        .unwrap()
        .clahe(Some(ClaheOptions {
            width: 50,
            height: 50,
            max_slope: Some(14),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //'width 100 width 50 maxSlope 3
    Sharp::new_from_file(fixtures::inputJpgClahe())
        .unwrap()
        .clahe(Some(ClaheOptions {
            width: 100,
            height: 50,
            max_slope: Some(3),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //width 100 width 100 maxSlope 0
    Sharp::new_from_file(fixtures::inputJpgClahe())
        .unwrap()
        .clahe(Some(ClaheOptions {
            width: 100,
            height: 100,
            max_slope: Some(0),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //uses default maxSlope of 3
    Sharp::new_from_file(fixtures::inputJpgClahe())
        .unwrap()
        .clahe(Some(ClaheOptions {
            width: 100,
            height: 50,
            max_slope: None,
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
}
