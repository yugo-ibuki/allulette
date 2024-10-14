use clap::Parser;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::Read;
use serde_json::Value;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the JSON file containing options
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// Number of items to select
    #[arg(short, long, default_value_t = 1)]
    count: usize,

    /// List all available options
    #[arg(long)]
    list_options: bool,
}

fn main() {
    let args = Args::parse();

    let options = if let Some(file_path) = args.file {
        read_options_from_file(&file_path)
    } else {
        vec!["Option 1".to_string(), "Option 2".to_string(), "Option 3".to_string()]
    };

    if args.list_options {
        println!("Available options:");
        for (index, option) in options.iter().enumerate() {
            println!("{}. {}", index + 1, option);
        }
        return;
    }

    let mut rng = rand::thread_rng();
    let selected: Vec<&String> = options.choose_multiple(&mut rng, args.count).collect();

    println!("Allulette selected option(s):");
    for option in selected {
        println!("- {}", option);
    }
}

fn read_options_from_file(file_path: &PathBuf) -> Vec<String> {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let json: Value = serde_json::from_str(&contents).expect("Failed to parse JSON");
    
    if let Value::Array(arr) = json {
        arr.into_iter()
           .filter_map(|v| v.as_str().map(String::from))
           .collect()
    } else {
        panic!("JSON file does not contain an array");
    }
}
