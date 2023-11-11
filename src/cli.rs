use std::{
    env::{self, Args},
    fs::File,
    io::{BufRead, BufReader},
    process,
};

pub struct Cli {
    // {{{
    /// Name of the list to operate on
    pub list_name: String,

    /// The item that will be added to the list
    pub item: String,

    /// Id (line number) of the item
    pub item_id: u8,

    /// Used when moving items
    pub new_id: u8,

    /// Create the list
    pub create: bool,

    /// Show all the lists the user created
    pub show_lists: bool,

    /// Don't ask for confirmation when removing a list
    pub confirmed: bool,

    /// Remove the list
    pub remove: bool,

    /// Add an item to the list
    pub add: bool,

    /// Append to an item of the list
    pub append: bool,

    /// Rewrite an item of the list
    pub edit: bool,

    /// Move an item to another place
    pub move_item: bool,

    /// Delete an item from the list
    pub delete: bool,
}
// }}}

impl Cli {
    // {{{
    pub fn new() -> Self {
        // {{{
        Self {
            list_name: String::new(),
            item: String::new(),
            item_id: 0,
            new_id: 0,

            // lists
            create: false,
            show_lists: true,
            confirmed: false,
            remove: false,

            // items
            add: false,
            append: false,
            edit: false,
            move_item: true,
            delete: false,
        }
    }
    // }}}

    pub fn parse_args(mut self) -> Self {
        // {{{
        let mut args = env::args();
        // First argument is the name of the program. Unneeded
        args.next();

        parse_list_operations(&mut self, &mut args);
        if self.show_lists || self.remove {
            return self;
        }

        parse_item_operations(&mut self, &mut args);
        self
    }
    // }}}

    pub fn print_help() {
        // {{{
    }
    // }}}
}
// }}}

fn parse_list_operations(cli: &mut Cli, args: &mut Args) {
    // {{{
    let arg = get_next_arg(args);

    if arg.is_empty() || arg == "show-lists" {
        return;
    }
    cli.show_lists = false;

    if arg == "help" || arg == "h" {
        Cli::print_help();
        process::exit(0);
    } else if arg == "create" || arg == "c" {
        cli.create = true;
    } else if arg == "remove" || arg == "r" {
        cli.remove = true;
    } else {
        cli.list_name = arg;
    }

    if cli.remove || cli.create {
        let arg = get_next_arg(args);

        if arg.is_empty() {
            eprintln!("Please provide the name of the list you wish to operate on");
            process::exit(1);
        }
        cli.list_name = arg;
    }

    if cli.remove {
        let arg = get_next_arg(args);
        if arg == "y" {
            cli.confirmed = true
        }
    }

    if !cli.create {
        let path = crate::get_path(&cli.list_name);
        check_list(&path);
    }
}
// }}}

fn parse_item_operations(cli: &mut Cli, args: &mut Args) {
    // {{{
    let arg = get_next_arg(args);

    if arg.is_empty() {
        return;
    }

    if arg == "add" || arg == "a" {
        cli.add = true;
    } else if arg == "append" || arg == "ap" {
        cli.append = true;
    } else if arg == "edit" || arg == "e" {
        cli.edit = true
    } else if arg == "move" || arg == "m" {
        cli.move_item = true;
    } else if arg == "delete" || arg == "d" {
        cli.delete = true;
    }

    if cli.append || cli.edit || cli.move_item || cli.delete {
        cli.item_id = get_item_id(args);

        check_id(&cli.list_name, cli.item_id);
        // -1 because lines are counted from 0
        cli.item_id -= 1;
    }

    if cli.add || cli.append || cli.edit {
        cli.item = get_item_content(args);
    }

    if cli.move_item {
        cli.new_id = get_item_id(args);

        check_id(&cli.list_name, cli.new_id);
        // -1 because lines are counted from 0
        cli.new_id -= 1;

        if cli.new_id == cli.item_id {
            eprintln!("Please provide different item ids");
            process::exit(1);
        }
    }
}
// }}}

fn get_next_arg(args: &mut Args) -> String {
    // {{{
    match args.next() {
        Some(a) => a,
        _ => String::new(),
    }
}
// }}}

fn get_item_id(args: &mut Args) -> u8 {
    // {{{
    let item_id = match get_next_arg(args).parse() {
        Ok(id) => id,
        _ => 0,
    };

    if item_id == 0 {
        eprintln!("Please provide a valid item id");
        process::exit(1);
    }

    item_id - 1
}
// }}}

fn get_item_content(args: &mut Args) -> String {
    // {{{
    let item = get_next_arg(args);

    if item.is_empty() {
        eprintln!("Please provide the item's content");
        process::exit(1);
    }
    item
}
// }}}

/// Makes sure the id is not above the amount of lines
fn check_id(list: &str, id: u8) {
    // {{{
    let path = crate::get_path(&list);
    let file = File::open(&path).expect("Unable to open list for reading");
    let line_count = BufReader::new(&file).lines().count();

    if usize::from(id) > line_count {
        eprintln!("The id is above the last id");
        process::exit(1);
    }
}
// }}}

/// Check if the list exists and warn the user
fn check_list(path: &str) {
    // {{{
    if !crate::list_exists(&path) {
        eprintln!("This list doesn't exist!");
        process::exit(1);
    }
}
// }}}
