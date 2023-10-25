use clap::Parser;
use std::{fs::File, process};

mod lists;

#[derive(Parser, Debug, PartialEq)]
#[command(version, long_about = None)]
#[command(about = "Manage lists in the CLI")]
#[command(author = "Emilly S.H./M.S.")]
#[command(next_line_help = true)]
pub struct Args {
    // {{{
    /// Name of the list to operate on.
    /// Pass without flags to print the list
    #[arg(required = true)]
    pub list_name: String,

    /// Create a list
    #[arg(short, long, required = false)]
    pub create: bool,

    /// Delete a list
    #[arg(short, long, required = false)]
    pub delete: bool,

    /// The id of an item, used when removing, moving or editing
    #[arg(required = false, index = 2)]
    pub id: u8,

    /// The item to add to a list
    #[arg(required = false)]
    pub item: String,

    /// Add an item to a list
    #[arg(short, long, required = false, index = 1)]
    pub add: bool,

    /// Remove an item from a list
    #[arg(short, long, required = false, index = 0)]
    pub remove: bool,

    /// Don't ask for confirmation when deleting or removing
    #[arg(long, required = false)]
    pub no_confirmation: bool,
    // }}}
}

pub fn run(args: Args) {
    // {{{
    let list: File;

    if args.create {
        list = lists::create_list(&args.list_name);
    } else if args.delete {
        lists::delete_list(&args.list_name, args.no_confirmation);
    } else {
        lists::print_list(&args.list_name);
    }
    // }}}
}

pub fn check_args(args: Args) -> Args {
    // {{{
    args
    // }}}
}

#[cfg(test)]
mod test {
    // // {{{
    // use crate::{check_args, Args};
    //
    // // check_args {{{
    // #[test]
    // fn check_args_success() {
    //     let messy_args = Args {
    //         list_name: "aa".to_string(),
    //         create: true,
    //         delete: false,
    //         add: None,
    //         remove: None,
    //         no_confirmation: false,
    //     };
    //
    //     let correct_args = Args {
    //         list_name: "aa".to_string(),
    //         create: true,
    //         delete: false,
    //         add: None,
    //         remove: None,
    //         no_confirmation: false,
    //     };
    //
    //     assert_eq!(correct_args, check_args(messy_args));
    // }
    // // }}}
    //
    // // }}}
}
