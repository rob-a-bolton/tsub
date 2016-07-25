#[macro_use]
extern crate clap;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use clap::App;

/// Represents a collection of char -> char mappings
type Substitution = HashMap<char, char>;

/// Takes the substitution definitions and puts them into a Substitution
/// The line must start with a single character which will be dropped,
/// followed by pairs of letters
fn read_substitution_pairs(rest: &str) -> Option<Substitution> {
    let mut pairs: HashMap<char, char> = HashMap::new();
    let mut chars = rest.chars();

    chars.next(); // Drop the separator character
    
    while let (Some(old), Some(new)) = (chars.next(), chars.next()) {
        pairs.insert(old, new);
    }

    if pairs.len() > 0 {
        Some(pairs)
    } else {
        None
    }
}

/// If a line is a valid susbstitution definition, parse and return
fn get_substitution(line: &str) -> Option<(String, Substitution)> {
    if let Some(index) = line.find(":") {
        let (name, rest) = line.split_at(index);
        if let Some(subs) = read_substitution_pairs(rest) {
            Some((name.to_string(), subs))
        } else {
            None
        }
    } else {
        None
    }
}

/// Opens a file and reads all of the substitutions into a
/// HashMap, indexing them by name.
fn get_substitutions(path: &str) -> Result<HashMap<String, Substitution>, &str> {
    let mut substitutions: HashMap<String, Substitution> = HashMap::new();
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err("Could not read file"),
    };

    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            if let Some((name, sub)) = get_substitution(&line) {
                substitutions.insert(name, sub);
            }
        }
    }
    Ok(substitutions)
}

fn substitute(substitution: &Substitution) -> Result<usize, std::io::Error>{
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
            Err(_) => panic!("ruh roh"),
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
        Err(_) => panic!("Cannot continue with no substitutions."),
    };

    if matches.is_present("list_substitutions") {
        for (substitution, _) in substitutions.iter() {
            println!("{}", substitution);
        }
    } else if let Some(sub_name) = matches.value_of("substitution") {
        if let Some(sub) = substitutions.get(sub_name) {
            if let Err(msg) = substitute(sub) {
                panic!(msg);
            }
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
