use directories::{self, ProjectDirs};
use std::{fs::File, process};
use tsk::cli::Cli;

fn main() {
    let proj =
        ProjectDirs::from("tsk", "Emilly", "tsk").expect("Unable to create project directory");

    let data_dir = proj.data_local_dir();
    let mut dir_entries = data_dir
        .read_dir()
        .expect("Unable to read contents of project directory")
        .peekable();

    if dir_entries.peek().is_none() {
        println!("Creating new 'tasks' file\n");

        let f = format!("{}/tasks", data_dir.to_str().unwrap());
        File::create(f).is_err().then(|| {
            eprintln!("Unable to create 'tasks' file");
            process::exit(1);
        });
    } else if let Ok(f) = dir_entries.next().unwrap() {
        if f.file_name() != "tasks" {
            eprintln!("Please make sure only one 'tasks' file");
            eprintln!("is located at {}", data_dir.display());
            process::exit(1)
        }
    }

    let cli = Cli::new().parse_args();
    tsk::run(cli);
}
