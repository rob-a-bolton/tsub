#[macro_use]
extern crate clap;

use std::collections::HashMap;
use clap::App;

type Substitution = HashMap<char, char>;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let config_path = matches.value_of("config").unwrap();

}
