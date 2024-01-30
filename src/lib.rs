use directories::ProjectDirs;

use crate::cli::{AddOpt, Cli};
use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
    process,
};

pub mod cli;

#[derive(Debug, PartialEq)]
enum MarkType {
    // {{{
    Done,
    Undone,
}
// }}}

pub fn run(cli: Cli) {
    // {{{
    let task_f = get_task_file();

    if cli.print {
        print_tasks(&task_f, cli.colored_output);
        process::exit(0)
    }

    // Operations {{{
    if cli.add {
        add_task(&task_f, &cli.task, &cli.add_to)
    } else if cli.mark_done {
        mark_task(&task_f, &cli.task_ids, MarkType::Done)
    } else if cli.unmark_done {
        mark_task(&task_f, &cli.task_ids, MarkType::Undone)
    } else if cli.clear_dones {
        clear_dones(&task_f)
    } else if cli.append {
        append_to_task(&task_f, cli.task_ids[0], &cli.task)
    } else if cli.edit {
        edit_task(&task_f, cli.task_ids[0], &cli.task)
    } else if cli.move_task {
        move_task(&task_f, cli.task_ids[0], cli.new_id)
    } else if cli.delete {
        delete_task(&task_f, cli.task_ids[0])
    }
    // }}}

    print_tasks(&task_f, cli.colored_output);
}
// }}}

