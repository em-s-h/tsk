use std::{
    fs::{self, File},
    path::Path,
    process,
};

use tsk::cli::Cli;

fn main() {
    let list = tsk::get_list();
    match Path::new(&list).try_exists() {
        Ok(false) | Err(_) => {
            eprintln!("Unable to find task list, creating a new one\n");
            fs::create_dir_all(list.replace("task_list", ""))
                .is_err()
                .then(|| {
                    eprintln!("Unable to create task directory");
                    process::exit(1);
                });
            File::create(list).is_err().then(|| {
                eprintln!("Unable to create task list");
                process::exit(1);
            });
        }
        _ => (),
    }

    let cli = Cli::new().parse_args();
    tsk::run(cli);
}
