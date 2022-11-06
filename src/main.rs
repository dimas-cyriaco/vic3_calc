use clap::Parser;
use serde::Deserialize;
use std::{collections::HashMap, fs, process::exit};
use vic3::{get_suggestions, Suggestion};

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
    let resources = load_data("data/resources.toml");
    let buildings = load_data("data/buildings.toml");

    let args = Args::parse();

    let suggestions: Vec<Suggestion> =
        get_suggestions(&args.resource, args.deficit, &resources, &buildings);

    println!("To resolve your {} problem you can build:", args.resource);

    for suggestion in suggestions {
        println!("\t* {}: {}", suggestion.solution.name, suggestion.quantity);
    }
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
