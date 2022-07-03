mod domain;
mod settings;

use settings::Settings;

fn main() {
    println!("Hello, world!");
    let settings = Settings::new();

    // Print out our settings
    println!("{:?}", settings);
}
