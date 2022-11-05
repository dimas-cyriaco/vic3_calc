use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RESOURCES: HashMap<&'static str, Resource> = {
        let mut m: HashMap<&'static str, Resource> = HashMap::new();

        m.insert(
            "wood",
            Resource {
                name: String::from("Wood"),
            },
        );
        m
    };
}

lazy_static! {
    static ref BUILDINGS: HashMap<&'static str, Building<'static>> = {
        let mut buildings: HashMap<&'static str, Building> = HashMap::new();

        let wood = RESOURCES.get("wood").unwrap();

        buildings.insert(
            "wood_cutter",
            Building {
                name: String::from("Wood Cutter"),
                inputs: vec![],
                outputs: vec![Output {
                    resource: wood,
                    quantity: 5,
                }],
            },
        );

        buildings
    };
}

fn main() {
    let deficit = 10;

    let suggestions: Vec<Suggestion> = get_suggestions("wood", deficit);

    dbg!(suggestions);
}

fn get_suggestions<'a>(resource_name: &str, deficit: u32) -> Vec<Suggestion<'a>> {
    let wood = RESOURCES.get(resource_name).unwrap();

    BUILDINGS
        .iter()
        .filter(|(_, building)| {
            building
                .outputs
                .iter()
                .any(|output| output.resource == wood)
        })
        .map(|(_, building)| {
            let output_quantity = building
                .outputs
                .iter()
                .find(|output| output.resource == wood)
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
    solution: &'a Building<'a>,
    quantity: u32,
}

#[derive(Debug, PartialEq)]
struct Resource {
    name: String,
}

#[derive(Debug, PartialEq)]
struct Input {
    resource: Resource,
    quantity: u32,
}

#[derive(Debug, PartialEq)]
struct Output<'a> {
    resource: &'a Resource,
    quantity: u32,
}

#[derive(Debug, PartialEq)]
struct Building<'a> {
    name: String,
    inputs: Vec<Input>,
    outputs: Vec<Output<'a>>,
}
