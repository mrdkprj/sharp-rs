#![allow(non_snake_case)]
use nonstd::fs;
use sharp::{
    output::{JpegOptions, PngOptions, TileOptions, WebpOptions},
    ForeignDzContainer, ForeignDzDepth, ForeignDzLayout, Sharp,
};
use std::path::Path;
mod fixtures;

#[test]
fn tile() {
    //Deep Zoom layout
    println!("{}", line!());
    let directory = fixtures::output("output.dzi_files");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .to_file_with_info(fixtures::output("output.dzi"))
        .unwrap();
    assert_eq!("dz", info.format);
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    assertDeepZoomTiles(&directory, 256, 13);

    //Deep Zoom layout with custom size+overlap
    println!("{}", line!());
    let directory = fixtures::output("output.512.dzi_files");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(512),
            overlap: Some(16),
            ..Default::default()
        }))
        .unwrap()
        .to_file_with_info(fixtures::output("output.512.dzi"))
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    assertDeepZoomTiles(&directory, 512 + (2 * 16), 13);
    assertTileOverlap(&directory, 512);

    //Deep Zoom layout with custom size+angle
    println!("{}", line!());
    let directory = fixtures::output("output.512_90.dzi_files");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(512),
            angle: Some(90),
            ..Default::default()
        }))
        .unwrap()
        .to_file_with_info(fixtures::output("output.512_90.dzi"))
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    assertDeepZoomTiles(&directory, 512, 13);
    let tile = directory.join("10").join("0_1.jpeg");
    let metadata = Sharp::new_from_file(tile).unwrap().metadata().unwrap();
    assert!(metadata.width == 512);
    assert!(metadata.height == 170);

    //Deep Zoom layout with depth of one
    println!("{}", line!());
    let directory = fixtures::output("output.512_depth_one.dzi_files");
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(512),
            depth: Some(sharp::ForeignDzDepth::One),
            ..Default::default()
        }))
        .unwrap()
        .to_file(fixtures::output("output.512_depth_one.dzi"))
        .unwrap();
    assertDeepZoomTiles(&directory, 512, 1);

    //Deep Zoom layout with depth of onepixel
    println!("{}", line!());
    let directory = fixtures::output("output.512_depth_onepixel.dzi_files");
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(512),
            depth: Some(sharp::ForeignDzDepth::Onepixel),
            ..Default::default()
        }))
        .unwrap()
        .to_file(fixtures::output("output.512_depth_onepixel.dzi"))
        .unwrap();
    assertDeepZoomTiles(&directory, 512, 13);

    //Deep Zoom layout with depth of onetile
    println!("{}", line!());
    let directory = fixtures::output("output.256_depth_onetile.dzi_files");
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(256),
            depth: Some(sharp::ForeignDzDepth::Onetile),
            ..Default::default()
        }))
        .unwrap()
        .to_file(fixtures::output("output.256_depth_onetile.dzi"))
        .unwrap();
    assertDeepZoomTiles(&directory, 256, 5);

    //Deep Zoom layout with skipBlanks
    println!("{}", line!());
    let directory = fixtures::output("output.256_skip_blanks.dzi_files");
    Sharp::new_from_file(fixtures::inputJpgOverlayLayer2())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(256),
            skip_blanks: Some(0),
            ..Default::default()
        }))
        .unwrap()
        .to_file(fixtures::output("output.256_skip_blanks.dzi"))
        .unwrap();
    let whiteTilePath = directory.join("11").join("0_0.jpeg");
    assert!(!whiteTilePath.exists());
    assertDeepZoomTiles(&directory, 256, 12);

    //Zoomify layout
    println!("{}", line!());
    let directory = fixtures::output("output.zoomify.dzi");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Zoomify),
            ..Default::default()
        }))
        .unwrap()
        .to_file_with_info(fixtures::output("output.zoomify.dzi"))
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    let stat = fs::stat(directory.join("ImageProperties.xml")).unwrap();
    assert!(stat.is_file);
    assert!(stat.size > 0);

    //Zoomify layout with depth one
    println!("{}", line!());
    let directory = fixtures::output("output.zoomify.depth_one.dzi");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(256),
            layout: Some(ForeignDzLayout::Zoomify),
            depth: Some(ForeignDzDepth::One),
            ..Default::default()
        }))
        .unwrap()
        .to_file_with_info(directory.clone())
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    assertZoomifyTiles(&directory, 256, 1);

    //Zoomify layout with depth onetile
    println!("{}", line!());
    let directory = fixtures::output("output.zoomify.depth_onetile.dzi");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(256),
            layout: Some(ForeignDzLayout::Zoomify),
            depth: Some(ForeignDzDepth::Onetile),
            ..Default::default()
        }))
        .unwrap()
        .to_file_with_info(directory.clone())
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    assertZoomifyTiles(&directory, 256, 5);

    //Zoomify layout with depth onepixel
    println!("{}", line!());
    let directory = fixtures::output("output.zoomify.depth_onepixel.dzi");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(256),
            layout: Some(ForeignDzLayout::Zoomify),
            depth: Some(ForeignDzDepth::Onepixel),
            ..Default::default()
        }))
        .unwrap()
        .to_file_with_info(directory.clone())
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    assertZoomifyTiles(&directory, 256, 13);

    //Zoomify layout with skip blanks
    println!("{}", line!());
    let directory = fixtures::output("output.zoomify.skipBlanks.dzi");
    let info = Sharp::new_from_file(fixtures::inputJpgOverlayLayer2())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(256),
            layout: Some(ForeignDzLayout::Zoomify),
            skip_blanks: Some(0),
            ..Default::default()
        }))
        .unwrap()
        .to_file_with_info(directory.clone())
        .unwrap();
    let whiteTilePath = directory.join("TileGroup0").join("2-0-0.jpg");
    assert!(!whiteTilePath.exists());
    assert_eq!(2048, info.width);
    assert_eq!(1536, info.height);
    assert_eq!(3, info.channels);
    assertZoomifyTiles(&directory, 256, 4);

    //Google layout
    let directory = fixtures::output("output.google.dzi");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Google),
            ..Default::default()
        }))
        .unwrap()
        .to_file_with_info(directory.clone())
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    let stat = fs::stat(directory.join("0").join("0").join("0.jpg")).unwrap();
    assert!(stat.is_file);
    assert!(stat.size > 0);

    //Google layout with jpeg format
    let directory = fixtures::output("output.jpg.google.dzi");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .jpeg(Some(JpegOptions {
            quality: Some(1),
            ..Default::default()
        }))
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Google),
            ..Default::default()
        }))
        .unwrap()
        .to_file_with_info(directory.clone())
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    let sample = directory.join("0").join("0").join("0.jpg");
    let metadata = Sharp::new_from_file(sample.clone()).unwrap().metadata().unwrap();
    assert_eq!("jpeg", metadata.format);
    assert_eq!("srgb", metadata.space);
    assert_eq!(3, metadata.channels);
    assert!(!metadata.has_profile);
    assert!(!metadata.has_alpha);
    assert_eq!(256, metadata.width);
    assert_eq!(256, metadata.height);
    let stat = fs::stat(sample).unwrap();
    assert!(stat.size < 2000);

    //Google layout with png format
    let directory = fixtures::output("output.png.google.dzi");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .png(Some(PngOptions {
            compression_level: Some(0),
            ..Default::default()
        }))
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Google),
            ..Default::default()
        }))
        .unwrap()
        .to_file_with_info(directory.clone())
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    let sample = directory.join("0").join("0").join("0.png");
    let metadata = Sharp::new_from_file(sample.clone()).unwrap().metadata().unwrap();
    assert_eq!("png", metadata.format);
    assert_eq!("srgb", metadata.space);
    assert_eq!(3, metadata.channels);
    assert!(!metadata.has_profile);
    assert!(!metadata.has_alpha);
    assert_eq!(256, metadata.width);
    assert_eq!(256, metadata.height);
    let stat = fs::stat(sample).unwrap();
    assert!(stat.size > 44000);

    //Google layout with webp format
    let directory = fixtures::output("output.webp.google.dzi");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .webp(Some(WebpOptions {
            quality: Some(1),
            effort: Some(0),
            ..Default::default()
        }))
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Google),
            ..Default::default()
        }))
        .unwrap()
        .to_file_with_info(directory.clone())
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    let sample = directory.join("0").join("0").join("0.webp");
    let metadata = Sharp::new_from_file(sample.clone()).unwrap().metadata().unwrap();
    assert_eq!("webp", metadata.format);
    assert_eq!("srgb", metadata.space);
    assert_eq!(3, metadata.channels);
    assert!(!metadata.has_profile);
    assert!(!metadata.has_alpha);
    assert_eq!(256, metadata.width);
    assert_eq!(256, metadata.height);
    let stat = fs::stat(sample).unwrap();
    assert!(stat.size < 2000);

    //Google layout with depth one
    println!("{}", line!());
    let directory = fixtures::output("output.google_depth_one.dzi");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Google),
            depth: Some(ForeignDzDepth::One),
            size: Some(256),
            ..Default::default()
        }))
        .unwrap()
        .to_file_with_info(directory.clone())
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    assertGoogleTiles(&directory, 256, 1);

    //Google layout with depth onetile
    println!("{}", line!());
    let directory = fixtures::output("output.google_depth_onetile.dzi");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Google),
            depth: Some(ForeignDzDepth::Onetile),
            size: Some(256),
            ..Default::default()
        }))
        .unwrap()
        .to_file_with_info(directory.clone())
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    assertGoogleTiles(&directory, 256, 5);

    //Google layout with default skip Blanks
    println!("{}", line!());
    let directory = fixtures::output("output.google_depth_skipBlanks.dzi");
    let info = Sharp::new_from_file(fixtures::inputPng())
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Google),
            size: Some(256),
            ..Default::default()
        }))
        .unwrap()
        .to_file_with_info(directory.clone())
        .unwrap();
    let whiteTilePath = directory.join("4").join("8").join("0.jpg");
    assert!(!whiteTilePath.exists());
    assert_eq!(2809, info.width);
    assert_eq!(2074, info.height);
    assert_eq!(3, info.channels);
    assertGoogleTiles(&directory, 256, 5);

    //Google layout with center image in tile
    let directory = fixtures::output("output.google_center.dzi");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Google),
            centre: Some(true),
            ..Default::default()
        }))
        .unwrap()
        .to_file_with_info(directory.clone())
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    assert_similar!(
        fixtures::expected("tile_centered.jpg"),
        std::fs::read(directory.clone().join("0").join("0").join("0.jpg")).unwrap(),
        None
    );

    //IIIFv2 layout
    let directory = fixtures::output("output.iiif.info");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Iiif),
            id: Some(String::from("https://sharp.test.com/iiif")),
            ..Default::default()
        }))
        .unwrap()
        .to_file_with_info(directory.clone())
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    let stat =
        fs::stat(directory.join("0,0,256,256").join("256,").join("0").join("default.jpg")).unwrap();
    assert!(stat.is_file);
    assert!(stat.size > 0);

    //IIIFv3 layout
    let directory = fixtures::output("output.iiif.info");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Iiif3),
            id: Some(String::from("https://sharp.test.com/iiif3")),
            ..Default::default()
        }))
        .unwrap()
        .to_file_with_info(directory.clone())
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    let stat =
        fs::stat(directory.join("0,0,256,256").join("256,256").join("0").join("default.jpg"))
            .unwrap();
    assert!(stat.is_file);
    assert!(stat.size > 0);

    //Write to ZIP container using file extension
    let container = fixtures::output("output.dz.container.zip");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .to_file_with_info(container.clone())
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    let stat = fs::stat(container).unwrap();
    assert!(stat.is_file);
    assert!(stat.size > 0);

    //Write to ZIP container using container tile option
    let container = fixtures::output("output.dz.containeropt.zip");
    let info = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            container: Some(ForeignDzContainer::Zip),
            ..Default::default()
        }))
        .unwrap()
        .to_file_with_info(container.clone())
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    let stat = fs::stat(container).unwrap();
    assert!(stat.is_file);
    assert!(stat.size > 0);

    //Write ZIP container to Buffer
    let container = fixtures::output("output.dz.tiles.zip");
    let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            basename: Some(String::from("output.dz.tiles")),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);
    assert_eq!(3, info.channels);
    std::fs::write(container.clone(), data).unwrap();
    let stat = fs::stat(container).unwrap();
    assert!(stat.is_file);
    assert!(stat.size > 0);

    fixtures::clean_up();
    rs_vips::Vips::shutdown();
}

