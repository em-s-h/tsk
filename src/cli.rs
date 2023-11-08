use std::env::{self, Args};

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

    /// Delete an item from the list
    pub delete: bool,

    /// Show all the available lists to operate on
    pub show_lists: bool,
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
            delete: false,
            show_lists: true,
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

    pub fn print_help() {
        // {{{
        unimplemented!()
    }
    // }}}
}
// }}}

fn get_next_arg(args: &mut Args) -> String {
    // {{{
    match args.next() {
        Some(a) => a,
        None => String::new(),
    }
}
// }}}

fn get_item_id(args: &mut Args) -> u8 {
    // {{{
    if let Ok(id) = get_next_arg(args).parse() {
        id
    } else {
        0
    }
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
            eprintln!("Please provide the name of the list you wish to operate on\n");
            Cli::print_help();
        }
        cli.list_name = arg;
    }
}
// }}}

fn parse_item_operations(cli: &mut Cli, args: &mut Args) {
    // {{{
    let arg = get_next_arg(args);

    if arg.is_empty() {
        return;
    }

    if arg == "add" {
        let item = get_next_arg(args);

        if item.is_empty() {
            eprintln!("Please provide the id of the item\n");
            Cli::print_help();
        }
        cli.item = item;
        cli.add = true;
    } else if arg == "delete" {
        let item_id = get_item_id(args);

        if item_id == 0 {
            eprintln!("Please provide a valid item id\n");
            Cli::print_help();
        }
        cli.item_id = item_id;
        cli.delete = true;
    } else if arg == "append" {
        let item_id = get_item_id(args);
        let content = get_next_arg(args);

        if item_id == 0 {
            eprintln!("Please provide a valid item id\n");
            Cli::print_help();
        }
        if content.is_empty() {
            eprintln!("Please provide the content you wish to append to the item\n");
            Cli::print_help();
        }

        cli.item_id = item_id;
        cli.item = content;
        cli.append = true;
    }
}
// }}}
