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

    /// Create the list
    pub create: bool,

    /// Remove the list
    pub remove: bool,

    /// Add an item to the list
    pub add: bool,

    /// Append to an item of the list
    pub append: bool,

    /// Rewrite an item of the list
    pub edit: bool,

    /// Delete an item from the list
    pub delete: bool,

    /// Show all the available lists to operate on
    pub show_lists: bool,

    /// Don't ask for confirmation when removing a list
    pub confirmed: bool,
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
            create: false,
            remove: false,
            add: false,
            append: false,
            edit: false,
            delete: false,
            show_lists: true,
            confirmed: false,
        }
    }
    // }}}

    pub fn parse_args(mut self) -> Self {
        // {{{
        let mut args = env::args();

        // First argument is the name of the program. Unneeded
        args.next();

        parse_list_operations(&mut self, &mut args);

        if self.show_lists {
            return self;
        }

        parse_item_operations(&mut self, &mut args);
        self
    }
    // }}}

    pub fn print_help(mssg: &str) {
        // {{{
        eprintln!("{mssg}\n");
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

    if arg == "create" {
        cli.create = true;
    } else if arg == "remove" {
        cli.remove = true;
    } else {
        cli.list_name = arg;
    }

    if cli.remove || cli.create {
        let arg = get_next_arg(args);

        if arg.is_empty() {
            Cli::print_help("Please provide the name of the list you wish to operate on");
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
        crate::check_list(&path);
    }
}
// }}}

fn parse_item_operations(cli: &mut Cli, args: &mut Args) {
    // {{{
    let arg = get_next_arg(args);

    if arg.is_empty() {
        return;
    }

    if arg == "append" || arg == "edit" || arg == "delete" || arg == "move" {
        cli.item_id = get_item_id(args);

        let path = crate::get_path(&cli.list_name);
        check_id(&path, cli.item_id);

        // -1 because lines are counted from 0
        cli.item_id = cli.item_id - 1;
    }

    if arg == "add" {
        cli.item = get_item_content(args, "Please provide the item's content");
        cli.add = true;
    } else if arg == "append" {
        cli.item = get_item_content(args, "Please provide the content to append to the item");
        cli.append = true;
    } else if arg == "edit" {
        cli.item = get_item_content(args, "Please provide the new content of the item");
        cli.edit = true;
    } else if arg == "delete" {
        cli.delete = true;
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
        Cli::print_help("Please provide a valid item id");
    }

    item_id - 1
}
// }}}

fn get_item_content(args: &mut Args, err_msg: &str) -> String {
    // {{{
    let item = get_next_arg(args);

    if item.is_empty() {
        Cli::print_help(err_msg);
    }
    item
}
// }}}

/// Makes sure the id is not above the amount of lines
fn check_id(path: &str, id: u8) {
    // {{{
    let file = File::open(path).expect("Unable to open list for reading");
    let line_count = BufReader::new(&file).lines().count();

    if usize::from(id) > line_count {
        eprintln!("The id is above the last id");
        process::exit(1);
    }
}
// }}}