// Verifies all tiles in a given dz output directory are <= size
fn assertDeepZoomTiles(directory: &Path, expected_size: i32, expected_levels: i32) {
    let levels: Vec<String> = fs::readdir(directory, false, false)
        .unwrap()
        .into_iter()
        .filter(|d| d.attributes.is_directory)
        .map(|d| d.full_path)
        .collect();
    assert_eq!(expected_levels, levels.len() as _);

    levels.into_iter().for_each(|level| {
        fs::readdir(level, false, false).unwrap().into_iter().for_each(|d| {
            let metadata = Sharp::new_from_file(d.full_path).unwrap().metadata().unwrap();
            assert_eq!("jpeg", metadata.format);
            assert_eq!("srgb", metadata.space);
            assert_eq!(3, metadata.channels);
            assert!(!metadata.has_profile);
            assert!(!metadata.has_alpha);
            assert!(metadata.width <= expected_size);
            assert!(metadata.height <= expected_size);
        });
    });
}

fn assertZoomifyTiles(directory: &Path, _expected_size: i32, expected_levels: i32) {
    let stat = fs::stat(directory.join("ImageProperties.xml")).unwrap();
    assert!(stat.is_file);
    assert!(stat.size > 0);

    let mut maxTileLevel = -1;
    fs::readdir(directory.join("TileGroup0"), false, false).unwrap().into_iter().for_each(|d| {
        let level: i32 = d.name.split('-').collect::<Vec<_>>().first().unwrap().parse().unwrap();
        maxTileLevel = maxTileLevel.max(level);
    });

    assert_eq!(maxTileLevel + 1, expected_levels);
}

