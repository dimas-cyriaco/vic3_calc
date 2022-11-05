use std::{collections::HashMap, fs, process::exit};
use vic3::{get_suggestions, Building, Resource, Suggestion};

fn main() {
    let resources_filename = "data/resources.toml";

    let resources_file_content = match fs::read_to_string(resources_filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", resources_filename);
            exit(1);
        }
    };

    let resources: HashMap<&str, Resource> = match toml::from_str(&resources_file_content) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", resources_filename);
            exit(1);
        }
    };

    let buildings_filename = "data/buildings.toml";

    let buildings_file_content = match fs::read_to_string(buildings_filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", buildings_filename);
            exit(1);
        }
    };

    let buildings: HashMap<&str, Building> = match toml::from_str(&buildings_file_content) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Unable to load data from `{}`", buildings_filename);
            dbg!(e);
            exit(1);
        }
    };

    let deficit = 15;

    let suggestions: Vec<Suggestion> = get_suggestions("wood", deficit, &resources, &buildings);

    dbg!(suggestions);
}
