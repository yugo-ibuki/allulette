use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use chrono::Local;
use dirs;

#[derive(Serialize, Deserialize, Debug)]
pub struct History {
    timestamp: String,
    selected: Vec<String>,
}

fn get_history_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("allulette");
    std::fs::create_dir_all(&path).unwrap();
    path.push("history.json");
    path
}

pub fn save_to_history(selected: Vec<String>) {
    let history_path = get_history_path();
    let mut histories: Vec<History> = if history_path.exists() {
        let contents = std::fs::read_to_string(&history_path).unwrap_or_else(|_| String::from("[]"));
        serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    };

    histories.push(History {
        timestamp: Local::now().to_rfc3339(),
        selected,
    });

    let json = serde_json::to_string_pretty(&histories).unwrap();
    std::fs::write(history_path, json).expect("Failed to save history");
}

pub fn show_history() {
    let history_path = get_history_path();
    if !history_path.exists() {
        println!("No history found.");
        return;
    }

    let contents = std::fs::read_to_string(history_path).expect("Failed to read history");
    let histories: Vec<History> = serde_json::from_str(&contents).expect("Failed to parse history");

    println!("Selection history:");
    for history in histories {
        println!("{}:", history.timestamp);
        for item in history.selected {
            println!("  - {}", item);
        }
    }
}

pub fn clear_history() {
    let history_path = get_history_path();
    if history_path.exists() {
        std::fs::remove_file(history_path).expect("Failed to clear history");
    }
} 