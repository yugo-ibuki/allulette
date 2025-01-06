use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use rand::distributions::{WeightedIndex, Distribution};

#[derive(Serialize, Deserialize, Debug)]
pub struct Option {
    name: String,
    weight: f64,
    enabled: bool,
}

impl Option {
    pub fn new(name: &str, weight: f64, enabled: bool) -> Self {
        Self {
            name: name.to_string(),
            weight,
            enabled,
        }
    }
}

pub fn read_options_from_file(file_path: &PathBuf) -> Vec<Option> {
    let mut file = File::open(file_path)
        .expect(&format!("Failed to open file: {:?}", file_path));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(&format!("Failed to read file: {:?}", file_path));

    if contents.trim().is_empty() {
        panic!("File is empty: {:?}", file_path);
    }

    match serde_json::from_str(&contents) {
        Ok(options) => options,
        Err(e) => panic!("Failed to parse JSON from {:?}: {}", file_path, e),
    }
}

pub fn list_options(options: &[Option]) {
    println!("Available options:");
    for (index, option) in options.iter().enumerate() {
        println!("{}. {} (weight: {}, {})", 
            index + 1, 
            option.name, 
            option.weight, 
            if option.enabled { "enabled" } else { "disabled" }
        );
    }
}

pub fn select_random_options(options: &[Option], count: usize) -> Vec<String> {
    let enabled_options: Vec<&Option> = options.iter()
        .filter(|o| o.enabled)
        .collect();

    let weights: Vec<f64> = enabled_options.iter()
        .map(|o| o.weight)
        .collect();

    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = rand::thread_rng();

    let mut selected = Vec::new();
    for _ in 0..count {
        if let Some(option) = enabled_options.get(dist.sample(&mut rng)) {
            selected.push(option.name.clone());
        }
    }

    selected
} 