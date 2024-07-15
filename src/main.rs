use std::{
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
    process,
};

use cli::Cli;
use directories::ProjectDirs;
use task_file::TaskFile;

pub mod cli;
pub mod task_file;

fn main() {
    let cli = Cli::new().parse_args();
    let file_path = get_task_file_path();
    let mut task_file = TaskFile::parse_file(&file_path);

    if cli.print {
        task_file.print(cli.colored_output);
        process::exit(0)
    }

    // Operations {{{
    let single_id = cli.task_ids[0].clone();

    if cli.add_task {
        task_file.add_task(&cli.task, &cli.add_to, &single_id)
        //
    } else if cli.mark_done {
        task_file.mark_tasks(&cli.task_ids, true)
    } else if cli.unmark_done {
        task_file.mark_tasks(&cli.task_ids, false)
        //
    } else if cli.move_task {
        task_file.move_task(&single_id, &cli.new_id)
    } else if cli.swap_tasks {
        task_file.swap_tasks(&single_id, &cli.new_id)
        //
    } else if cli.append_task {
        task_file.append_to_task(&single_id, &cli.task)
    } else if cli.edit_task {
        task_file.edit_task(&single_id, &cli.task)
        //
    } else if cli.delete_task {
        task_file.delete_task(&single_id)
    } else if cli.clear_dones {
        task_file.clear_dones()
    }
    // }}}

    task_file.print(cli.colored_output);
    task_file.save_to_file(&file_path)
}

pub fn get_task_file_path() -> PathBuf {
    // {{{
    let proj = ProjectDirs::from("tsk", "Emilly", "tsk").unwrap_or_else(|| {
        eprintln!("Unable to retrieve/create the project directory");
        process::exit(1)
    });

    let data_dir = proj.data_local_dir();
    let mut tasks_file = data_dir.to_path_buf();
    tasks_file.push("tasks.json");

    match Path::try_exists(&tasks_file) {
        Ok(false) | Err(_) => {
            println!("Creating new 'tasks' file\n");

            let f = format!("{}/tasks.json", data_dir.to_str().unwrap());
            File::create(f).is_err().then(|| {
                eprintln!("Unable to create 'tasks' file");
                process::exit(1);
            });
        }
        Ok(true) => match OpenOptions::new().append(true).read(true).open(&tasks_file) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Unable to open 'tasks' file for reading & writting");
                eprintln!("Err: {e}");
                process::exit(1)
            }
        },
    }
    tasks_file
}
// }}}

// Required for integration tests
#[cfg(test)]
mod tests {}
