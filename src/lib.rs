use crate::cli::Cli;
use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    process,
};

pub mod cli;

const TASK_LIST: &str = "/.local/share/tsk/task_list";

pub fn run(cli: Cli) {
    // {{{
    let list = get_list();

    if cli.print_help {
        Cli::print_help();
        process::exit(0);
    } else if cli.print_version {
        Cli::print_version();
        process::exit(0);
    } else if cli.print {
        print_tasks(&list, cli.colored_output);
        process::exit(0);
    }

    // Operations {{{
    if cli.add {
        println!("Adding task...");
        add_task(&list, &cli.task);
    } else if cli.mark_done {
        // Mark task as done operation {{{
        let operation = |writer: &mut BufWriter<File>, id: usize, ln: String| {
            if cli.task_ids.contains(&id) && !ln.contains("[X]") {
                writeln!(writer, "{ln} [X]").expect("Unable to write to tmp file");
            } else {
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            }
        };
        // }}}

        println!("Marking tasks as done...");
        operate_list(&list, operation);
    } else if cli.unmark_done {
        // Unmark task as done operation {{{
        let operation = |writer: &mut BufWriter<File>, id: usize, ln: String| {
            let ln = if cli.task_ids.contains(&id) {
                ln.replace("[X]", "").trim().to_string()
            } else {
                ln
            };
            writeln!(writer, "{ln}").expect("Unable to write to tmp file");
        };
        // }}}

        println!("Unmarking tasks...");
        operate_list(&list, operation);
    } else if cli.clear_dones {
        // Remove all tasks marked as done operation {{{
        let operation = |writer: &mut BufWriter<File>, _id: usize, ln: String| {
            if !ln.contains("[X]") {
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            }
        };
        // }}}

        println!("Clearing tasks...");
        operate_list(&list, operation);
    } else if cli.append {
        // Append to a task operation {{{
        let operation = |writer: &mut BufWriter<File>, id: usize, ln: String| {
            if cli.task_ids[0] == id {
                let ln = ln
                    .contains("[X]")
                    .then(|| ln.replace("[X]", ""))
                    .unwrap_or(ln);

                writeln!(writer, "{ln}{}", cli.task).expect("Unable to write to tmp file");
            } else {
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            }
        };
        // }}}

        println!("Appending content...");
        operate_list(&list, operation);
    } else if cli.edit {
        // Edit a task operation {{{
        let operation = |writer: &mut BufWriter<File>, id: usize, ln: String| {
            if cli.task_ids[0] == id {
                writeln!(writer, "{}", cli.task).expect("Unable to write to tmp file");
            } else {
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            }
        };
        // }}}

        println!("Editing task...");
        operate_list(&list, operation);
    } else if cli.move_task {
        // Move a task operation {{{
        let operation = |writer: &mut BufWriter<File>, id: usize, ln: String| {
            if cli.new_id == id {
                let movin_up = cli.task_ids[0] > cli.new_id;
                let task = {
                    // Get task to be moved {{{
                    let f = File::open(get_list()).expect("Unable to open file for reading");
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
        operate_list(&list, operation);
    } else if cli.delete {
        // Delete a task operation {{{
        let operation = |writer: &mut BufWriter<File>, id: usize, ln: String| {
            if cli.task_ids[0] != id {
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            }
        };
        // }}}

        println!("Deleting task...");
        operate_list(&list, operation);
    }
    // }}}

    print_tasks(&list, cli.colored_output);
}
// }}}

fn operate_list<F>(list: &str, operation: F)
where
    F: Fn(&mut BufWriter<File>, usize, String),
{
    // {{{
    let out_list = list.to_string() + ".tmp";

    // Scope ensures files are closed
    {
        let file = File::open(list).expect("Unable to open list for reading");
        let out_file = File::create(&out_list).expect("Unable to create output file");

        let reader = BufReader::new(file);
        let mut writer = BufWriter::new(out_file);

        for (id, ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
            operation(&mut writer, id, ln);
        }
    }
    fs::rename(out_list, list).expect("Unable to rename tmp file");
}
// }}}

fn print_tasks(list: &str, colored: bool) {
    // {{{
    fn is_done(ln: &str) -> bool {
        // {{{
        if ln.contains("[X]") {
            true
        } else {
            false
        }
    }
    // }}}

    println!("Tasks:\n");

    let file = File::open(list).expect("Unable to open file for reading");
    let reader = BufReader::new(file);

    for (id, ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
        let id = id + 1;

        if is_done(&ln) && colored {
            println!("{id}. \x1b[0;32m{ln} \x1b[0m");
        } else if colored {
            println!("{id}. \x1b[0;31m{ln} \x1b[0m");
        } else {
            println!("{id}. {ln}");
        }
    }
}
// }}}

fn add_task(list: &str, task: &str) {
    // {{{
    let task = task.to_string() + "\n";

    let mut list = OpenOptions::new()
        .append(true)
        .open(list)
        .expect("Unable to open file for writting");

    list.write_all(task.as_bytes())
        .expect("Unable to write to file");
}
// }}}

/// Get the task list
fn get_list() -> String {
    // {{{
    // Windows support isn't planned
    if let Some(h) = env::home_dir() {
        let h = h.to_str().unwrap();
        format!("{h}{TASK_LIST}")
    } else {
        eprintln!("Unable to obtain home directory");
        process::exit(1);
    }
}
// }}}
