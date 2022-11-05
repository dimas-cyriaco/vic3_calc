use std::{collections::HashMap, fs, process::exit};

use serde::Deserialize;

fn main() {
    let resources_filename = "data/resources.toml";

    let resources_file_content = match fs::read_to_string(resources_filename) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", resources_filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    let resources: HashMap<&str, Resource> = match toml::from_str(&resources_file_content) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from `{}`", resources_filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    let buildings_filename = "data/buildings.toml";

    let buildings_file_content = match fs::read_to_string(buildings_filename) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", buildings_filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    let buildings: HashMap<&str, Building> = match toml::from_str(&buildings_file_content) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(e) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from `{}`", buildings_filename);
            dbg!(e);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    let deficit = 10;

    let suggestions: Vec<Suggestion> = get_suggestions("wood", deficit, &resources, &buildings);

    dbg!(suggestions);
}

fn get_suggestions<'a>(
    resource_id: &str,
    deficit: u32,
    resources: &'a HashMap<&str, Resource>,
    buildings: &'a HashMap<&str, Building>,
) -> Vec<Suggestion<'a>> {
    let wood = resources.get(resource_id).unwrap();

    buildings
        .iter()
        .filter(|(_, building)| {
            building.outputs.iter().any(|output| {
                dbg!(output.resource_id.clone());
                dbg!(resource_id);
                output.resource_id == resource_id
            })
        })
        .map(|(_, building)| {
            let output_quantity = building
                .outputs
                .iter()
                .find(|output| output.resource_id == *resource_id)
                .map_or(0, |o| o.quantity);

            let quantity = deficit / output_quantity;

            Suggestion {
                deficit,
                target: wood,
                solution: building,
                quantity,
            }
        })
        .collect()
}

#[derive(Debug, PartialEq)]
struct Suggestion<'a> {
    deficit: u32,
    target: &'a Resource,
    solution: &'a Building,
    quantity: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Resource {
    name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Input {
    resource_id: String,
    quantity: u32,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Output {
    resource_id: String,
    quantity: u32,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Building {
    name: String,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
}
