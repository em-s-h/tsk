use directories::{self, ProjectDirs};
use std::{fs::File, path::Path, process};
use tsk::cli::Cli;

fn main() {
    let proj = ProjectDirs::from("tsk", "Emilly", "tsk").unwrap_or_else(|| {
        eprintln!("Unable to retrieve/create the project directory");
        process::exit(1)
    });

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
        Ok(true) => {
            // Check if file has write-read permissions {{{
            use std::fs::OpenOptions;
            match OpenOptions::new().append(true).read(true).open(&tasks_file) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Unable to open 'tasks' file for reading & writting");
                    eprintln!("Err: {e}");
                    process::exit(1)
                }
            }
        } // }}}
    }

    let cli = Cli::new().parse_args();
    tsk::run(cli);
}
