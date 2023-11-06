use clap::Parser;
use items::Item;
use std::{path::Path, process};

mod items;
mod lists;

#[derive(Parser, Debug, PartialEq)]
#[command(version, long_about = None)]
#[command(about = "Manage lists in the CLI")]
#[command(next_line_help = true)]
pub struct Args {
    // {{{
    /// Name of the list to operate on
    ///
    /// Pass without flags to print the list
    #[arg(default_value = "n/a", name = "list_name")]
    pub list_name: String,

    /// Create the list <list_name>
    #[arg(short, long)]
    pub create: bool,

    /// Remove the list <list_name>
    #[arg(short, long)]
    pub remove: bool,

    /// Add an item to a list
    #[arg(short, long, default_value = "n/a", name = "item_content")]
    pub add: String,

    /// Delete an item from a list
    #[arg(short, long, default_value_t = 0, name = "item_id")]
    pub delete: u8,

    /// Don't ask for confirmation when deleting or removing
    #[arg(long)]
    pub no_confirmation: bool,

    /// Show all the available lists to operate on
    ///
    /// Default when no arguments are provided
    #[arg(long, default_value_t = true)]
    pub show_lists: bool,
}
// }}}

// const LISTS_DIR: &str = "~/.local/share/lists/";
const LISTS_DIR: &str = "./lists/"; // For debugging

pub fn run(args: Args) {
    // {{{
    let item = Item {
        contents: args.add.clone(),
        id: args.delete,
    };

    let path = get_path(&args.list_name);

    if args.create {
        lists::create_list(&path);
        println!("List created");
    } else if args.remove {
        lists::remove_list(&path, &args.list_name, args.no_confirmation);
        println!("List deleted");
        process::exit(0);
    }

    if item.contents != "n/a" {
        items::add_item(&path, &item.contents);
        println!("Item added");
    }
    if item.id != 0 {
        items::delete_item(&path, item.id);
        println!("Item removed");
    }

    if args.show_lists {
        lists::show_lists();
    } else {
        lists::print_list(&path, &args.list_name);
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
