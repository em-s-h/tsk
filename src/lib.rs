use directories::ProjectDirs;

use crate::cli::Cli;
use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
    process,
};

pub mod cli;

pub fn run(cli: Cli) {
    // {{{
    let task_f = get_task_file();

    if cli.print_help {
        Cli::print_help();
        process::exit(0);
    } else if cli.print_version {
        Cli::print_version();
        process::exit(0);
    } else if cli.print {
        print_tasks(&task_f, cli.colored_output);
        process::exit(0);
    }

    // Operations {{{
    if cli.add {
        println!("Adding task...");
        add_task(&task_f, &cli.task);
    } else if cli.mark_done {
        // Mark tasks as done operation {{{
        let operation = |writer: &mut BufWriter<File>, id: usize, ln: String| {
            let ln = if cli.task_ids.contains(&id) && !ln.contains("[X]") {
                ln.replace("[ ]", "[X]")
            } else {
                ln
            };
            writeln!(writer, "{ln}").expect("Unable to write to tmp file");
        };
        // }}}

        println!("Marking tasks as done...");
        operate_task_file(&task_f, operation);
    } else if cli.unmark_done {
        // Unmark tasks as done operation {{{
        let operation = |writer: &mut BufWriter<File>, id: usize, ln: String| {
            let ln = if cli.task_ids.contains(&id) && ln.contains("[X]") {
                ln.replace("[X]", "[ ]")
            } else {
                ln
            };
            writeln!(writer, "{ln}").expect("Unable to write to tmp file");
        };
        // }}}

        println!("Unmarking tasks...");
        operate_task_file(&task_f, operation);
    } else if cli.clear_dones {
        // Remove all tasks marked as done operation {{{
        let operation = |writer: &mut BufWriter<File>, _id: usize, ln: String| {
            if !ln.contains("[X]") {
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            }
        };
        // }}}

        println!("Clearing tasks...");
        operate_task_file(&task_f, operation);
    } else if cli.append {
        // Append to a task operation {{{
        let operation = |writer: &mut BufWriter<File>, id: usize, ln: String| {
            let ln = if cli.task_ids[0] == id {
                ln.contains("[X]")
                    .then(|| ln.replace("[X]", "[ ]"))
                    .unwrap_or(ln)
                    + &cli.task
            } else {
                ln
            };
            writeln!(writer, "{ln}").expect("Unable to write to tmp file");
        };
        // }}}

        println!("Appending content...");
        operate_task_file(&task_f, operation);
    } else if cli.edit {
        // Edit a task operation {{{
        let operation = |writer: &mut BufWriter<File>, id: usize, ln: String| {
            let ln = (cli.task_ids[0] == id)
                .then(|| "[ ] ".to_owned() + &cli.task)
                .unwrap_or(ln);

            writeln!(writer, "{ln}").expect("Unable to write to tmp file");
        };
        // }}}

        println!("Editing task...");
        operate_task_file(&task_f, operation);
    } else if cli.move_task {
        // Move a task operation {{{
        let operation = |writer: &mut BufWriter<File>, id: usize, ln: String| {
            if cli.new_id == id {
                let movin_up = cli.task_ids[0] > cli.new_id;
                let task = {
                    // Get task to be moved {{{
                    let f = File::open(get_task_file()).expect("Unable to open file for reading");
                    let reader = BufReader::new(f);
                    let mut task = String::new();

                    for (i, ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
                        if i == cli.task_ids[0] {
                            task = ln;
                        }
                    }
                    if task.is_empty() {
                        eprintln!("Unable to find the task to move");
                        process::exit(1);
                    }
                    task
                };
                // }}}

                // Ex.: 1 -> 3
                if !movin_up {
                    writeln!(writer, "{ln}").expect("Unable to write to tmp file");
                }
                writeln!(writer, "{task}").expect("Unable to write to tmp file");

                // Ex.: 3 -> 1
                if movin_up {
                    writeln!(writer, "{ln}").expect("Unable to write to tmp file");
                }
            } else if cli.task_ids[0] != id {
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            }
        };
        // }}}

        println!("Moving task...");
        operate_task_file(&task_f, operation);
    } else if cli.delete {
        // Delete a task operation {{{
        let operation = |writer: &mut BufWriter<File>, id: usize, ln: String| {
            if cli.task_ids[0] != id {
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            }
        };
        // }}}

        println!("Deleting task...");
        operate_task_file(&task_f, operation);
    }
    // }}}

    print_tasks(&task_f, cli.colored_output);
}
// }}}

fn operate_task_file<F>(task_f: &PathBuf, operation: F)
where
    F: Fn(&mut BufWriter<File>, usize, String),
{
    // {{{
    let out_task_f = {
        // {{{
        let mut tmp = task_f.clone();
        tmp.pop();
        tmp.push("tasks.tmp");
        tmp
    };
    // }}}

    // Scope ensures files are closed
    {
        let file = File::open(task_f).expect("Unable to open task file for reading");
        let out_file = File::create(&out_task_f).expect("Unable to create output file");

        let reader = BufReader::new(file);
        let mut writer = BufWriter::new(out_file);

        for (id, ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
            operation(&mut writer, id, ln);
        }
    }
    fs::rename(out_task_f, task_f).expect("Unable to rename tmp file");
}
// }}}

fn print_tasks(task_f: &PathBuf, colored: bool) {
    // {{{
    let meta = fs::metadata(task_f).expect("Unable to obtain file metadata");
    if meta.len() == 0 {
        println!("No tasks to print");
        process::exit(0);
    }

    println!("Tasks:\n");

    let file = File::open(task_f).expect("Unable to open file for reading");
    let reader = BufReader::new(file);

    for (id, ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
        let is_done = ln.contains("[X]");
        let id = id + 1;

        if is_done && colored {
            println!("{id}. \x1b[0;32m{ln} \x1b[0m");
        } else if colored {
            println!("{id}. \x1b[0;31m{ln} \x1b[0m");
        } else {
            println!("{id}. {ln}");
        }
    }
}
// }}}

fn add_task(task_f: &PathBuf, task: &str) {
    // {{{
    let task = format!("[ ] {}\n", task.to_string());

    let mut task_f = OpenOptions::new()
        .append(true)
        .open(task_f)
        .expect("Unable to open file for writting");

    task_f
        .write_all(task.as_bytes())
        .expect("Unable to write to file");
}
// }}}

/// Get the tasks file
fn get_task_file() -> PathBuf {
    // {{{
    let proj =
        ProjectDirs::from("tsk", "Emilly", "tsk").expect("Unable to create project directory");

    let data_dir = proj.data_local_dir();
    let dir_entries = data_dir
        .read_dir()
        .expect("Unable to read contents of project directory");

    for e in dir_entries {
        if let Ok(f) = e {
            if f.file_name() == "tasks" {
                let mut path = data_dir.to_path_buf();
                path.push("tasks");
                return path;
            }
        }
    }
    eprintln!("Unable to obtain 'tasks' file");
    process::exit(1);
}
// }}}
