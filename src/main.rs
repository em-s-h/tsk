use clap::Parser;
use clist::Args;
use std::process;

fn main() {
    let mut args = Args::parse();

    args.list_name = args
        .list_name
        .trim()
        .replace([' ', '\n', '\t'], "")
        .to_owned();

    if args.create && args.remove {
        eprintln!("Cannot create and remove a list at the same time!");
        process::exit(1);
    }

    clist::run(args);
}
