use crate::cli::Cli;
use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    process,
};

/// Handles the data of the program
pub mod cli;

const TASK_LIST: &str = "/.local/share/tsk/task_list";

pub fn run(cli: Cli) {
    // {{{
    let list = get_list();

    if cli.print_help {
        Cli::print_help();
        process::exit(0);
    }

    if cli.print {
        print_tasks(&list, cli.colored_output);
        process::exit(0);
    }

    if cli.add {
        add_task(&list, &cli.task);
        println!("Task added");
    } else if cli.append {
        append_to_task(&list, cli.task_id, &cli.task);
        println!("Content appended");
    } else if cli.edit {
        edit_task(&list, cli.task_id, &cli.task);
        println!("Task edited");
    } else if cli.move_task {
        move_task(&list, cli.task_id, cli.new_id);
        println!("Task moved");
    } else if cli.delete {
        delete_task(&list, cli.task_id);
        println!("Task deleted");
    }

    print_tasks(&list, cli.colored_output);
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
            println!("\x1b[0;32m{id}. {ln} \x1b[0m");
        } else if colored {
            println!("\x1b[0;31m{id}. {ln} \x1b[0m");
        } else {
            println!("{id}. {ln}");
        }
    }
}
// }}}

fn mark_done(list: &str, id: u8) {
    // {{{
    let out_list = list.to_string() + ".tmp";

    // Scope ensures files are closed
    {
        let (reader, mut writer) = prep_files(list, &out_list);

        for (i, ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
            if i == id.into() {
                writeln!(writer, "[X] {ln}").expect("Unable to write to tmp file");
            } else {
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            }
        }
    }
    fs::rename(out_list, list).expect("Unable to rename tmp file");
}
// }}}

fn unmark_done(list: &str, id: u8) {
    // {{{
    let out_list = list.to_string() + ".tmp";

    // Scope ensures files are closed
    {
        let (reader, mut writer) = prep_files(list, &out_list);

        for (i, mut ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
            if i == id.into() {
                ln = ln.replace("[X]", "");
            }
            writeln!(writer, "{ln}").expect("Unable to write to tmp file");
        }
    }
    fs::rename(out_list, list).expect("Unable to rename tmp file");
}
// }}}

fn clear_done(list: &str) {
    // {{{
    let out_list = list.to_string() + ".tmp";

    // Scope ensures files are closed
    {
        let (reader, mut writer) = prep_files(list, &out_list);

        for ln in reader.lines().map(|l| l.unwrap()) {
            if ln.contains("[X]") {
                continue;
            } else {
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            }
        }
    }
    fs::rename(out_list, list).expect("Unable to rename tmp file");
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

fn append_to_task(list: &str, id: u8, content: &str) {
    // {{{
    let out_list = list.to_string() + ".tmp";

    // Scope ensures files are closed
    {
        let (reader, mut writer) = prep_files(list, &out_list);

        for (i, ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
            if i == id.into() {
                writeln!(writer, "{ln}{content}").expect("Unable to write to tmp file");
            } else {
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            }
        }
    }
    fs::rename(out_list, list).expect("Unable to rename tmp file");
}
// }}}

fn edit_task(list: &str, id: u8, new_content: &str) {
    // {{{
    let out_list = list.to_string() + ".tmp";

    // Scope ensures files are closed
    {
        let (reader, mut writer) = prep_files(list, &out_list);

        for (i, ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
            if i == id.into() {
                writeln!(writer, "{new_content}").expect("Unable to write to tmp file");
            } else {
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            }
        }
    }
    fs::rename(out_list, list).expect("Unable to rename tmp file");
}
// }}}

fn move_task(list: &str, from: u8, to: u8) {
    // {{{
    let task = {
        // Get task to be moved {{{
        let f = File::open(list).expect("Unable to open file for reading");
        let reader = BufReader::new(f);
        let mut task = String::new();

        for (i, ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
            if i == from.into() {
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

    let out_list = list.to_string() + ".tmp";

    // Scope ensures files are closed
    {
        let (reader, mut writer) = prep_files(list, &out_list);

        for (i, ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
            if i == from.into() {
                continue;
            } else if i == to.into() {
                writeln!(writer, "{task}").expect("Unable to write to tmp file");
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            } else {
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            }
        }
    }
    fs::rename(out_list, list).expect("Unable to rename tmp file");
}
// }}}

fn delete_task(list: &str, id: u8) {
    // {{{
    let out_list = list.to_string() + ".tmp";

    // Scope ensures files are closed
    {
        let (reader, mut writer) = prep_files(list, &out_list);

        for (i, ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
            if i != id.into() {
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            }
        }
    }
    fs::rename(out_list, list).expect("Unable to rename tmp file");
}
// }}}

fn prep_files(read_file: &str, out_file: &str) -> (BufReader<File>, BufWriter<File>) {
    // {{{
    let file = File::open(read_file).expect("Unable to open list for reading");
    let out_file = File::create(out_file).expect("Unable to create output file");

    let reader = BufReader::new(file);
    let writer = BufWriter::new(out_file);

    (reader, writer)
}
// }}}

fn get_list() -> String {
    // {{{
    if let Some(h) = env::home_dir() {
        let h = h.to_str().unwrap();
        format!("{h}{TASK_LIST}")
    } else {
        eprintln!("Unable to obtain home directory");
        process::exit(1);
    }
}
// }}}

#[cfg(test)]
mod test {
    // {{{
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn add_task_ok() {
        // {{{
        let list = crate::get_list();
        add_task(&list, &"new task");

        let f = File::open(&list).unwrap();
        let last_line = get_last_line(f);
        delete_task(&list, 1);

        assert_eq!(last_line, "new task");
    }
    // }}}

    #[test]
    fn delete_task_ok() {
        // {{{
        let list = crate::get_list();
        add_task(&list, &"new task");
        delete_task(&list, 1);

        let f = File::open(&list).unwrap();
        let last_line = get_last_line(f);

        assert_ne!(last_line, "new task");
    }
    // }}}

    #[test]
    fn append_task_ok() {
        // {{{
        let list = crate::get_list();
        add_task(&list, "with");
        append_to_task(&list, 1, " addition");

        let f = File::open(&list).unwrap();
        let last_line = get_last_line(f);
        delete_task(&list, 1);

        assert_eq!(last_line, "with addition");
    }
    // }}}

    #[test]
    fn edit_task_ok() {
        // {{{
        let list = crate::get_list();
        add_task(&list, "original");
        edit_task(&list, 1, "new!!");

        let f = File::open(&list).unwrap();
        let last_line = get_last_line(f);
        delete_task(&list, 1);

        assert_eq!(last_line, "new!!");
    }
    // }}}

    fn get_last_line(file: File) -> String {
        // {{{
        BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .last()
            .unwrap()
    }
    // }}}
}
// }}}
