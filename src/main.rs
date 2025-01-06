mod cli;
mod option;
mod history;

use clap::Parser;
use cli::Args;
use option::Option;
use history::{save_to_history, show_history, clear_history};

fn main() {
    let args = Args::parse();

    if args.clear_history {
        clear_history();
        println!("History cleared!");
        return;
    }

    if args.history {
        show_history();
        return;
    }

    let options = if let Some(file_path) = args.file {
        option::read_options_from_file(&file_path)
    } else {
        vec![
            Option::new("Option 1", 1.0, true),
            Option::new("Option 2", 1.0, true),
            Option::new("Option 3", 1.0, true),
        ]
    };

    if args.list_options {
        option::list_options(&options);
        return;
    }

    let selected = option::select_random_options(&options, args.count);

    println!("Allulette selected option(s):");
    for option in &selected {
        println!("- {}", option);
    }

    save_to_history(selected);
}
