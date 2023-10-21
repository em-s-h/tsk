use clap::Parser;
use clist::Args;

fn main() {
    let args = Args::parse();
    let args = clist::check_args(args);
    clist::run(args);
}
