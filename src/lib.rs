use crate::cli::Cli;
use std::{path::Path, process};

/// Handles the data of the program
pub mod cli;
mod items;
mod lists;

// const LISTS_DIR: &str = "~/.local/share/lists/";
const LISTS_DIR: &str = "./lists/"; // For debugging

pub fn run(cli: Cli) {
    // {{{
    let path = get_path(&cli.list_name);

    if cli.create {
        lists::create_list(&path);
        println!("List created");
    } else if cli.remove {
        lists::remove_list(&path, &cli.list_name, cli.confirmed);
        println!("List removed");
        process::exit(0);
    }

    if cli.add {
        items::add_item(&path, &cli.item);
        println!("Item added");
    } else if cli.delete {
        items::delete_item(&path, cli.item_id);
        println!("Item deleted");
    }

    if cli.show_lists {
        lists::show_lists();
    } else {
        lists::print_list(&path, &cli.list_name);
    }
}
// }}}

/// Check if the list exists and warn the user
fn check_list(path: &str) {
    // {{{
    if !list_exists(&path) {
        eprintln!("This list doesn't exist!");
        process::exit(1);
    }
}
// }}}

fn list_exists(path: &str) -> bool {
    // {{{
    let path = Path::new(path);
    if let Ok(exists) = path.try_exists() {
        exists
    } else {
        eprintln!("Unable to verify the existence of file");
        false
    }
}
// }}}

fn get_path(list: &str) -> String {
    format!("{}{}", LISTS_DIR, list)
}

#[cfg(test)]
mod test {
    // {{{
}
// }}}
