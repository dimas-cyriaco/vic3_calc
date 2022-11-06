mod vic;

use std::collections::HashMap;

pub use vic::{Building, Resource};

#[derive(Debug, Eq, PartialEq)]
pub struct Suggestion<'a> {
    pub deficit: u32,
    pub target: &'a Resource,
    pub solution: &'a Building,
    pub quantity: usize,
}

/// # Panics
pub fn get_suggestions<'a, S: ::std::hash::BuildHasher>(
    resource_id: &str,
    deficit: u32,
    resources: &'a HashMap<String, Resource, S>,
    buildings: &'a HashMap<String, Building, S>,
) -> Vec<Suggestion<'a>> {
    let wood = resources.get(resource_id).unwrap();

    buildings
        .iter()
        .filter(|(_, building)| {
            building
                .outputs
                .iter()
                .any(|output| output.resource_id == resource_id)
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
mod tests {
    use crate::vic::{BuildingFactory, ResourceFactory};

    use super::*;

    #[test]
    fn test_suggestion() {
        let resource_factory = ResourceFactory::new();
        let building_factory = BuildingFactory::new();

        let resource_id = String::from("wood");
        let wood = resource_factory.build(|_| {});
        let logging_camp = building_factory.build(|_| {});

        let resources = HashMap::from([(resource_id.clone(), wood.clone())]);
        let buildings = HashMap::from([(String::from("logging_camp"), logging_camp.clone())]);

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
