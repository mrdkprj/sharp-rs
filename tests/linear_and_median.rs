mod fixtures;
use sharp::{
    input::{Create, CreateRaw, Inputs, SharpOptions},
    output::PngOptions,
    resize::ResizeOptions,
    Colour, Interpretation, Sharp,
};

#[test]
pub fn linear_and_median() {
    //applies linear levels adjustment w/o alpha ch
    let a = 255.0 / (203.0 - 70.0);
    let b = -70.0 * a;
    let data = Sharp::new_from_file(fixtures::inputJpgWithLowContrast())
        .unwrap()
        .linear(Some(vec![a]), Some(vec![b]))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("low-contrast-linear.jpg"), data, None);

    //applies slope level adjustment w/o alpha ch
    let data = Sharp::new_from_file(fixtures::inputJpgWithLowContrast())
        .unwrap()
        .linear(Some(vec![a]), None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("low-contrast-slope.jpg"), data, None);

    //applies offset level adjustment w/o alpha ch
    let data = Sharp::new_from_file(fixtures::inputJpgWithLowContrast())
        .unwrap()
        .linear(None, Some(vec![b]))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("low-contrast-offset.jpg"), data, None);

    //applies linear levels adjustment w alpha ch
    let data = Sharp::new_from_file(fixtures::inputPngOverlayLayer1())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(240),
            ..Default::default()
        })
        .unwrap()
        .linear(Some(vec![a]), Some(vec![b]))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("alpha-layer-1-fill-linear.png"), data, None);

    //applies linear levels adjustment to 16-bit w alpha ch
    let data = Sharp::new_from_file(fixtures::inputPngWithTransparency16bit())
        .unwrap()
        .linear(Some(vec![a]), Some(vec![b]))
        .unwrap()
        .png(Some(PngOptions {
            compression_level: Some(0),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("linear-16bit.png"), data, None);

    //applies slope level adjustment w alpha ch
    let data = Sharp::new_from_file(fixtures::inputPngOverlayLayer1())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(240),
            ..Default::default()
        })
        .unwrap()
        .linear(Some(vec![a]), None)
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("alpha-layer-1-fill-slope.png"), data, None);

    //applies offset level adjustment w alpha ch
    let data = Sharp::new_from_file(fixtures::inputPngOverlayLayer1())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(240),
            ..Default::default()
        })
        .unwrap()
        .linear(None, Some(vec![b]))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("alpha-layer-1-fill-offset.png"), data, None);

    //per channel level adjustment
    let data = Sharp::new_from_file(fixtures::inputWebP())
        .unwrap()
        .linear(Some(vec![0.25, 0.5, 0.75]), Some(vec![50.0, 100.0, 50.0]))
        .unwrap()
        .to_buffer()
        .unwrap();
    assert_similar!(fixtures::expected("linear-per-channel.jpg"), data, None);

    //output is integer, not float, RGB
    let data = Sharp::new(Inputs::new().create(Create {
        width: 1,
        height: 1,
        channels: 3,
        background: Colour::new(255, 0, 0, 1.0),
        ..Default::default()
    }))
    .unwrap()
    .linear(Some(vec![1.0]), Some(vec![0.0]))
    .unwrap()
    .tiff(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    let metadata = Sharp::new_from_buffer(data).unwrap().metadata().unwrap();
    assert_eq!(metadata.channels, 3);
    assert_eq!(metadata.depth, "uchar");

    //Median filter
    let raw = CreateRaw {
        width: 6,
        height: 6,
        channels: 1,
        ..Default::default()
    };
    let row: Vec<u8> = vec![0, 3, 15, 63, 127, 255];
    let take_count = row.len() * row.len();
    let input: Vec<u8> = row.clone().into_iter().cycle().take(take_count).collect();

    //default window (3x3)
    let data = Sharp::new_from_buffer_with_opts(
        input.clone(),
        SharpOptions {
            raw: Some(raw.clone()),
            ..Default::default()
        },
    )
    .unwrap()
    .median(None)
    .unwrap()
    .to_colourspace(Interpretation::BW)
    .raw(None)
    .unwrap()
    .to_buffer()
    .unwrap();
    assert_eq!(data[0..6], row.clone());

    //3x3 window
    let data = Sharp::new_from_buffer_with_opts(
        input.clone(),
        SharpOptions {
            raw: Some(raw.clone()),
            ..Default::default()
        },
    )
    .unwrap()
    .median(Some(3))
    .unwrap()
    .to_colourspace(Interpretation::BW)
    .raw(None)
    .unwrap()
    .to_buffer()
    .unwrap();
    assert_eq!(data[0..6], row.clone());

    //5x5 window
    let data = Sharp::new_from_buffer_with_opts(
        input.clone(),
        SharpOptions {
            raw: Some(raw.clone()),
            ..Default::default()
        },
    )
    .unwrap()
    .median(Some(5))
    .unwrap()
    .to_colourspace(Interpretation::BW)
    .raw(None)
    .unwrap()
    .to_buffer()
    .unwrap();
    assert_eq!(data[0..6], vec![0, 3, 15, 15, 63, 127]);
}
