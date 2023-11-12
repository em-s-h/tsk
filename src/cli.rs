use std::{
    env::{self, Args},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    process,
};

#[derive(Debug)]
pub struct Cli {
    // {{{
    /// Name of the list to operate on
    pub list_name: String,

    /// Used when renaming a list
    pub old_list_name: String,

    /// The item that will be added to the list
    pub item: String,

    /// Id (line number) of the item
    pub item_id: u8,

    /// Used when moving items
    pub new_id: u8,

    pub print_help: bool,

    /// Create the list
    pub create: bool,

    /// Show all the lists the user created
    pub show_lists: bool,

    /// Print the contents of the list
    pub print: bool,

    /// Rename the list
    pub rename: bool,

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
            old_list_name: String::new(),
            item: String::new(),
            item_id: 0,
            new_id: 0,
            print_help: false,

            // lists
            create: false,
            print: false,
            show_lists: false,
            rename: false,
            confirmed: false,
            remove: false,

            // items
            add: false,
            append: false,
            edit: false,
            move_item: false,
            delete: false,
        }
    }
    // }}}

    pub fn parse_args(mut self) -> Self {
        // {{{
        let mut args = env::args();
        args.next(); // First argument is unneeded

        let mut arg = get_next_arg(&mut args);
        if arg == "--help" || arg == "-h" {
            self.print_help = true;
            return self;
        } else if arg.is_empty() || arg == "--show-lists" {
            self.show_lists = true;
            return self;
        }

        // Parse list related arguments {{{
        if arg == "create" || arg == "c" {
            self.create = true;
        } else if arg == "print" || arg == "p" {
            self.print = true;
        } else if arg == "rename" || arg == "ren" {
            self.rename = true;
        } else if arg == "remove" || arg == "r" {
            self.remove = true;
        } else {
            self.list_name = arg;
        }

        if self.create || self.print || self.rename || self.remove {
            let arg = get_next_arg(&mut args);

            if arg.is_empty() {
                eprintln!("Please provide the name of the list you wish to operate on");
                process::exit(1);
            }
            self.list_name = arg;
        }

        // Check if the list exists {{{
        if !self.create {
            let path = crate::get_path(&self.list_name);

            let list_exists = {
                let path = Path::new(&path);
                match path.try_exists() {
                    Ok(result) => result,
                    _ => false,
                }
            };

            if !list_exists {
                eprintln!("This list doesn't exist!");
                process::exit(1);
            }
        }
        // }}}

        if self.rename {
            let arg = get_next_arg(&mut args);

            if arg.is_empty() {
                eprintln!("Please provide the new name of the list");
                process::exit(1);
            } else if arg == self.list_name {
                eprintln!("Please provide different list names");
                process::exit(1);
            }
            self.old_list_name = self.list_name;
            self.list_name = arg;
        }

        if self.remove {
            let arg = get_next_arg(&mut args);
            if arg == "y" {
                self.confirmed = true
            }
            return self;
        }
        // }}}

        // Parse item related arguments {{{
        /// Makes sure the id is not above the amount of lines in a list
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

        arg = get_next_arg(&mut args);
        if arg.is_empty() {
            return self;
        }

        if arg == "add" || arg == "a" {
            self.add = true;
        } else if arg == "append" || arg == "ap" {
            self.append = true;
        } else if arg == "edit" || arg == "e" {
            self.edit = true
        } else if arg == "move" || arg == "m" {
            self.move_item = true;
        } else if arg == "delete" || arg == "d" {
            self.delete = true;
        }

        if self.append || self.edit || self.move_item || self.delete {
            self.item_id = get_item_id(&mut args);

            check_id(&self.list_name, self.item_id);
            // -1 because lines are counted from 0
            self.item_id -= 1;
        }

        if self.add || self.append || self.edit {
            self.item = get_item_content(&mut args);
        }

        if self.move_item {
            self.new_id = get_item_id(&mut args);

            check_id(&self.list_name, self.new_id);
            // -1 because lines are counted from 0
            self.new_id -= 1;

            if self.new_id == self.item_id {
                eprintln!("Please provide different item ids");
                process::exit(1);
            }
        }
        // }}}

        self
    }
    // }}}

    pub fn print_help() {
        // {{{
        println!(
            "
Made by: Emilly M.S./S.H.

clist: A list manager for the CLI.

Usage: clist [Options] [List command] [Item command] [Args]...

Options:
    --help -h
        Print this message
    --show-lists
        Print the names of the available lists
        Default when no args are passed

List commands:
    create c <list_name>
        Create the list
    print p <list_name>
        Print the contents of the list
        Default when only passing the name of the list
    rename ren <old_list_name> <new_list_name>
        Rename the list
    remove r <list_name>
        Remove the list

Item commands:
    add a <item>
        Add an item to the list
    append ap <item_id> <content>
        Append contents to an existing item
    edit e <item_id> <new_item>
        Replace the contents of an item
    move m <item_id> <new_item_id>
        Move an item to a new location
    delete d <item_id>
        Delete an item from the list
        "
        );
    }
    // }}}
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