fn assertGoogleTiles(directory: &Path, _expected_size: i32, expected_levels: i32) {
    let levels: Vec<String> = fs::readdir(directory, false, false)
        .unwrap()
        .into_iter()
        .filter(|d| d.attributes.is_directory)
        .map(|d| d.full_path)
        .collect();
    assert_eq!(expected_levels, levels.len() as _);

    let stat = fs::stat(directory.join("blank.png")).unwrap();
    assert!(stat.is_file);
    assert!(stat.size > 0);

    // Basic check to confirm lowest and highest level tiles exist
    let stat = fs::stat(directory.join("0").join("0").join("0.jpg")).unwrap();
    assert!(stat.is_file);
    assert!(stat.size > 0);

    let stat = fs::stat(directory.join((expected_levels - 1).to_string()).join("0").join("0.jpg"))
        .unwrap();
    assert!(stat.is_file);
    assert!(stat.size > 0);
}

// Verifies tiles at specified level in a given output directory are > size+overlap
fn assertTileOverlap(directory: &Path, tileSize: i32) {
    let mut levels: Vec<i32> = fs::readdir(directory, false, false)
        .unwrap()
        .into_iter()
        .filter(|d| d.attributes.is_directory)
        .map(|d| d.name.parse::<i32>().unwrap())
        .collect();
    levels.sort();
    // Select the highest tile level
    let highestLevel = levels.last().unwrap();
    // Get sorted tiles from greatest level
    let mut tiles: Vec<String> =
        fs::readdir(directory.join(highestLevel.to_string()), false, false)
            .unwrap()
            .into_iter()
            .map(|d| d.full_path)
            .collect();
    tiles.sort();
    // Select a tile from the approximate center of the image
    let squareTile = tiles.get((tiles.len() as f64 / 2.0).floor() as usize).unwrap();
    let metadata = Sharp::new_from_file(squareTile).unwrap().metadata().unwrap();
    assert!(metadata.width > tileSize);
    assert!(metadata.height > tileSize);
}
