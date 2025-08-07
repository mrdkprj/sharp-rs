use sharp::{input::SharpOptions, Sharp};
mod fixtures;

#[test]
fn rotate() {
    ["auto", "constructor"].iter().for_each(|rotate_method| {
        let options = if *rotate_method == "constructor" {
            Some(SharpOptions {
                auto_orient: Some(true),
                ..Default::default()
            })
        } else {
            None
        };

        //Auto-rotate
        if let Some(opts) = options.clone() {
            Sharp::new_from_file_with_opts(fixtures::inputJpg(), opts)
                .unwrap()
                .to_buffer()
                .unwrap();
        } else {
            Sharp::new_from_file(fixtures::inputJpg())
                .unwrap()
                .auto_orient()
                .unwrap()
                .to_buffer()
                .unwrap();
        };

        //Auto-rotate then resize
        if let Some(opts) = options.clone() {
            Sharp::new_from_file_with_opts(fixtures::inputJpg(), opts)
                .unwrap()
                .resize(320, 320)
                .unwrap()
                .to_buffer()
                .unwrap();
        } else {
            Sharp::new_from_file(fixtures::inputJpg())
                .unwrap()
                .auto_orient()
                .unwrap()
                .resize(320, 320)
                .unwrap()
                .to_buffer()
                .unwrap();
        };

        //Resize then auto-rotate
        if options.is_none() {
            Sharp::new_from_file(fixtures::inputJpg())
                .unwrap()
                .auto_orient()
                .unwrap()
                .resize(320, 320)
                .unwrap()
                .to_buffer()
                .unwrap();
        }

        //auto-rotate, flip, flop
        if let Some(opts) = options.clone() {
            Sharp::new_from_file_with_opts(fixtures::inputJpg(), opts)
                .unwrap()
                .flip(true)
                .unwrap()
                .flop(true)
                .unwrap()
                .to_buffer()
                .unwrap();
        } else {
            Sharp::new_from_file(fixtures::inputJpg())
                .unwrap()
                .auto_orient()
                .unwrap()
                .flip(true)
                .unwrap()
                .flop(true)
                .unwrap()
                .to_buffer()
                .unwrap();
        };
    });
}
