mod fixtures;
use sharp::Sharp;

#[test]
pub fn dilate_erode() {
    //dilate 1 png
    let (data, info) = Sharp::new_from_file(fixtures::inputPngDotAndLines())
        .unwrap()
        .dilate(Some(1))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(100, info.width);
    assert_eq!(100, info.height);
    assert_similar!(fixtures::expected("dilate-1.png"), data, None);

    //dilate 1 png - default width
    let (data, info) = Sharp::new_from_file(fixtures::inputPngDotAndLines())
        .unwrap()
        .dilate(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(100, info.width);
    assert_eq!(100, info.height);
    assert_similar!(fixtures::expected("dilate-1.png"), data, None);

    //erode 1 png
    let (data, info) = Sharp::new_from_file(fixtures::inputPngDotAndLines())
        .unwrap()
        .erode(Some(1))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(100, info.width);
    assert_eq!(100, info.height);
    assert_similar!(fixtures::expected("erode-1.png"), data, None);

    //erode 1 png - default width
    let (data, info) = Sharp::new_from_file(fixtures::inputPngDotAndLines())
        .unwrap()
        .erode(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("png", info.format);
    assert_eq!(100, info.width);
    assert_eq!(100, info.height);
    assert_similar!(fixtures::expected("erode-1.png"), data, None);

    rs_vips::Vips::shutdown();
}
