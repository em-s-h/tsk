use clist::cli::Cli;

fn main() {
    let _cli = Cli::new().parse_args();

    // args.list_name = args
    //     .list_name
    //     .trim()
    //     .replace([' ', '\n', '\t'], "")
    //     .to_owned();
    //
    // if (args.create || args.add != "n/a") && args.remove {
    //     eprintln!("Cannot create or add an item and remove a list at the same time");
    //     process::exit(1);
    // }
    //
    // if args.list_name != "n/a" && args.show_lists {
    //     args.show_lists = false;
    // }

    // println!("{:?}", args);
    // clist::run(args);
}
