mod fixtures;
use sharp::{
    input::Input,
    operation::{BooleanOptions, Raw},
    OperationBoolean, Sharp,
};

#[test]
pub fn boolean() {
    let buffer = std::fs::read(fixtures::inputJpgBooleanTest()).unwrap();
    //Boolean operation between two images
    // operation, file
    [OperationBoolean::And, OperationBoolean::Or, OperationBoolean::Eor].iter().for_each(|b| {
        let data = Sharp::new_from_file(fixtures::inputJpg())
            .unwrap()
            .resize(320, 240)
            .unwrap()
            .boolean(
                Input::path(fixtures::inputJpgBooleanTest().to_string_lossy().to_string()),
                *b,
                None,
            )
            .unwrap()
            .to_buffer()
            .unwrap();
        assert_similar!(
            fixtures::expected(&format!("boolean_{}_result.jpg", to_string(*b))),
            data,
            None
        );

        // operation, buffer
        let data = Sharp::new_from_file(fixtures::inputJpg())
            .unwrap()
            .resize(320, 240)
            .unwrap()
            .boolean(Input::buffer(buffer.clone()), *b, None)
            .unwrap()
            .to_buffer()
            .unwrap();
        assert_similar!(
            fixtures::expected(&format!("boolean_{}_result.jpg", to_string(*b))),
            data,
            None
        );

        // operation, raw
        let (buf, info) = Sharp::new_from_file(fixtures::inputJpgBooleanTest())
            .unwrap()
            .raw(None)
            .unwrap()
            .to_buffer_with_info()
            .unwrap();

        let data = Sharp::new_from_file(fixtures::inputJpg())
            .unwrap()
            .resize(320, 240)
            .unwrap()
            .boolean(
                Input::buffer(buf),
                *b,
                Some(BooleanOptions {
                    raw: Raw {
                        width: info.width,
                        height: info.height,
                        channels: info.channels,
                    },
                }),
            )
            .unwrap()
            .to_buffer()
            .unwrap();
        assert_similar!(
            fixtures::expected(&format!("boolean_{}_result.jpg", to_string(*b))),
            data,
            None
        );
    });
}

fn to_string(o: OperationBoolean) -> String {
    match o {
        OperationBoolean::And => "and",
        OperationBoolean::Or => "or",
        OperationBoolean::Eor => "eor",
        _ => "",
    }
    .to_string()
}
