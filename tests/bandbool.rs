mod fixtures;
use sharp::{OperationBoolean, Sharp};

#[test]
pub fn bandbool() {
    //Bandbool per-channel boolean operations
    [OperationBoolean::And, OperationBoolean::Or, OperationBoolean::Eor].iter().for_each(|b| {
        let (data, info) = Sharp::new_from_file(fixtures::inputPngBooleanNoAlpha())
            .unwrap()
            .bandbool(*b)
            .to_colourspace(sharp::Interpretation::BW)
            .to_buffer_with_info()
            .unwrap();
        assert_eq!(200, info.width);
        assert_eq!(200, info.height);
        assert_eq!(1, info.channels);
        assert_similar!(
            fixtures::expected(&format!("bandbool_{}_result.png", to_string(*b))),
            data,
            None
        );
    });

    //'sRGB image retains 3 channels
    let (_, info) = Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .bandbool(OperationBoolean::And)
        .to_buffer_with_info()
        .unwrap();
    assert_eq!(3, info.channels);
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
