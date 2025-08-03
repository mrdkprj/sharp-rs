mod fixtures;
use sharp::{OperationBoolean, Sharp};

#[test]
pub fn bandbool() {
    //Bandbool per-channel boolean operations
    [OperationBoolean::And, OperationBoolean::Or, OperationBoolean::Eor].iter().for_each(|b| {
        Sharp::new_from_file(fixtures::inputPngBooleanNoAlpha())
            .unwrap()
            .bandbool(*b)
            .to_colourspace(sharp::Interpretation::BW)
            .to_buffer()
            .unwrap();
    });

    //'sRGB image retains 3 channels
    Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .bandbool(OperationBoolean::And)
        .to_buffer()
        .unwrap();
}
