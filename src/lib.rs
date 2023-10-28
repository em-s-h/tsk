use clap::Parser;

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
    #[arg(required = true, index = 1)]
    pub list_name: String,

    /// Create a list
    #[arg(short, long, required = false)]
    pub create: bool,

    /// Delete a list
    #[arg(short, long, required = false)]
    pub delete: bool,

    /// The item to add to a list
    #[arg(index = 2, default_value = "-")]
    pub item: String,

    /// The id of an item, used when removing, moving or editing
    #[arg(index = 3, default_value_t = 0)]
    pub id: u8,

    /// Add an item to a list
    #[arg(short, long, required = false)]
    pub add: bool,

    /// Remove an item from a list
    #[arg(short, long, required = false)]
    pub remove: bool,

    /// Don't ask for confirmation when deleting or removing
    #[arg(long, required = false)]
    pub no_confirmation: bool,
    // }}}
}

pub fn run(args: Args) {
    // {{{
    if args.create {
        lists::create_list(&args.list_name);
    } else if args.delete {
        lists::delete_list(&args.list_name, args.no_confirmation);
    } else {
        lists::print_list(&args.list_name);
    }
    // }}}
}

#[cfg(test)]
mod test {
    // // {{{
    // // }}}
}
