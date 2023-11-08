use clist::cli::Cli;

fn main() {
    let cli = Cli::new().parse_args();

    // println!("{:?}", args);
    clist::run(cli);
}
