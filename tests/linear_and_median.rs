mod fixtures;
use sharp::{
    input::{Create, CreateRaw, SharpOptions},
    output::PngOptions,
    Colour, Interpretation, Sharp,
};

#[test]
pub fn linear_and_median() {
    //applies linear levels adjustment w/o alpha ch
    let a = 255.0 / 203.0 - 70.0;
    let b = -70.0 * a;
    Sharp::new_from_file(fixtures::inputJpgWithLowContrast())
        .unwrap()
        .linear(Some(vec![a]), Some(vec![b]))
        .unwrap()
        .to_buffer()
        .unwrap();

    //applies slope level adjustment w/o alpha ch
    Sharp::new_from_file(fixtures::inputJpgWithLowContrast())
        .unwrap()
        .linear(Some(vec![a]), None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //applies offset level adjustment w/o alpha ch
    Sharp::new_from_file(fixtures::inputJpgWithLowContrast())
        .unwrap()
        .linear(None, Some(vec![b]))
        .unwrap()
        .to_buffer()
        .unwrap();

    //applies linear levels adjustment w alpha ch
    Sharp::new_from_file(fixtures::inputJpgWithLowContrast())
        .unwrap()
        .resize(240, 240)
        .unwrap()
        .linear(Some(vec![a]), Some(vec![b]))
        .unwrap()
        .to_buffer()
        .unwrap();

    //applies linear levels adjustment to 16-bit w alpha ch
    Sharp::new_from_file(fixtures::inputJpgWithLowContrast())
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

    //applies slope level adjustment w alpha ch
    Sharp::new_from_file(fixtures::inputJpgWithLowContrast())
        .unwrap()
        .resize(240, 240)
        .unwrap()
        .linear(Some(vec![a]), None)
        .unwrap()
        .to_buffer()
        .unwrap();

    //applies slope level adjustment w alpha ch
    Sharp::new_from_file(fixtures::inputJpgWithLowContrast())
        .unwrap()
        .resize(240, 240)
        .unwrap()
        .linear(None, Some(vec![b]))
        .unwrap()
        .to_buffer()
        .unwrap();

    //per channel level adjustment
    Sharp::new_from_file(fixtures::inputWebP())
        .unwrap()
        .linear(Some(vec![0.25, 0.5, 0.75]), Some(vec![50.0, 100.0, 50.0]))
        .unwrap()
        .to_buffer()
        .unwrap();

    //output is integer, not float, RGB
    Sharp::new(SharpOptions {
        create: Some(Create {
            width: 1,
            height: 1,
            channels: 3,
            background: Colour::new(255, 0, 0, 1.0),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
    .linear(Some(vec![1.0]), Some(vec![0.0]))
    .unwrap()
    .tiff(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    //Median filter
    let raw = CreateRaw {
        width: 6,
        height: 6,
        channels: 1,
        ..Default::default()
    };
    let row: Vec<u8> = vec![0, 3, 15, 63, 127, 255];
    let take_count = row.len() * row.len();
    let input: Vec<u8> = row.into_iter().cycle().take(take_count).collect();
    //default window (3x3)
    Sharp::new_from_buffer_with_opts(
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

    //3x3 window
    Sharp::new_from_buffer_with_opts(
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

    //5x5 window
    Sharp::new_from_buffer_with_opts(
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
}
