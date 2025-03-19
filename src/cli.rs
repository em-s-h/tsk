use std::{cmp::Ordering, env, error::Error, fmt::Debug, process};

const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug)]
pub struct Cli {
    /// Make `print` output colored.
    pub colored_output: bool,

    /// The command to run.
    pub command: String,

    /// Where to add a new task.
    pub add_to: String,

    /// Comma separeted list of ids/subids.
    pub task_ids: String,

    /// Single id for commands that move tasks.
    pub move_id: String,

    /// Text for commands that modify a task's content.
    pub contents: String,
}

impl Cli {
    pub fn print_help() {
        // {{{
        println!(
            "{NAME} {VERSION}: {DESCRIPTION}
        Made by: {AUTHOR}

        Usage: {NAME} [Options] [Command] [Sub-Options] [Args]

        Options:
            --help      -h
                Print this message
            --version   -v
                Print the program version
            --no-color
                Don't make the output colored
            --add-to -t=[position] 
                Used by `add`; Values: top, bot[tom]
            --subtask -s=[parent_id]
                Used by `add` to add a task as a subtask.

        Commands:
            print
                Print tasks, default when not passing any args
            add     <task>
                Add a new task.
            do      <task_ids>
                Mark task(s) as done
            undo    <task_ids>
                Unmark task(s) as done
            move    <task_id> <new_task_id>
                Move a task to a new location
            swap    <task_id> <other_task_id>
                Swap the places of two tasks
            append  <task_id> <text>
                Append <text> to an existing task
            edit    <task_id> <new_text>
                Replace the text of the task with <new_text>
            delete  <task_ids>
                Delete 1 task
            clear   
                Delete all tasks that are marked as done
        "
        );
    }

    pub fn parse_args() -> Self {
        let mut cli = Self {
            colored_output: true,
            command: "print".to_owned(),
            add_to: "top".to_owned(),
            task_ids: String::new(),
            move_id: String::new(),
            contents: String::new(),
        };
        let options = env::args().filter(|a| a.starts_with('-'));

        for o in options {
            match o.as_str() {
                "-h" | "--help" => {
                    Cli::print_help();
                    process::exit(0)
                }
                "-v" | "--version" => {
                    println!("{NAME} {VERSION}");
                    process::exit(0)
                }
                "--no-color" => cli.colored_output = false,
                _ => (),
            }
            if !o.contains('=') {
                continue;
            }
            let v: Vec<&str> = o.split('=').collect();
            match v[0] {
                "-t" | "--add-to" => {
                    if v[1] != "top" && v[1] != "bot" && v[1] != "bottom" {
                        eprintln!("Invalid value `{}`", v[1]);
                        process::exit(1)
                    }
                    cli.add_to = v[1].to_string();
                }
                "-s" | "--subtask" => cli.task_ids = v[1].to_string(),
                _ => {
                    eprintln!("Unknown option `{}`", v[0]);
                    process::exit(1)
                }
            }
        }

        let mut args = env::args().filter(|a| !a.starts_with('-'));
        args.next(); // Path of executable not needed.

        let arg = args.next();
        if arg.is_none() {
            return cli;
        }

        let arg = arg.unwrap();
        match arg.as_str() {
            "print" => return cli,
            "clear" => {
                cli.command = arg;
                return cli;
            }
            "add" | "do" | "undo" | "move" | "swap" | "edit" | "append" | "delete" => {
                cli.command = arg;
            }
            _ => {
                eprintln!("Unknown command `{arg}`");
                process::exit(1)
            }
        }

        let arg = args.next().unwrap_or_else(|| {
            eprintln!("Missing arguments for command `{}`", cli.command);
            process::exit(1)
        });

        if cli.command == "add" {
            cli.contents = arg;
            return cli;
        }
        cli.task_ids = arg;
        match cli.command.as_str() {
            "do" | "undo" | "delete" => return cli,
            _ => (),
        }

        let arg = args.next().unwrap_or_else(|| {
            eprintln!("Missing additional arguments for command `{}`", cli.command);
            process::exit(1)
        });

        match cli.command.as_str() {
            "move" | "swap" => cli.move_id = arg,
            "edit" | "append" => cli.contents = arg,
            _ => (),
        }
        cli
    }

    /// Takes a list of ids and returns it dedupped and sorted.
    pub fn parse_id_list(ids: &str) -> Result<String, Box<dyn Error>> {
        let mut v: Vec<String> = ids.split(',').map(|i| i.to_string()).collect();
        if !ids.contains('.') {
            let err = v.iter().find_map(|s| s.parse::<usize>().err());
            if let Some(e) = err {
                return Err(Box::new(e));
            }

            v.sort_by(|a, b| {
                a.parse::<usize>()
                    .unwrap()
                    .cmp(&b.parse::<usize>().unwrap())
            });

            v.dedup();
            return Ok(v.join(","));
        }
        let err = v.iter().find_map(|s| s.parse::<f32>().err());
        if let Some(e) = err {
            return Err(Box::new(e));
        }

        v.sort_by(|a, b| {
            let a = a.parse::<f32>().unwrap();
            let b = b.parse::<f32>().unwrap();

            if a >= b {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        });

        v.dedup();
        Ok(v.join(","))
    }

    /// Takes a range pattern and returns a list of ids.
    pub fn parse_id_range(ids: &str) -> Result<String, Box<dyn Error>> {
        let v: Vec<String> = ids.split("..").map(|i| i.to_string()).collect();
        let start: usize = if v[0].contains('.') {
            let v: Vec<&str> = v[0].split('.').collect();
            v[0].parse()?
        } else {
            v[0].parse()?
        };

        let end: usize = if v[1].contains('.') {
            let v: Vec<&str> = v[1].split('.').collect();
            v[0].parse()?
        } else {
            v[1].parse()?
        };

        let mut out: Vec<String> = (start..=end).map(|i| i.to_string()).collect();
        let l = out.len() - 1;

        out[0] = v[0].clone();
        out[l] = v[1].clone();
        return Ok(out.join(","));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_list_normal_ids() {
        let res = Cli::parse_id_list("1,2,3,4,4,3,12,5");
        let res = res.unwrap_or_default();
        assert_eq!(res, "1,2,3,4,5,12".to_string());
    }

    #[test]
    fn test_parse_list_sub_ids() {
        let res = Cli::parse_id_list("3.3,2.3,1.2,3.3");
        let res = res.unwrap_or_default();
        assert_eq!(res, "1.2,2.3,3.3".to_string());
    }

    #[test]
    fn test_parse_list_mixed_ids() {
        let res = Cli::parse_id_list("1.2,2.3,3.3,4,4,3.3,12,5");
        let res = res.unwrap_or_default();
        assert_eq!(res, "1.2,2.3,3.3,4,5,12".to_string());
    }

    #[test]
    fn test_parse_range_normal_ids() {
        let res = Cli::parse_id_range("1..5");
        let res = res.unwrap_or_default();
        assert_eq!(res, "1,2,3,4,5".to_string());
    }

    #[test]
    fn test_parse_range_sub_ids() {
        let res = Cli::parse_id_range("1.2..5.9");
        let res = res.unwrap_or_default();
        assert_eq!(res, "1.2,2,3,4,5.9".to_string());
    }

    #[test]
    fn test_parse_range_mixed_ids() {
        let res = Cli::parse_id_range("1..5.9");
        let res = res.unwrap_or_default();
        assert_eq!(res, "1,2,3,4,5.9".to_string());
    }
}
