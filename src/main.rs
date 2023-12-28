use directories::{self, ProjectDirs};
use std::{fs::File, path::Path, process};
use tsk::cli::Cli;

fn main() {
    let proj =
        ProjectDirs::from("tsk", "Emilly", "tsk").expect("Unable to create project directory");

    let data_dir = proj.data_local_dir();
    let mut tasks_file = data_dir.to_path_buf();
    tasks_file.push("tasks");

    match Path::try_exists(&tasks_file) {
        Ok(false) | Err(_) => {
            println!("Creating new 'tasks' file\n");

            let f = format!("{}/tasks", data_dir.to_str().unwrap());
            File::create(f).is_err().then(|| {
                eprintln!("Unable to create 'tasks' file");
                process::exit(1);
            });
        }
        _ => (),
    }

    let cli = Cli::new().parse_args();
    tsk::run(cli);
}
