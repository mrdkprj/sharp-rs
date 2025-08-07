use sharp::{operation::BlurOptions, Sharp};
mod fixtures;

#[test]
fn timeout() {
    Sharp::set_concurrency(1);
    match Sharp::new_from_file(fixtures::inputJpg())
        .unwrap()
        .blur(Some(BlurOptions {
            sigma: 200.0,
            ..Default::default()
        }))
        .unwrap()
        .timeout(1)
        .to_buffer()
    {
        Ok(_) => panic!("not timeout"),
        Err(_) => println!("timeout done"),
    }
}
