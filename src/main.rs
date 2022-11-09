use clap::Parser;
use serde::Deserialize;
use std::{collections::HashMap, fs, process::exit};
use vic3::Plan;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the resource to plan to
    #[arg(short, long)]
    resource: String,

    /// Quantity of resource you are lacking in you economy
    #[arg(short, long, default_value_t = 1)]
    deficit: u32,
}

fn main() {
    let resources = load_data("data/resources.yaml");
    let buildings = load_data("data/buildings.yaml");

    let args = Args::parse();

    let mut plan = Plan::new(resources, buildings);

    plan.add_goal(&args.resource, args.deficit);

    println!("To resolve your {} problem you can build:", args.resource);

    for suggestion in plan.suggestions {
        println!("\t* {}: {}", suggestion.solution, suggestion.quantity);
    }
}

fn load_data<T: for<'a> Deserialize<'a>>(filename: &str) -> HashMap<String, T> {
    let resources_file_content = match fs::read_to_string(filename) {
        Ok(d) => d,
        Err(e) => {
            dbg!(e);
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };

    match serde_yaml::from_str(&resources_file_content) {
        Ok(d) => d,
        Err(e) => {
            dbg!(e);
            eprintln!("Unable to load data from `{}`", filename);
            exit(1);
        }
    }
}
