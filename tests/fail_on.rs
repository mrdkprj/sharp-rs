mod fixtures;
use sharp::{input::SharpOptions, Sharp};

#[test]
pub fn fail_on() {
    match Sharp::new_from_file_with_opts(
        fixtures::inputJpgTruncated(),
        SharpOptions {
            fail_on: Some(sharp::FailOn::Error),
            ..Default::default()
        },
    )
    .unwrap()
    .to_buffer()
    {
        Ok(_) => {}
        Err(e) => println!("error:{:?}", e),
    };
}
