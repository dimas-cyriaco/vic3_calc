use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, process::exit};

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

            // https://www.reddit.com/r/rust/comments/bk7v15/my_next_favourite_way_to_divide_integers_rounding/
            // Follow this up: https://doc.rust-lang.org/std/primitive.i32.html#method.div_ceil
            let quantity = (0..deficit).step_by(output_quantity as usize).size_hint().0;

            Suggestion {
                deficit,
                target: wood,
                solution: building,
                quantity,
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_suggestion() {
        let resource_factory = ResourceFactory::new();
        let building_factory = BuildingFactory::new();

        let resource_id = String::from("wood");
        let wood = resource_factory.build(|_| {});
        let logging_camp = building_factory.build(|_| {});

        let resources = HashMap::from([(&resource_id[0..], wood.clone())]);
        let buildings = HashMap::from([("logging_camp", logging_camp.clone())]);

        let deficit = 3;

        let expected_suggestion = Suggestion {
            deficit,
            target: &wood,
            solution: &logging_camp,
            quantity: 3,
        };

        let suggestions = get_suggestions(&resource_id, deficit, &resources, &buildings);

        assert!(expected_suggestion == suggestions[0]);
    }
}

#[derive(Debug, PartialEq)]
struct Suggestion<'a> {
    deficit: u32,
    target: &'a Resource,
    solution: &'a Building,
    quantity: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Resource {
    name: String,
}
beaver::define! {
    ResourceFactory (Resource) {
        name -> |_| String::from("Wood"),
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct Input {
    resource_id: String,
    quantity: u32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct Output {
    resource_id: String,
    quantity: u32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct Building {
    name: String,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
}
beaver::define! {
    BuildingFactory (Building) {
        name -> |_| String::from("Wood"),
        inputs -> |_| Vec::new(),
        outputs -> |_| {
            let mut vec = Vec::new();

            let output = Output {
                resource_id: String::from("wood"),
                quantity: 1,
            };

            vec.push(output);

            vec
        },
    }
}
