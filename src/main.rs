use tsk::cli::Cli;

fn main() {
    let cli = Cli::new().parse_args();
    tsk::run(cli);
}
