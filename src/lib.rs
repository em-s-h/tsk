use crate::cli::Cli;
use std::{env, process};

/// Handles the data of the program
pub mod cli;
mod items;
mod lists;

const LISTS_DIR: &str = "/.local/share/lists/";

pub fn run(cli: Cli) {
    // {{{
    let path = get_path(&cli.list_name);

    if cli.print_help {
        Cli::print_help();
        process::exit(0);
    } else if cli.show_lists {
        lists::show_lists();
        process::exit(0);
    }

    if cli.create {
        lists::create_list(&path);
        println!("List created");
    } else if cli.print {
        lists::print_list(&path, &cli.list_name);
        process::exit(0);
    } else if cli.rename {
        lists::rename_list(&path, &cli.old_list_name, &cli.list_name);
        println!("List renamed");
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
    } else if cli.move_item {
        items::move_item(&path, cli.item_id, cli.new_id);
        println!("Item moved");
    } else if cli.delete {
        items::delete_item(&path, cli.item_id);
        println!("Item deleted");
    }

    if !cli.create {
        lists::print_list(&path, &cli.list_name);
    }
}
// }}}

fn get_lists_dir() -> String {
    // {{{
    if let Some(h) = env::home_dir() {
        let h = h.to_str().unwrap();
        h.to_string() + LISTS_DIR
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
