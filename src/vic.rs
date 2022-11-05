use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Resource {
    pub name: String,
}
beaver::define! {
    pub ResourceFactory (Resource) {
        name -> |_| String::from("Wood"),
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Input {
    pub resource_id: String,
    pub quantity: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Output {
    pub resource_id: String,
    pub quantity: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Building {
    pub name: String,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}
beaver::define! {
    pub BuildingFactory (Building) {
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
