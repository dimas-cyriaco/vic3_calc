mod vic;

use std::collections::HashMap;

pub use vic::{Building, Resource};

#[derive(Debug, Eq, PartialEq)]
pub struct Suggestion {
    pub deficit: u32,
    pub quantity: usize,
    pub solution: String,
    pub target: String,
}

pub type ResourceRepo = HashMap<String, Resource>;
pub type BuildingRepo = HashMap<String, Building>;

pub struct Plan {
    pub suggestions: Vec<Suggestion>,
    pub surpluses: HashMap<String, u32>,
    resources: ResourceRepo,
    buildings: BuildingRepo,
}

impl Plan {
    #[must_use]
    pub fn new(resources: ResourceRepo, buildings: BuildingRepo) -> Plan {
        Plan {
            suggestions: vec![],
            surpluses: HashMap::new(),
            resources,
            buildings,
        }
    }

    pub fn add_goal(&mut self, resource_id: &str, quantity: u32) {
        let mut suggestions =
            get_suggestions(resource_id, quantity, &self.resources, &self.buildings);

        self.suggestions.append(&mut suggestions);
    }
}

/// # Panics
#[must_use]
pub fn get_suggestions(
    resource_id: &str,
    deficit: u32,
    resources: &ResourceRepo,
    buildings: &BuildingRepo,
) -> Vec<Suggestion> {
    let resource = match resources.get(resource_id) {
        Some(d) => d,
        None => {
            return vec![];
        }
    };

    let suggestions = buildings
        .iter()
        .filter(|(_, building)| {
            building
                .outputs
                .iter()
                .any(|output| output.resource_id == resource_id)
        })
        .flat_map(|(_, building)| {
            let output_quantity = building
                .outputs
                .iter()
                .find(|output| output.resource_id == *resource_id)
                .map_or(0, |o| o.quantity);

            // https://www.reddit.com/r/rust/comments/bk7v15/my_next_favourite_way_to_divide_integers_rounding/
            // Follow this up: https://doc.rust-lang.org/std/primitive.i32.html#method.div_ceil
            let quantity = (0..deficit).step_by(output_quantity as usize).size_hint().0;

            let mut input_suggestions: Vec<Suggestion> = building
                .inputs
                .iter()
                .flat_map(|i| {
                    // testar se a quantidade necessária já existe nas sugestões.
                    // Suggestion pode conter `excedents`.
                    get_suggestions(
                        &i.resource_id,
                        i.quantity * u32::try_from(quantity).unwrap(),
                        resources,
                        buildings,
                    )
                })
                .collect();

            input_suggestions.push(Suggestion {
                deficit,
                target: resource.name.clone(),
                solution: building.name.clone(),
                quantity,
            });

            input_suggestions
        })
        .collect();

    suggestions
    // Ok(suggestions)
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};

    use crate::vic::{Input, Output};

    use super::*;

    macro_rules! hash {
        ($($k:expr => $v:expr),* $(,)?) => {{
            core::convert::From::from([$(($k, $v),)*])
        }};
    }

    #[test]
    fn test_plan_add_goal() {
        let wood: Resource = Faker.fake();
        let wood_resource_id = String::from("wood");

        let logging_camp: Building = Building {
            inputs: vec![],
            outputs: vec![Output {
                resource_id: wood_resource_id.clone(),
                quantity: 5,
            }],
            ..Faker.fake()
        };
        let logging_camp_id = String::from("logging_camp");

        let resources = hash! { wood_resource_id.clone() => wood.clone() };
        let buildings = hash! { logging_camp_id.clone() => logging_camp.clone() };

        let mut plan = Plan::new(resources, buildings);
        let deficit = 10;

        plan.add_goal(&wood_resource_id, deficit);

        let expected_suggestions = vec![Suggestion {
            deficit,
            target: wood.name.clone(),
            solution: logging_camp.name.clone(),
            quantity: 2,
        }];

        let suggestions = plan.suggestions;

        assert_eq!(expected_suggestions, suggestions);
    }

    #[test]
    fn test_suggestion() {
        let wood: Resource = Faker.fake();
        let wood_resource_id = String::from("wood");

        let logging_camp: Building = Building {
            inputs: vec![],
            outputs: vec![Output {
                resource_id: wood_resource_id.clone(),
                quantity: 1,
            }],
            ..Faker.fake()
        };
        let logging_camp_id = String::from("logging_camp");

        let resources = hash! { wood_resource_id.clone() => wood.clone() };
        let buildings = hash! { logging_camp_id.clone() => logging_camp.clone() };

        let deficit = 3;

        let expected_suggestion = Suggestion {
            deficit,
            target: wood.name.clone(),
            solution: logging_camp.name.clone(),
            quantity: 3,
        };

        let suggestions = get_suggestions(&wood_resource_id, deficit, &resources, &buildings);

        assert_eq!(expected_suggestion, suggestions[0]);
    }

    #[test]
    fn test_chained_suggestion() {
        let wood: Resource = Faker.fake();
        let wood_resource_id = String::from("wood");

        let tools: Resource = Faker.fake();
        let tools_resource_id = String::from("tools");

        let logging_camp: Building = Building {
            inputs: vec![],
            outputs: vec![Output {
                resource_id: wood_resource_id.clone(),
                quantity: 10,
            }],
            ..Faker.fake()
        };
        let logging_camp_id = String::from("logging_camp");

        let tooling_workshop: Building = Building {
            inputs: vec![Input {
                resource_id: wood_resource_id.clone(),
                quantity: 20,
            }],
            outputs: vec![Output {
                resource_id: tools_resource_id.clone(),
                quantity: 10,
            }],
            ..Faker.fake()
        };
        let tooling_workshop_id = String::from("tooling_workshop");

        let resources = hash! {
            wood_resource_id.clone() => wood.clone(),
            tools_resource_id.clone() => tools.clone(),
        };

        let buildings = hash! {
            logging_camp_id.clone() => logging_camp.clone(),
            tooling_workshop_id.clone() => tooling_workshop.clone(),
        };

        let deficit = 20;

        let expected_suggestions = vec![
            Suggestion {
                deficit: 40,
                target: wood.name.clone(),
                solution: logging_camp.name.clone(),
                quantity: 4,
            },
            Suggestion {
                deficit,
                target: tools.name.clone(),
                solution: tooling_workshop.name.clone(),
                quantity: 2,
            },
        ];

        let suggestions = get_suggestions(&tools_resource_id, deficit, &resources, &buildings);

        assert_eq!(expected_suggestions, suggestions);
    }

    // Testar referência circular.
}
