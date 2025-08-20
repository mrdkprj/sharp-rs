mod fixtures;
use sharp::{operation::ClaheOptions, Sharp};

#[test]
pub fn clahe() {
    //width 5 width 5 maxSlope 0
    let data = Sharp::new_from_file(fixtures::inputJpgClahe())
        .unwrap()
        .clahe(Some(ClaheOptions {
            width: 5,
            height: 5,
            max_slope: Some(0),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("clahe-5-5-0.jpg"), data, Some(10));

    //width 5 width 5 maxSlope 5
    let data = Sharp::new_from_file(fixtures::inputJpgClahe())
        .unwrap()
        .clahe(Some(ClaheOptions {
            width: 5,
            height: 5,
            max_slope: Some(5),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("clahe-5-5-5.jpg"), data, None);

    //width 11 width 25 maxSlope 14
    let data = Sharp::new_from_file(fixtures::inputJpgClahe())
        .unwrap()
        .clahe(Some(ClaheOptions {
            width: 11,
            height: 25,
            max_slope: Some(14),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("clahe-11-25-14.jpg"), data, None);

    //width 50 width 50 maxSlope 0
    let data = Sharp::new_from_file(fixtures::inputJpgClahe())
        .unwrap()
        .clahe(Some(ClaheOptions {
            width: 50,
            height: 50,
            max_slope: Some(0),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("clahe-50-50-0.jpg"), data, None);

    //width 50 width 50 maxSlope 14
    let data = Sharp::new_from_file(fixtures::inputJpgClahe())
        .unwrap()
        .clahe(Some(ClaheOptions {
            width: 50,
            height: 50,
            max_slope: Some(14),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("clahe-50-50-14.jpg"), data, None);

    //'width 100 width 50 maxSlope 3
    let data = Sharp::new_from_file(fixtures::inputJpgClahe())
        .unwrap()
        .clahe(Some(ClaheOptions {
            width: 100,
            height: 50,
            max_slope: Some(3),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("clahe-100-50-3.jpg"), data, None);

    //width 100 width 100 maxSlope 0
    let data = Sharp::new_from_file(fixtures::inputJpgClahe())
        .unwrap()
        .clahe(Some(ClaheOptions {
            width: 100,
            height: 100,
            max_slope: Some(0),
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("clahe-100-100-0.jpg"), data, None);

    //uses default maxSlope of 3
    let data = Sharp::new_from_file(fixtures::inputJpgClahe())
        .unwrap()
        .clahe(Some(ClaheOptions {
            width: 100,
            height: 50,
            max_slope: None,
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("clahe-100-50-3.jpg"), data, None);
}
