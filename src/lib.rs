use clap::Parser;
use std::process;

#[derive(Parser, Debug, PartialEq)]
#[command(version, long_about = None)]
#[command(author = "Emilly S.H./M.S.")]
#[command(next_line_help = true)]
pub struct Args {
    // {{{
    /// The name of the list
    pub list_name: String,

    /// Create a list
    #[arg(short, long, required = false)]
    pub create: bool,

    /// Delete a list
    #[arg(short, long, required = false)]
    pub delete: bool,

    /// Print a list (default)
    #[arg(short, long, required = false, default_value_t = true)]
    pub print: bool,

    /// Don't ask for confirmation when deleting or removing
    #[arg(long, required = false)]
    pub no_confirmation: bool,
    // }}}
}

pub fn run(args: Args) {
    // {{{
    println!("{:?}", args);
    // }}}
}

/// Verify that args are valid and organize them if possible
/// Ex.: create and delete flags can't both be on
pub fn check_args(mut args: Args) -> Args {
    // {{{
    use std::io::{self, Write};

    if args.create && args.delete {
        eprintln!("Cannot create and delete a list at the same time!");
        process::exit(1);
    }

    if args.create || args.delete {
        args.print = false;
    }

    if args.delete && !args.no_confirmation {
        println!("Are you sure you want to delete {}?", args.list_name);
        print!("(y/n): ");

        io::stdout().flush().expect("Unable to flush stdout");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read from stdin");

        if input.to_lowercase().starts_with("n") {
            println!("Aborting...");
            process::exit(0);
        }
    }
    args
    // }}}
}

#[cfg(test)]
mod test {
    // {{{
    use crate::{check_args, Args};

    // check_args {{{
    #[test]
    fn check_args_success() {
        let messy_args = Args {
            list_name: "aa".to_string(),
            create: true,
            delete: false,
            print: true,
            no_confirmation: false,
        };

        let correct_args = Args {
            list_name: "aa".to_string(),
            create: true,
            delete: false,
            print: false,
            no_confirmation: false,
        };

        assert_eq!(correct_args, check_args(messy_args));
    }
    // }}}
    // }}}
}
