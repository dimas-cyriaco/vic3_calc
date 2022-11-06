use serde::Deserialize;
use std::{collections::HashMap, fs, process::exit};
use vic3::{get_suggestions, Suggestion};

fn main() {
    let resources = load_data("data/resources.toml");
    let buildings = load_data("data/buildings.toml");

    let deficit = 15;

    let suggestions: Vec<Suggestion> = get_suggestions("wood", deficit, &resources, &buildings);

    dbg!(suggestions);
}

fn load_data<T: for<'a> Deserialize<'a>>(filename: &str) -> HashMap<String, T> {
    let resources_file_content = if let Ok(d) = fs::read_to_string(filename) {
        d
    } else {
        eprintln!("Could not read file `{}`", filename);
        exit(1);
    };

    if let Ok(d) = toml::from_str(&resources_file_content) {
        d
    } else {
        eprintln!("Unable to load data from `{}`", filename);
        exit(1);
    }
}
