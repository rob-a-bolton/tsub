#[macro_use]
extern crate clap;
extern crate rustc_serialize;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, BufReader, BufRead, Write};
use clap::App;
use rustc_serialize::json;

/// Represents a collection of char -> char mappings
type Substitution = HashMap<char, char>;

/// Opens a file and reads all of the substitutions into a
/// HashMap, indexing them by name.
fn get_substitutions(path: &str) -> Result<HashMap<String, Substitution>, &str> {
    let substitutions: HashMap<String, Substitution>;
    let mut f = match File::open(path) {
        Ok(file) => file,
        Err(err) => return Err("Could not open file."),
    };
        
    let mut json_str = String::new();
    
    if let Err(err) = f.read_to_string(&mut json_str) {
        return Err("Error reading file.");
    }

    match json::decode(&json_str) {
        Ok(json) => Ok(json),
        Err(err) => Err("Cold not decode JSON."),
    }
}

fn substitute(substitution: &Substitution) -> Result<usize, std::io::Error>{
    let mut output = std::io::stdout();
    let input = std::io::stdin();
    for line in input.lock().lines() {
        match line {
            Ok(line) => {
                let str_vec: Vec<char> = line.chars().map(
                    |c| match substitution.get(&c) {
                            Some(new_char) => new_char.clone(),
                            None => c,
                        }
                ).collect();
                let s: String = str_vec.into_iter().collect();
                println!("{}", s);
            },
            Err(err) => panic!("ruh roh"),
        }
    }

    Ok(0)
}


/// Runs the program, processing the command line arguments.
///
/// See [cli.yml](cli.yml) for the command line arguments taken
fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let config_path = matches.value_of("config").unwrap();
    let substitutions = match get_substitutions(config_path) {
        Ok(subs) => subs,
        Err(err) => panic!("Cannot continue with no substitutions."),
    };

    if matches.is_present("list_substitutions") {
        for (substitution, _) in substitutions.iter() {
            println!("{}", substitution);
        }
    } else if let Some(sub_name) = matches.value_of("substitution") {
        if let Some(sub) = substitutions.get(sub_name) {
            substitute(sub);
        } else {
            panic!("Invalid substitution.");
        }
    } else {
        panic!("No substitution chosen.");
    }
}


/// Tests whether substitutions can be loaded from
/// a JSON file
#[test]
fn deserialize_substitutions() {
    let test_file = "test/deserialize.json";

    let substitutions = match get_substitutions(test_file) {
        Ok(subs) => subs,
        Err(err) => panic!("Could not read substitutions from file."),
    };

    assert!(substitutions.contains_key("Test"));
    let test_subtitution = match substitutions.get("Test") {
        Some(sub) => sub,
        None => panic!("Test substitution not found."),
    };

    match test_subtitution.get(&'A') {
        Some(val) => assert_eq!('a', *val),
        None => panic!("Missing entry for A"),
    }
    match test_subtitution.get(&'B') {
        Some(val) => assert_eq!('b', *val),
        None => panic!("Missing entry for B"),
    }
    match test_subtitution.get(&'C') {
        Some(val) => assert_eq!('c', *val),
        None => panic!("Missing entry for C"),
    }
}
