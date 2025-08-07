use sharp::{
    output::{JpegOptions, PngOptions, TileOptions, WebpOptions},
    ForeignDzContainer, ForeignDzDepth, ForeignDzLayout, Sharp,
};
mod fixtures;

#[test]
fn tile() {
    //Deep Zoom layout with custom size+overlap
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(512),
            overlap: Some(16),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Deep Zoom layout with custom size+angle
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(512),
            angle: Some(90),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Deep Zoom layout with depth of one
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(512),
            depth: Some(sharp::ForeignDzDepth::One),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Deep Zoom layout with depth of onepixel
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(512),
            depth: Some(sharp::ForeignDzDepth::Onepixel),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //eep Zoom layout with depth of onetile
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(256),
            depth: Some(sharp::ForeignDzDepth::Onetile),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Deep Zoom layout with skipBlanks
    Sharp::new_from_file(fixtures::inputJpgOverlayLayer2())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(256),
            skip_blanks: Some(0),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Zoomify layout
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Zoomify),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Zoomify layout with depth one
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(256),
            layout: Some(ForeignDzLayout::Zoomify),
            depth: Some(ForeignDzDepth::One),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Zoomify layout with depth onepixel
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(256),
            layout: Some(ForeignDzLayout::Zoomify),
            depth: Some(ForeignDzDepth::Onepixel),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Zoomify layout with depth onetile
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(256),
            layout: Some(ForeignDzLayout::Zoomify),
            depth: Some(ForeignDzDepth::Onetile),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Zoomify layout with skip blanks
    Sharp::new_from_file(fixtures::inputJpgOverlayLayer2())
        .unwrap()
        .tile(Some(TileOptions {
            size: Some(256),
            layout: Some(ForeignDzLayout::Zoomify),
            skip_blanks: Some(0),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Google layout
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Google),

            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Google layout with jpeg format
    Sharp::new_from_file(fixtures::inputJpg())
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
        .to_buffer()
        .unwrap();

    //Google layout with png format
    Sharp::new_from_file(fixtures::inputJpg())
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
        .to_buffer()
        .unwrap();

    //Google layout with webp format
    Sharp::new_from_file(fixtures::inputJpg())
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
        .to_buffer()
        .unwrap();

    //Google layout with depth one
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Google),
            depth: Some(ForeignDzDepth::One),
            size: Some(256),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Google layout with depth onetile
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Google),
            depth: Some(ForeignDzDepth::Onetile),
            size: Some(256),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Google layout with default skip Blanks
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Google),
            size: Some(256),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Google layout with center image in tile
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Google),
            centre: Some(true),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //IIIF layout
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Iiif),
            id: Some(String::from("https://sharp.test.com/iiif")),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //IIIFv3 layout
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            layout: Some(ForeignDzLayout::Iiif3),
            id: Some(String::from("https://sharp.test.com/iiif3")),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Write to ZIP container using container tile option
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            container: Some(ForeignDzContainer::Zip),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();

    //Write ZIP container to Buffer
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .tile(Some(TileOptions {
            basename: Some(String::from("output.dz.tiles")),
            ..Default::default()
        }))
        .unwrap()
        .to_buffer()
        .unwrap();
}
