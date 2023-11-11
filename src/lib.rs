use crate::cli::Cli;
use std::{env, path::Path, process};

/// Handles the data of the program
pub mod cli;
mod items;
mod lists;

const LISTS_DIR: &str = "/.local/share/lists/";

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
    } else if cli.append {
        items::append_to_item(&path, cli.item_id, &cli.item);
        println!("Content appended");
    } else if cli.edit {
        items::edit_item(&path, cli.item_id, &cli.item);
        println!("Item edited");
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

fn get_lists_dir() -> String {
    // {{{
    if let Some(h) = env::home_dir() {
        let h = h.to_str().unwrap();
        let h = h.to_string();
        h + LISTS_DIR
    } else {
        eprintln!("Unable to obtain home directory");
        process::exit(1);
    }
}
// }}}

fn get_path(list: &str) -> String {
    // {{{
    let list_dir = get_lists_dir();

    format!("{list_dir}{list}")
}
// }}}
