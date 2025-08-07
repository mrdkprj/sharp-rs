mod fixtures;
use sharp::{
    operation::{BooleanOptions, Raw},
    OperationBoolean, Sharp,
};

#[test]
pub fn boolean() {
    let buffer = std::fs::read(fixtures::inputJpgBooleanTest()).unwrap();
    //Boolean operation between two images
    [OperationBoolean::And, OperationBoolean::Or, OperationBoolean::Eor].iter().for_each(|b| {
        Sharp::new_from_file(fixtures::inputJpg())
            .unwrap()
            .resize(320, 240)
            .unwrap()
            .boolean(
                sharp::input::Input::Path(
                    fixtures::inputJpgBooleanTest().to_string_lossy().to_string(),
                ),
                *b,
                None,
            )
            .unwrap()
            .to_buffer()
            .unwrap();

        Sharp::new_from_file(fixtures::inputJpg())
            .unwrap()
            .resize(320, 240)
            .unwrap()
            .boolean(sharp::input::Input::Buffer(buffer.clone()), *b, None)
            .unwrap()
            .to_buffer()
            .unwrap();

        let data = Sharp::new_from_file(fixtures::inputJpgBooleanTest())
            .unwrap()
            .raw(None)
            .unwrap()
            .to_buffer()
            .unwrap();

        Sharp::new_from_file(fixtures::inputJpg())
            .unwrap()
            .resize(320, 240)
            .unwrap()
            .boolean(
                sharp::input::Input::Buffer(data),
                *b,
                Some(BooleanOptions {
                    raw: Raw {
                        width: 320,
                        height: 240,
                        channels: 3,
                    },
                }),
            )
            .unwrap()
            .to_buffer()
            .unwrap();
    });
}
