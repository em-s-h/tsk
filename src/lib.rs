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

    /// Remove a list
    #[arg(short, long, required = false)]
    pub remove: bool,

    /// Add an item to a list
    #[arg(short, long, required = false, default_value = "")]
    pub add: String,

    /// Delete an item from a list
    #[arg(short, long, required = false, default_value_t = 0)]
    pub delete: u8,

    /// Don't ask for confirmation when deleting or removing
    #[arg(long, required = false)]
    pub no_confirmation: bool,
    // }}}
}

pub fn run(args: Args) {
    // {{{
    if args.create {
        lists::create_list(&args.list_name);
    } else if args.remove {
        lists::remove_list(&args.list_name, args.no_confirmation);
    }

    if !args.add.is_empty() {
        lists::add_item(&args.list_name, &args.add);
    } else if args.delete != 0 {
        lists::delete_item(&args.list_name, args.delete);
    }

    lists::print_list(&args.list_name);
    // }}}
}

#[cfg(test)]
mod test {
    // // {{{
    // // }}}
}
