use sharp::{
    input::{Create, Inputs},
    resize::{Fit, Region, ResizeOptions},
    Colour, Kernel, Sharp,
};
mod fixtures;

#[test]
fn resize() {
    //Exact crop
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(320, 240)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(320, info.width);
    assert_eq!(240, info.height);

    //Fixed width
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(320, info.width);
    assert_eq!(261, info.height);

    //Fixed height
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            height: Some(320),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(392, info.width);
    assert_eq!(320, info.height);

    //Identity transform
    let (_, info) =
        Sharp::new_from_file(fixtures::inputJpg()).unwrap().to_buffer_with_info().unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);

    //Upscale
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(3000),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(3000, info.width);
    assert_eq!(2450, info.height);

    //Webp resize then extract large image
    let (_, info) = Sharp::new_from_file(fixtures::inputWebP())
        .unwrap()
        .resize(0x4000, 0x4000)
        .unwrap()
        .extract(Region {
            top: 0x2000,
            left: 0x2000,
            width: 256,
            height: 256,
        })
        .unwrap()
        .webp(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("webp", info.format);
    assert_eq!(256, info.width);
    assert_eq!(256, info.height);

    //WebP shrink-on-load rounds to zero, ensure recalculation is correct
    let (data, info1) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(1080, 607)
        .unwrap()
        .webp(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("webp", info1.format);
    assert_eq!(1080, info1.width);
    assert_eq!(607, info1.height);

    let (_, info) = Sharp::new_from_buffer(data)
        .unwrap()
        .resize(233, 131)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("webp", info.format);
    assert_eq!(233, info.width);
    assert_eq!(131, info.height);

    //JPEG shrink-on-load with 90 degree rotation, ensure recalculation is correct
    let (data, info1) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(1920, 1280)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(1920, info1.width);
    assert_eq!(1280, info1.height);

    let (_, info) = Sharp::new_from_buffer(data)
        .unwrap()
        .rotate(90, None)
        .unwrap()
        .resize(533, 800)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(533, info.width);
    assert_eq!(800, info.height);

    //TIFF embed known to cause rounding errors
    let (_, info) = Sharp::new_from_file(fixtures::inputTiff())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(240),
            height: Some(320),
            fit: Some(Fit::Contain),
            ..Default::default()
        })
        .unwrap()
        .jpeg(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(240, info.width);
    assert_eq!(320, info.height);

    //TIFF known to cause rounding errors
    let (_, info) = Sharp::new_from_file(fixtures::inputTiff())
        .unwrap()
        .resize(240, 320)
        .unwrap()
        .jpeg(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(240, info.width);
    assert_eq!(320, info.height);

    //fit=inside, portrait
    let (_, info) = Sharp::new_from_file(fixtures::inputTiff())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(320),
            fit: Some(Fit::Inside),
            ..Default::default()
        })
        .unwrap()
        .jpeg(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(243, info.width);
    assert_eq!(320, info.height);

    //fit=outside, portrait
    let (_, info) = Sharp::new_from_file(fixtures::inputTiff())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(320),
            fit: Some(Fit::Outside),
            ..Default::default()
        })
        .unwrap()
        .jpeg(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(320, info.width);
    assert_eq!(422, info.height);

    //fit=inside, landscape
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(320),
            fit: Some(Fit::Inside),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(320, info.width);
    assert_eq!(261, info.height);

    //fit=outside, landscape
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(320),
            fit: Some(Fit::Outside),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(392, info.width);
    assert_eq!(320, info.height);

    //fit=inside, provide only one dimension
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),

            fit: Some(Fit::Inside),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(320, info.width);
    assert_eq!(261, info.height);

    //fit=inside, provide only one dimension
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),

            fit: Some(Fit::Outside),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!("jpeg", info.format);
    assert_eq!(320, info.width);
    assert_eq!(261, info.height);

    //Do not enlarge when input width is already less than output width
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(2800),
            without_enlargement: Some(true),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);

    //Do not enlarge when input height is already less than output height
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            height: Some(2300),
            without_enlargement: Some(true),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);

    //Do crop when fit = cover and withoutEnlargement = true and width >= outputWidth, and height < outputHeight
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(3000),
            height: Some(1000),
            without_enlargement: Some(true),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(1000, info.height);

    //Do crop when fit = cover and withoutEnlargement = true and width < outputWidth, and height >= outputHeight
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(1500),
            height: Some(2226),
            without_enlargement: Some(true),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(1500, info.width);
    assert_eq!(2225, info.height);

    //Do enlarge when input width is less than output width
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(2800),
            without_enlargement: Some(false),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(2800, info.width);
    assert_eq!(2286, info.height);

    //Do enlarge when input width is less than output width
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(2800),
            without_reduction: Some(true),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(2800, info.width);
    assert_eq!(2286, info.height);

    //Do enlarge when input height is less than output height
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            height: Some(2300),
            without_reduction: Some(true),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(2817, info.width);
    assert_eq!(2300, info.height);

    //Do enlarge when input width is less than output width
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(2800),
            without_reduction: Some(true),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(2800, info.width);
    assert_eq!(2286, info.height);

    //Do not resize when both withoutEnlargement and withoutReduction are true
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(320),
            fit: Some(Fit::Fill),
            without_enlargement: Some(true),
            without_reduction: Some(true),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);

    //Do not reduce size when fit = outside and withoutReduction are true and height > outputHeight and width > outputWidth
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(320),
            fit: Some(Fit::Outside),
            without_reduction: Some(true),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);

    //Do resize when fit = outside and withoutReduction are true and input height > height and input width > width
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(320),
            fit: Some(Fit::Outside),
            without_reduction: Some(true),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);

    //Do resize when fit = outside and withoutReduction are true and input height > height and input width > width
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(3000),
            height: Some(3000),
            fit: Some(Fit::Outside),
            without_reduction: Some(true),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(3674, info.width);
    assert_eq!(3000, info.height);

    //fit=fill, downscale width and height
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(320),
            fit: Some(Fit::Fill),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(320, info.height);

    //fit=fill, downscale width
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            fit: Some(Fit::Fill),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(2225, info.height);

    //fit=fill, downscale height
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            height: Some(320),
            fit: Some(Fit::Fill),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(320, info.height);

    //fit=fill, upscale width and height
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(3000),
            height: Some(3000),
            fit: Some(Fit::Fill),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(3000, info.width);
    assert_eq!(3000, info.height);

    //fit=fill, upscale width
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(3000),

            fit: Some(Fit::Fill),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(3000, info.width);
    assert_eq!(2225, info.height);

    //fit=fill, upscale height
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            height: Some(3000),

            fit: Some(Fit::Fill),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(3000, info.height);

    //fit=fill, downscale width, upscale height
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(320),
            height: Some(3000),
            fit: Some(Fit::Fill),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(320, info.width);
    assert_eq!(3000, info.height);

    //fit=fill, upscale width, downscale height
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(3000),
            height: Some(320),
            fit: Some(Fit::Fill),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(3000, info.width);
    assert_eq!(320, info.height);

    //fit=fill, identity transform
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            fit: Some(Fit::Fill),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(2725, info.width);
    assert_eq!(2225, info.height);

    // /Dimensions that result in differing even shrinks on each axis
    let (data, info1) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(645, 399)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(645, info1.width);
    assert_eq!(399, info1.height);

    let (data, info) = Sharp::new_from_buffer(data)
        .unwrap()
        .resize(150, 100)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(150, info.width);
    assert_eq!(100, info.height);
    assert_similar!(fixtures::expected("resize-diff-shrink-even.jpg"), data, None);

    //Dimensions that result in differing odd shrinks on each axis
    let (data, info1) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .resize(600, 399)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(600, info1.width);
    assert_eq!(399, info1.height);

    let (data, info) = Sharp::new_from_buffer(data)
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(200),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(200, info.width);
    assert_eq!(133, info.height);
    assert_similar!(fixtures::expected("resize-diff-shrink-odd.jpg"), data, None);

    [true, false].iter().for_each(|val| {
        let (data, info) = Sharp::new_from_file(fixtures::inputJpgCenteredImage())
            .unwrap()
            .resize_with_opts(ResizeOptions {
                width: Some(9),
                height: Some(8),
                fast_shrink_on_load: Some(*val),
                ..Default::default()
            })
            .unwrap()
            .png(None)
            .unwrap()
            .to_buffer_with_info()
            .unwrap();
        assert_eq!(9, info.width);
        assert_eq!(8, info.height);
        assert_similar!(fixtures::expected("fast-shrink-on-load.png"), data, None);
    });

    [Kernel::Nearest, Kernel::Cubic, Kernel::Mitchell, Kernel::Lanczos2, Kernel::Lanczos3]
        .iter()
        .for_each(|kernel| {
            let (data, info) = Sharp::new_from_file(fixtures::inputJpg())
                .unwrap()
                .resize_with_opts(ResizeOptions {
                    width: Some(320),
                    kernel: Some(*kernel),
                    ..Default::default()
                })
                .unwrap()
                .to_buffer_with_info()
                .unwrap();
            assert_eq!(320, info.width);
            assert_similar!(fixtures::inputJpg(), data, None);
        });

    //nearest upsampling with integral factor
    let (_, info) = Sharp::new_from_file(fixtures::inputTiff8BitDepth())
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(210),
            height: Some(210),
            kernel: Some(Kernel::Nearest),
            ..Default::default()
        })
        .unwrap()
        .png(None)
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(210, info.width);
    assert_eq!(210, info.height);

    //Ensure shortest edge (height) is at least 1 pixel
    let (_, info) = Sharp::new(Inputs::new().create(Create {
        width: 10,
        height: 2,
        channels: 3,
        background: Colour::rgb(255, 0, 0),
        ..Default::default()
    }))
    .unwrap()
    .resize_with_opts(ResizeOptions {
        width: Some(2),
        ..Default::default()
    })
    .unwrap()
    .to_buffer_with_info()
    .unwrap();
    assert_eq!(2, info.width);
    assert_eq!(1, info.height);

    //Ensure shortest edge (width) is at least 1 pixel
    let (_, info) = Sharp::new(Inputs::new().create(Create {
        width: 2,
        height: 10,
        channels: 3,
        background: Colour::rgb(255, 0, 0),
        ..Default::default()
    }))
    .unwrap()
    .resize_with_opts(ResizeOptions {
        height: Some(2),
        ..Default::default()
    })
    .unwrap()
    .to_buffer_with_info()
    .unwrap();
    assert_eq!(1, info.width);
    assert_eq!(2, info.height);

    //Ensure embedded shortest edge (height) is at least 1 pixel
    let (_, info) = Sharp::new(Inputs::new().create(Create {
        width: 200,
        height: 1,
        channels: 3,
        background: Colour::rgb(255, 0, 0),
        ..Default::default()
    }))
    .unwrap()
    .resize_with_opts(ResizeOptions {
        width: Some(50),
        height: Some(50),
        fit: Some(Fit::Contain),
        ..Default::default()
    })
    .unwrap()
    .to_buffer_with_info()
    .unwrap();
    assert_eq!(50, info.width);
    assert_eq!(50, info.height);

    //Ensure embedded shortest edge (width) is at least 1 pixel
    let (_, info) = Sharp::new(Inputs::new().create(Create {
        width: 1,
        height: 200,
        channels: 3,
        background: Colour::rgb(255, 0, 0),
        ..Default::default()
    }))
    .unwrap()
    .resize_with_opts(ResizeOptions {
        width: Some(50),
        height: Some(50),
        fit: Some(Fit::Contain),
        ..Default::default()
    })
    .unwrap()
    .to_buffer_with_info()
    .unwrap();
    assert_eq!(50, info.width);
    assert_eq!(50, info.height);

    //Skip shrink-on-load where one dimension <4px
    let jpeg = Sharp::new(Inputs::new().create(Create {
        width: 100,
        height: 3,
        channels: 3,
        background: Colour::rgb(255, 0, 0),
        ..Default::default()
    }))
    .unwrap()
    .jpeg(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    let (_, info) = Sharp::new_from_buffer(jpeg)
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(8),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(info.width, 8);
    assert_eq!(info.height, 1);

    //Skip JPEG shrink-on-load for known libjpeg rounding errors
    let input = Sharp::new(Inputs::new().create(Create {
        width: 1000,
        height: 667,
        channels: 3,
        background: Colour::rgb(255, 0, 0),
        ..Default::default()
    }))
    .unwrap()
    .jpeg(None)
    .unwrap()
    .to_buffer()
    .unwrap();

    let (_, info) = Sharp::new_from_buffer(input)
        .unwrap()
        .resize_with_opts(ResizeOptions {
            width: Some(500),
            ..Default::default()
        })
        .unwrap()
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(info.width, 500);
    assert_eq!(info.height, 334);

    rs_vips::Vips::shutdown();
}