fn print_tasks(task_f: &PathBuf, colored: bool) {
    // {{{
    let meta = fs::metadata(task_f).unwrap_or_else(|e| {
        eprintln!("Unable to obtain file metadata");
        eprintln!("Err: {e}");
        process::exit(1)
    });
    if meta.len() == 0 {
        println!("No tasks to print");
        process::exit(0);
    }
    println!("Tasks:\n");

    let file = File::open(task_f).expect("File has been verified to be readable");
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

fn operate_task_file<F>(task_f: &PathBuf, operation: F)
// {{{
where
    F: Fn(&mut BufWriter<File>, usize, String),
{
    let out_task_f = {
        // {{{
        let mut tmp = task_f.clone();
        tmp.pop();
        tmp.push("tasks.tmp");
        tmp
    };
    // }}}

    let out_file = File::create(&out_task_f).unwrap_or_else(|e| {
        eprintln!("Unable to create tmp output file");
        eprintln!("Err: {e}");
        process::exit(1)
    });
    let meta = fs::metadata(task_f).unwrap_or_else(|e| {
        eprintln!("Unable to obtain file metadata");
        eprintln!("Err: {e}");
        process::exit(1)
    });
    let mut writer = BufWriter::new(out_file);

    if meta.len() != 0 {
        let file = File::open(task_f).expect("File has been verified to be readable");
        let reader = BufReader::new(file);

        for (id, ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
            operation(&mut writer, id, ln);
        }
    } else {
        operation(&mut writer, 0, String::new());
    }

    fs::rename(out_task_f, task_f).unwrap_or_else(|e| {
        eprintln!("Unable to rename tmp file");
        eprintln!("Err: {e}");
        process::exit(1)
    });
}
// }}}

fn add_task(task_f: &PathBuf, task: &str, place: &AddOpt) {
    // {{{
    println!("Adding task...");

    if *place == AddOpt::Top {
        operate_task_file(
            &task_f,
            |writer: &mut BufWriter<File>, id: usize, ln: String| {
                if id == 0 {
                    writeln!(writer, "[ ] {}", task)
                        .expect("File has been verified to be writable");
                }
                if !ln.is_empty() {
                    writeln!(writer, "{ln}").expect("File has been verified to be writable");
                }
            },
        );
    } else {
        let s = format!("[ ] {}\n", task);
        let mut f = OpenOptions::new()
            .append(true)
            .open(&task_f)
            .expect("File has been verified to be openable");

        f.write_all(s.as_bytes()).unwrap_or_else(|e| {
            eprintln!("Error while writting to 'tasks' file");
            eprintln!("Err: {e}");
            process::exit(1)
        })
    }
}
// }}}

fn mark_task(task_f: &PathBuf, ids: &Vec<usize>, m_type: MarkType) {
    // {{{
    let (pat, new) = if m_type == MarkType::Done {
        println!("Marking tasks as done...");
        ("[ ]", "[X]")
    } else {
        println!("Unmarking tasks...");
        ("[X]", "[ ]")
    };

    operate_task_file(
        &task_f,
        |writer: &mut BufWriter<File>, id: usize, ln: String| {
            let ln = if ids.contains(&id) && ln.contains(pat) {
                ln.replace(pat, new)
            } else {
                ln
            };
            writeln!(writer, "{ln}").expect("File has been verified to be writable");
        },
    );
}
// }}}

fn move_task(task_f: &PathBuf, from: usize, to: usize) {
    // {{{
    println!("Moving task...");

    let task = {
        // Get task to be moved {{{
        let f = File::open(task_f).expect("File has been verified to be readable");
        let reader = BufReader::new(f);
        let mut task = String::new();

        for (i, ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
            if i == from {
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

    operate_task_file(
        &task_f,
        |writer: &mut BufWriter<File>, id: usize, ln: String| {
            if to == id {
                let movin_up = from > to;

                // Ex.: 1 -> 3
                if !movin_up {
                    writeln!(writer, "{ln}").expect("File has been verified to be writable");
                }
                writeln!(writer, "{task}").expect("File has been verified to be writable");

                // Ex.: 3 -> 1
                if movin_up {
                    writeln!(writer, "{ln}").expect("File has been verified to be writable");
                }
            } else if from != id {
                writeln!(writer, "{ln}").expect("File has been verified to be writable");
            }
        },
    );
}
// }}}

fn append_to_task(task_f: &PathBuf, id: usize, content: &str) {
    // {{{
    println!("Appending content...");

    operate_task_file(
        &task_f,
        |writer: &mut BufWriter<File>, f_id: usize, ln: String| {
            let ln = if id == f_id {
                let ln = ln
                    .contains("[X]")
                    .then(|| ln.replace("[X]", "[ ]"))
                    .unwrap_or(ln);
                format!("{ln} {content}")
            } else {
                ln
            };
            writeln!(writer, "{ln}").expect("File has been verified to be writable");
        },
    );
}
// }}}

fn edit_task(task_f: &PathBuf, id: usize, new_content: &str) {
    // {{{
    println!("Editing task...");

    operate_task_file(
        &task_f,
        |writer: &mut BufWriter<File>, f_id: usize, ln: String| {
            let ln = if id == f_id {
                format!("[ ] {new_content}")
            } else {
                ln
            };

            writeln!(writer, "{ln}").expect("File has been verified to be writable");
        },
    );
}
// }}}

fn delete_task(task_f: &PathBuf, id: usize) {
    // {{{
    println!("Deleting task...");

    operate_task_file(
        &task_f,
        |writer: &mut BufWriter<File>, f_id: usize, ln: String| {
            if id != f_id {
                writeln!(writer, "{ln}").expect("File has been verified to be writable");
            }
        },
    );
}
// }}}

fn clear_dones(task_f: &PathBuf) {
    // {{{
    println!("Clearing tasks...");

    operate_task_file(
        &task_f,
        |writer: &mut BufWriter<File>, _: usize, ln: String| {
            if !ln.contains("[X]") {
                writeln!(writer, "{ln}").expect("File has been verified to be writable");
            }
        },
    );
}
// }}}

fn get_task_file() -> PathBuf {
    // {{{
    let proj = ProjectDirs::from("tsk", "Emilly", "tsk")
        .expect("Project directory has been verified to exist/be retriavable");

    let data_dir = proj.data_local_dir();
    let dir_entries = data_dir.read_dir().unwrap_or_else(|e| {
        eprintln!("Unable to read contents of the project directory");
        eprintln!("Err: {e}");
        process::exit(1);
    });

    for e in dir_entries {
        match e {
            Ok(f) => {
                if f.file_name() == "tasks" {
                    return f.path();
                }
            }
            Err(e) => {
                eprintln!("Unable to read directory contents");
                eprintln!("Err: {e}");
                process::exit(1)
            }
        }
    }
    eprintln!("Unable to obtain 'tasks' file");
    process::exit(1);
}
// }}}

#[cfg(test)]
mod test {
    // {{{
    use super::*;
    use std::path::PathBuf;

    /// Create a file with 5 undone tasks
    fn get_test_file() -> PathBuf {
        // {{{
        let file = PathBuf::from("/tmp/tasks.test");
        let mut f = File::create(&file).unwrap_or_else(|e| {
            eprintln!("Unable to create test file in /tmp");
            eprintln!("Err: {e}");
            process::exit(1)
        });

        for i in 0..5 {
            let s = format!("[ ] Test line {}\n", i);
            f.write_all(s.as_bytes()).unwrap_or_else(|e| {
                eprintln!("Unable to preparate test file");
                eprintln!("Err: {e}");
                process::exit(1)
            })
        }
        file
    }
    // }}}

    /// Create a file with 5 done tasks
    fn get_done_test_file() -> PathBuf {
        // {{{
        let file = PathBuf::from("/tmp/tasks.test");
        let mut f = File::create(&file).unwrap_or_else(|e| {
            eprintln!("Unable to create test file in /tmp");
            eprintln!("Err: {e}");
            process::exit(1)
        });

        for i in 0..5 {
            let s = format!("[X] Test line {}\n", i);
            f.write_all(s.as_bytes()).unwrap_or_else(|e| {
                eprintln!("Unable to preparate test file");
                eprintln!("Err: {e}");
                process::exit(1)
            })
        }
        file
    }
    // }}}

    // 'add_task' tests {{{
    #[test]
    fn add_task_top() {
        // {{{
        let f = get_test_file();
        let task = "Test Add Top";

        add_task(&f, &task, &AddOpt::Top);
        let task = format!("[ ] {task}");

        let f = File::open(&f).unwrap_or_else(|e| {
            eprintln!("Unable to open file");
            eprintln!("Err: {e}");
            process::exit(1);
        });

        let first_ln = BufReader::new(&f)
            .lines()
            .map(|l| l.unwrap())
            .next()
            .unwrap_or_else(|| {
                eprintln!("Unable to obtain first line of file");
                process::exit(1)
            });

        assert_eq!(task, first_ln);
    }
    // }}}

    #[test]
    fn add_task_bot() {
        // {{{
        let f = get_test_file();
        let task = "Test Add Bottom";

        add_task(&f, &task, &AddOpt::Bottom);
        let task = format!("[ ] {task}");

        let f = File::open(&f).unwrap_or_else(|e| {
            eprintln!("Unable to open file");
            eprintln!("Err: {e}");
            process::exit(1);
        });

        let last_ln = BufReader::new(&f)
            .lines()
            .map(|l| l.unwrap())
            .last()
            .unwrap_or_else(|| {
                eprintln!("Unable to obtain last line of file");
                process::exit(1)
            });

        assert_eq!(task, last_ln);
    }
    // }}}
    // }}}

    // 'mark_task' tests {{{
    fn test_mark_done(ids: Vec<usize>) {
        // {{{
        let f = get_test_file();

        mark_task(&f, &ids, MarkType::Done);

        let f = File::open(&f).unwrap_or_else(|e| {
            eprintln!("Unable to open file");
            eprintln!("Err: {e}");
            process::exit(1);
        });

        let lines = BufReader::new(&f).lines().map(|l| l.unwrap());

        for (i, l) in lines.enumerate() {
            if ids.contains(&i) {
                assert!(l.contains("[X]"));
            } else {
                assert!(l.contains("[ ]"))
            }
        }
    }
    // }}}

    fn test_unmark_done(ids: Vec<usize>) {
        // {{{
        let f = get_done_test_file();

        mark_task(&f, &ids, MarkType::Undone);

        let f = File::open(&f).unwrap_or_else(|e| {
            eprintln!("Unable to open file");
            eprintln!("Err: {e}");
            process::exit(1);
        });

        let lines = BufReader::new(&f).lines().map(|l| l.unwrap());

        for (i, l) in lines.enumerate() {
            if ids.contains(&i) {
                assert!(l.contains("[ ]"));
            } else {
                assert!(l.contains("[X]"));
            }
        }
    }
    // }}}

    #[test]
    fn mark_one() {
        test_mark_done(vec![0]);
    }

    #[test]
    fn mark_multiple() {
        test_mark_done(vec![0, 2, 3]);
    }

    #[test]
    fn unmark_one() {
        test_unmark_done(vec![0])
    }

    #[test]
    fn unmark_multiple() {
        test_unmark_done(vec![0, 2, 4])
    }
    // }}}

    // 'append_to_task' tests {{{
    #[test]
    fn test_append_task() {
        // {{{
        let f = get_done_test_file();

        let content = "New content";
        let og_task = "Test line 0";

        append_to_task(&f, 0, content);
        let new_task = format!("[ ] {og_task} {content}");
        let first_ln = {
            // {{{
            let f = File::open(&f).unwrap_or_else(|e| {
                eprintln!("Unable to open file");
                eprintln!("Err: {e}");
                process::exit(1);
            });

            BufReader::new(&f)
                .lines()
                .map(|l| l.unwrap())
                .next()
                .unwrap_or_else(|| {
                    eprintln!("Unable to obtain last line");
                    process::exit(1);
                })
        };
        // }}}

        assert_eq!(new_task, first_ln);
        assert_ne!(og_task, first_ln);
        assert!(!first_ln.contains("[X]"));
    }
    // }}}
    // }}}

    // 'edit_task' tests {{{
    #[test]
    /// Upon editing the new task must be marked undone, (in this test) be different from the
    /// original and must match what was passed to the function
    fn test_edit_task() {
        // {{{
        let f = get_done_test_file();

        let new_task = "New content";
        let og_task = "[X] Task line 0";

        edit_task(&f, 0, new_task);
        let new_task = format!("[ ] {new_task}");
        let first_ln = {
            // {{{
            let f = File::open(&f).unwrap_or_else(|e| {
                eprintln!("Unable to open file");
                eprintln!("Err: {e}");
                process::exit(1);
            });

            BufReader::new(&f)
                .lines()
                .map(|l| l.unwrap())
                .next()
                .unwrap_or_else(|| {
                    eprintln!("Unable to obtain last line");
                    process::exit(1);
                })
        };
        // }}}

        assert_eq!(new_task, first_ln);
        assert_ne!(og_task, first_ln);
        assert!(!first_ln.contains("[X]"));
    }
    // }}}
    // }}}

    // 'move_task' tests {{{
    #[test]
    fn test_move_task() {
        let f = get_test_file();
        let first_ln = "[ ] Test line 0";

        move_task(&f, 0, 4);
        let last_ln = {
            // {{{
            let f = File::open(&f).unwrap_or_else(|e| {
                eprintln!("Unable to open file");
                eprintln!("Err: {e}");
                process::exit(1);
            });

            BufReader::new(&f)
                .lines()
                .map(|l| l.unwrap())
                .last()
                .unwrap_or_else(|| {
                    eprintln!("Unable to obtain last line of file");
                    process::exit(1)
                })
        };
        // }}}

        assert_eq!(first_ln, last_ln)
    }
    // }}}

    // 'clear_dones' tests {{{
    #[test]
    fn clear_full_file() {
        // {{{
        let f = get_done_test_file();
        clear_dones(&f);

        let f = File::open(&f).unwrap_or_else(|e| {
            eprintln!("Unable to open file");
            eprintln!("Err: {e}");
            process::exit(1);
        });

        let lines_c = BufReader::new(&f).lines().count();
        assert_eq!(0, lines_c)
    }
    // }}}

    #[test]
    fn clear_one_task() {
        // {{{
        let f = get_test_file();
        {
            let f = File::open(&f).unwrap_or_else(|e| {
                eprintln!("Unable to open file");
                eprintln!("Err: {e}");
                process::exit(1);
            });

            let mut writer = BufWriter::new(&f);
            writeln!(writer, "[X] Done task").unwrap_or_else(|e| {
                eprintln!("Unable to write to test file");
                eprintln!("Err: {e}");
                process::exit(1)
            });
        }
        clear_dones(&f);

        let f = File::open(&f).unwrap_or_else(|e| {
            eprintln!("Unable to open file");
            eprintln!("Err: {e}");
            process::exit(1);
        });

        let lines = BufReader::new(&f).lines().count();

        assert_eq!(5, lines)
    }
    // }}}
    // }}}

    // 'delete_task' tests {{{
    #[test]
    fn test_delete_task() {
        // {{{
        let f = get_test_file();
        delete_task(&f, 0);

        let f = File::open(&f).unwrap_or_else(|e| {
            eprintln!("Unable to open file");
            eprintln!("Err: {e}");
            process::exit(1);
        });

        let ln_c = BufReader::new(&f).lines().count();
        assert_eq!(4, ln_c);
    }
    // }}}
    // }}}
}
// }}}
