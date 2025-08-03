mod fixtures;
use sharp::Sharp;

#[test]
pub fn dilate_erode() {
    //dilate 1 png
    Sharp::new_from_file(fixtures::inputPngDotAndLines())
        .unwrap()
        .dilate(Some(1))
        .unwrap()
        .to_buffer()
        .unwrap();

    //dilate 1 png - default width
    Sharp::new_from_file(fixtures::inputPngDotAndLines())
        .unwrap()
        .dilate(None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //erode 1 png
    Sharp::new_from_file(fixtures::inputPngDotAndLines())
        .unwrap()
        .erode(Some(1))
        .unwrap()
        .to_buffer()
        .unwrap();

    //erode 1 png - default width
    Sharp::new_from_file(fixtures::inputPngDotAndLines())
        .unwrap()
        .erode(None)
        .unwrap()
        .to_buffer()
        .unwrap();
}
