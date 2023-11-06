use std::env::{self, Args};

pub struct Cli {
    // {{{
    /// Name of the list to operate on
    /// Pass without flags to print the list
    pub list_name: String,

    /// Create the list <list_name>
    pub create: bool,

    /// Remove the list <list_name>
    pub remove: bool,

    /// Add an item to a list
    pub item: String,

    /// Delete an item from a list
    pub item_id: u8,

    /// Don't ask for confirmation when deleting or removing
    pub no_confirmation: bool,

    /// Show all the available lists to operate on
    /// Default when no arguments are provided
    pub show_lists: bool,
}
// }}}

impl Cli {
    // {{{
    pub fn new() -> Self {
        // {{{
        Self {
            list_name: String::new(),
            create: false,
            remove: false,
            item: String::new(),
            item_id: 0,
            no_confirmation: false,
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

        self
    }

    pub fn print_help() {}
    // }}}
}
// }}}

fn parse_list_operations(cli: &mut Cli, args: &mut Args) {
    // {{{
    let arg = if let Some(a) = args.next() {
        a
    } else {
        String::new()
    };

    if !arg.is_empty() {
        cli.show_lists = false;
    }

    if arg == "create" || arg == "c" {
        cli.create = true;
    } else if arg == "remove" || arg == "r" {
        cli.remove = true;
    } else {
        cli.list_name = arg;
    }

    if cli.remove || cli.create {
        let arg = if let Some(a) = args.next() {
            a
        } else {
            String::new()
        };

        if arg.is_empty() {
            eprintln!("Please provide the name of the list you wish to operate on");
            Cli::print_help();
        }

        cli.list_name = arg;
    }
}
// }}}

fn parse_item_operations(cli: &mut Cli, args: &mut Args) {
    // {{{
}
// }}}
