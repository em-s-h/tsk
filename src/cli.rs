use std::{
    cmp::Ordering,
    env::{self},
    error::Error,
    fmt::Debug,
    io::{self, Write},
    process,
};

use crate::task_file::TaskFile;

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
    pub fn new() -> Self {
        Self {
            colored_output: true,
            command: "print".to_owned(),
            add_to: "top".to_owned(),
            task_ids: String::new(),
            move_id: String::new(),
            contents: String::new(),
        }
    }

    pub fn print_help() {
        // {{{
        println!(
            "{NAME} {VERSION}: {DESCRIPTION}
        Made by: {AUTHOR}

        Usage: {NAME} [Options] [Command] [Sub-Options] [Args]

        Options:
            --generate-shell-completions <prompt> <current_word>,<position>
                Generate shell completions
            --help      -h
                Print this message
            --version   -v
                Print the program version
            --no-color  -c
                Don't make the output colored
            --all       -a
                Shortcut for selecting all tasks.
                Not used by commands that use only a single id
            --add-to    -t=<position> 
                Used by `add`.
                Values: top, bot[tom]
            --subtask   -s=<parent_id>
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
                Delete tasks
            clear   
                Delete all tasks that are marked as done
        "
        );
    }

    /// Generate shell completions.
    /// `args` should be formatted this way: "$prompt $current_word,$position"
    /// Ex.: "tsk ad -t = top 'hi' ad,1"
    pub fn generate_shell_completions(args: Vec<String>) -> Result<String, &'static str> {
        if args.is_empty() {
            return Ok("".to_string());
        }

        let no_opts: Vec<String> = {
            let mut a: Vec<String> = args
                .iter()
                .filter(|a| !a.starts_with('-'))
                .map(|a| a.to_string())
                .collect();

            for _ in 0..a.iter().filter(|_a| *_a == "=").count() {
                let id = a.iter().position(|_a| _a == "=").unwrap_or_default();
                if id == 0 {
                    continue;
                }
                a.remove(id + 1);
                a.remove(id);
            }
            a
        };

        let (current_word, position) = {
            let l = args.last().unwrap();
            if !l.contains(',') {
                return Err("Missing `$current_word,$position`");
            }
            let a: Vec<&str> = l.split(',').collect();
            (a[0], a[1].parse::<usize>().unwrap_or_default())
        };

        // `position` will always be >=1 since that's when app specific completions start.
        if position == 0 {
            return Err("Current word position is not usize");
        }

        // Find the position of the '=' that the user is at.
        let near_eq = current_word == "=" || args[position - 1] == "=";
        if near_eq {
            let opt = if current_word == "=" {
                &args[position - 1]
            } else if args[position - 1] == "=" {
                &args[position - 2]
            } else {
                ""
            };

            if near_eq && (opt == "-t" || opt == "--add-to") {
                return Ok("top bottom".to_string());
            }
        }

        if current_word.starts_with("--") {
            return Ok(
                "--help --version --no-color --all --add-to --subtask --generate-shell-completions"
                    .to_string(),
            );
        } else if current_word.starts_with('-') {
            return Ok("-h -v -c -a -t -s".to_string());
        } else if position == 1 || no_opts.get(1).is_some_and(|a| a == current_word) {
            return Ok("print add do undo move swap append edit delete clear".to_string());
        }

        if no_opts.get(1).is_some_and(|a| a == "edit")
            && no_opts.get(3).is_some_and(|a| a.is_empty())
        {
            let id = no_opts.get(2).unwrap_or_else(|| process::exit(1));
            let id = id.parse::<usize>().unwrap_or_else(|_| process::exit(1));

            let tf = TaskFile::load();
            if let Some(cont) = tf.get_task_contents(&id.to_string()) {
                return Ok(format!("'{cont}'"));
            }
            return Ok("".to_string());
        }

        Ok("".to_string())
    }

    /// Parse cmd line arguments.
    /// When debugging the vector's 1st value should be an empty string, since that's
    /// where the program's path would be normally, and that value is skipped over.
    pub fn parse_args(dbg_args: Option<Vec<String>>) -> Result<Self, String> {
        let mut cli = Self::new();
        let options = if let Some(a) = dbg_args.clone() {
            let a: Vec<String> = a.into_iter().filter(|a| a.starts_with('-')).collect();
            a.into_iter()
        } else {
            let a: Vec<String> = env::args().filter(|a| a.starts_with('-')).collect();
            a.into_iter()
        };

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
                "-c" | "--no-color" => cli.colored_output = false,
                "-a" | "--all" => cli.task_ids = "all".to_string(),

                "--generate-shell-completions" => {
                    let args: Vec<String> = env::args()
                        .skip_while(|i| i != "--generate-shell-completions")
                        .skip(1)
                        .collect();

                    let res = Self::generate_shell_completions(args);
                    if let Ok(out) = res {
                        print!("{out}");
                        io::stdout().flush().unwrap();
                        process::exit(0)
                    } else if let Err(e) = res {
                        eprintln!("{e}");
                        process::exit(1)
                    }
                }

                _ if !o.contains('=') => return Err(format!("Unknown option `{o}`")),
                _ => (),
            }
            if !o.contains('=') {
                continue;
            }
            let v: Vec<&str> = o.split('=').collect();
            match v[0] {
                "-t" | "--add-to" => {
                    if v[1] != "top" && v[1] != "bot" && v[1] != "bottom" {
                        return Err(format!("Invalid option value `{}`", v[1]));
                    }
                    cli.add_to = v[1].to_string();
                }
                "-s" | "--subtask" => {
                    if v[1].is_empty() {
                        return Err(format!("Please provide an id"));
                    }
                    cli.task_ids = v[1].to_string();
                }
                _ => {
                    return Err(format!("Unknown option `{}`", v[0]));
                }
            }
        }

        let mut args = if let Some(a) = dbg_args {
            let a: Vec<String> = a.into_iter().filter(|a| !a.starts_with('-')).collect();
            a.into_iter()
        } else {
            let a: Vec<String> = env::args().filter(|a| !a.starts_with('-')).collect();
            a.into_iter()
        };
        args.next(); // Path of executable not needed.

        let arg = args.next();
        if arg.is_none() {
            return Ok(cli);
        }

        let arg = arg.unwrap();
        match arg.as_str() {
            "print" => return Ok(cli),
            "clear" => {
                cli.command = arg;
                return Ok(cli);
            }
            "add" | "do" | "undo" | "move" | "swap" | "edit" | "append" | "delete" => {
                cli.command = arg;
            }
            _ => {
                return Err(format!("Unknown command `{arg}`"));
            }
        }

        let arg = args.next().unwrap_or_default();
        if arg.is_empty() && cli.task_ids != "all" {
            return Err(format!("Missing arguments for `{}`", cli.command));
        }

        if cli.command == "add" {
            cli.contents = arg;
            return Ok(cli);
        }
        if cli.task_ids != "all" {
            cli.task_ids = arg;
        }
        match cli.command.as_str() {
            "do" | "undo" => return Ok(cli),

            "delete" if cli.task_ids == "all" => {
                return Err(format!("Flag `--all` not allowed for single task commands"))
            }
            "delete" => return Ok(cli),
            _ => (),
        }

        let arg = args.next().unwrap_or_default();
        if cli.task_ids == "all" {
            return Err(format!("Flag `--all` not allowed for single task commands"));
        } else if arg.is_empty() {
            return Err(format!("Missing second argument for `{}`", cli.command));
        }

        match cli.command.as_str() {
            "move" | "swap" if cli.task_ids == arg => {
                return Err(format!("Please provide different ids"));
            }

            "move" | "swap" => cli.move_id = arg,
            "edit" | "append" => cli.contents = arg,
            _ => (),
        }
        Ok(cli)
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

    // // SHELL COMPLETIONS
    /// `args` ex.: "tsk ad -t = top 'hello world' ad,1"
    fn get_comp(args: &str) -> Result<String, &'static str> {
        let a: Vec<String> = if args.is_empty() {
            vec![]
        } else {
            args.split(' ').map(|s| s.to_string()).collect()
        };
        Cli::generate_shell_completions(a)
    }

    #[test]
    fn test_cmd_completion() {
        let comp = get_comp("tsk    ,1");
        assert!(comp.is_ok());
        assert_eq!(
            comp.unwrap(),
            "print add do undo move swap append edit delete clear"
        );

        let comp = get_comp("tsk ad ad,1");
        assert!(comp.is_ok());
        assert_eq!(
            comp.unwrap(),
            "print add do undo move swap append edit delete clear"
        );

        let comp = get_comp("tsk ze ze,1");
        assert!(comp.is_ok());
        assert_eq!(
            comp.unwrap(),
            "print add do undo move swap append edit delete clear"
        );
    }

    #[test]
    fn test_cmd_completion_after_options() {
        let comp = get_comp("tsk -a    ,2");
        assert!(comp.is_ok());
        assert_eq!(
            comp.unwrap(),
            "print add do undo move swap append edit delete clear"
        );

        let comp = get_comp("tsk -a sw sw,2");
        assert!(comp.is_ok());
        assert_eq!(
            comp.unwrap(),
            "print add do undo move swap append edit delete clear"
        );

        let comp = get_comp("tsk -a -s = 2    ,5");
        assert!(comp.is_ok());
        assert_eq!(
            comp.unwrap(),
            "print add do undo move swap append edit delete clear"
        );

        let comp = get_comp("tsk -a -s = 2 pr pr,5");
        assert!(comp.is_ok());
        assert_eq!(
            comp.unwrap(),
            "print add do undo move swap append edit delete clear"
        );
    }

    #[test]
    fn test_short_option_completion() {
        let comp = get_comp("tsk - -,1");
        assert!(comp.is_ok());
        assert_eq!(comp.unwrap(), "-h -v -c -a -t -s");

        let comp = get_comp("tsk -c -c,1");
        assert!(comp.is_ok());
        assert_eq!(comp.unwrap(), "-h -v -c -a -t -s");

        let comp = get_comp("tsk -z -z,1");
        assert!(comp.is_ok());
        assert_eq!(comp.unwrap(), "-h -v -c -a -t -s");
    }

    #[test]
    fn test_long_option_completion() {
        let comp = get_comp("tsk -- --,1");
        assert!(comp.is_ok());
        assert_eq!(
            comp.unwrap(),
            "--help --version --no-color --all --add-to --subtask --generate-shell-completions"
        );

        let comp = get_comp("tsk --h --h,1");
        assert!(comp.is_ok());
        assert_eq!(
            comp.unwrap(),
            "--help --version --no-color --all --add-to --subtask --generate-shell-completions"
        );

        let comp = get_comp("tsk --z --z,1");
        assert!(comp.is_ok());
        assert_eq!(
            comp.unwrap(),
            "--help --version --no-color --all --add-to --subtask --generate-shell-completions"
        );
    }

    #[test]
    fn test_add_to_option_completion() {
        let comp = get_comp("tsk -t =    ,3");
        assert!(comp.is_ok());
        assert_eq!(comp.unwrap(), "top bottom");

        let comp = get_comp("tsk --add-to =    ,3");
        assert!(comp.is_ok());
        assert_eq!(comp.unwrap(), "top bottom");

        let comp = get_comp("tsk -t = =,2");
        assert!(comp.is_ok());
        assert_eq!(comp.unwrap(), "top bottom");

        let comp = get_comp("tsk --add-to = =,2");
        assert!(comp.is_ok());
        assert_eq!(comp.unwrap(), "top bottom");
    }

    #[test]
    fn test_edit_completion() {
        let comp = get_comp("tsk edit 2    ,3");
        let tf = TaskFile::load();
        assert!(comp.is_ok());

        let cont = if let Some(c) = tf.get_task_contents("2") {
            format!("'{c}'")
        } else {
            "".to_string()
        };
        assert_eq!(comp.unwrap(), cont);
    }

    #[test]
    fn test_incorrect_completion_args() {
        let comp = get_comp("tsk edit 2    3");
        assert!(comp.is_err());

        let comp = get_comp("tsk edit 2    ,-1");
        assert!(comp.is_err());

        let comp = get_comp("tsk edit 2    ,a");
        assert!(comp.is_err());
    }

    #[test]
    fn test_empty_completion() {
        let comp = get_comp("");
        assert!(comp.is_ok());
        assert_eq!(comp.unwrap(), "");

        let comp = get_comp("tsk add -t = 2 'hello'    ,5");
        assert!(comp.is_ok());
        assert_eq!(comp.unwrap(), "");
    }

    // // ARG PARSING
    /// `args` ex.: "tsk add -t=top 'hello world' "
    fn get_cli(args: &str) -> Result<Cli, String> {
        let args = args.trim();
        let a = args.split(' ').map(|s| s.to_owned()).collect();
        Cli::parse_args(Some(a))
    }

    // OPTIONS
    #[test]
    fn test_flag_nocolor() {
        let cli = get_cli("tsk -c");
        assert!(cli.is_ok());
        assert_eq!(cli.unwrap().colored_output, false);
    }

    #[test]
    fn test_flag_all() {
        let cli = get_cli("tsk do -a");
        assert!(cli.is_ok());
        assert_eq!(cli.unwrap().task_ids, "all");
    }

    #[test]
    fn test_flag_all_fail_on_single_id_cmd() {
        let cli = get_cli("tsk edit -a");
        assert!(cli.is_err());
        assert_eq!(
            cli.err().unwrap(),
            "Flag `--all` not allowed for single task commands"
        );

        let cli = get_cli("tsk delete -a");
        assert!(cli.is_err());
    }

    #[test]
    fn test_option_addto() {
        let cli = get_cli("tsk -t=top");
        assert!(cli.is_ok());
        assert_eq!(cli.unwrap().add_to, "top");

        let cli = get_cli("tsk -t=bot");
        assert!(cli.is_ok());
        assert_eq!(cli.unwrap().add_to, "bot");

        let cli = get_cli("tsk -t=bottom");
        assert!(cli.is_ok());
        assert_eq!(cli.unwrap().add_to, "bottom");
    }

    #[test]
    fn test_option_subtask() {
        let cli = get_cli("tsk -s=1.2");
        assert!(cli.is_ok());
        assert_eq!(cli.unwrap().task_ids, "1.2")
    }

    #[test]
    fn test_option_addto_wrong_value() {
        let cli = get_cli("tsk -t=tophat");
        assert!(cli.is_err());

        let cli = get_cli("tsk -t=");
        assert!(cli.is_err());
    }

    #[test]
    fn test_option_subtask_empty_value() {
        let cli = get_cli("tsk -s=");
        assert!(cli.is_err());
    }

    #[test]
    fn test_unknow_option_err() {
        let cli = get_cli("tsk -l=all");
        assert!(cli.is_err());

        let cli = get_cli("tsk --loll");
        assert!(cli.is_err());
    }

    // COMMANDS
    #[test]
    fn test_cmds_without_ids() {
        let cli = get_cli("tsk print");
        assert!(cli.is_ok());
        assert_eq!(cli.unwrap().command, "print");

        let cli = get_cli("tsk");
        assert!(cli.is_ok());
        assert_eq!(cli.unwrap().command, "print");

        let cli = get_cli("tsk clear");
        assert!(cli.is_ok());
        assert_eq!(cli.unwrap().command, "clear");
    }

    #[test]
    fn test_add_cmd() {
        let cli = get_cli("tsk add -s=2 test");
        assert!(cli.is_ok());
        let cli = cli.unwrap();

        assert_eq!(cli.command, "add");
        assert_eq!(cli.contents, "test");
        assert_eq!(cli.task_ids, "2");

        let cli = get_cli("tsk add test");
        assert!(cli.is_ok());
        let cli = cli.unwrap();

        assert_eq!(cli.command, "add");
        assert_eq!(cli.contents, "test");
        assert_eq!(cli.task_ids, String::new());
    }

    #[test]
    fn test_mark_cmds() {
        let cli = get_cli("tsk do 2");
        assert!(cli.is_ok());
        let cli = cli.unwrap();

        assert_eq!(cli.command, "do");
        assert_eq!(cli.task_ids, "2");

        let cli = get_cli("tsk undo 2");
        assert!(cli.is_ok());
        let cli = cli.unwrap();

        assert_eq!(cli.command, "undo");
        assert_eq!(cli.task_ids, "2");

        let cli = get_cli("tsk do ");
        assert!(cli.is_err());

        let cli = get_cli("tsk undo ");
        assert!(cli.is_err());

        let cli = get_cli("tsk do 2,3..2.45.1,2,35,10a");
        assert!(cli.is_ok());
        let cli = cli.unwrap();

        assert_eq!(cli.command, "do");
        assert_eq!(cli.task_ids, "2,3..2.45.1,2,35,10a");

        let cli = get_cli("tsk undo 2,3..2.45.1,2,35,10a");
        assert!(cli.is_ok());
        let cli = cli.unwrap();

        assert_eq!(cli.command, "undo");
        assert_eq!(cli.task_ids, "2,3..2.45.1,2,35,10a");
    }

    #[test]
    fn test_move_cmds() {
        let cli = get_cli("tsk move 2 3");
        assert!(cli.is_ok());
        let cli = cli.unwrap();

        assert_eq!(cli.command, "move");
        assert_eq!(cli.task_ids, "2");
        assert_eq!(cli.move_id, "3");

        let cli = get_cli("tsk swap 2 4");
        assert!(cli.is_ok());
        let cli = cli.unwrap();

        assert_eq!(cli.command, "swap");
        assert_eq!(cli.task_ids, "2");
        assert_eq!(cli.move_id, "4");

        let cli = get_cli("tsk move a b");
        assert!(cli.is_ok());
        let cli = cli.unwrap();

        assert_eq!(cli.command, "move");
        assert_eq!(cli.task_ids, "a");
        assert_eq!(cli.move_id, "b");

        let cli = get_cli("tsk swap a b");
        assert!(cli.is_ok());
        let cli = cli.unwrap();

        assert_eq!(cli.command, "swap");
        assert_eq!(cli.task_ids, "a");
        assert_eq!(cli.move_id, "b");

        let cli = get_cli("tsk move ");
        assert!(cli.is_err());

        let cli = get_cli("tsk swap ");
        assert!(cli.is_err());

        let cli = get_cli("tsk move 2 ");
        assert!(cli.is_err());

        let cli = get_cli("tsk swap 3 ");
        assert!(cli.is_err());

        let cli = get_cli("tsk move 2 2");
        assert!(cli.is_err());

        let cli = get_cli("tsk swap 3 3");
        assert!(cli.is_err());
    }

    #[test]
    fn test_edit_cmds() {
        let cli = get_cli("tsk edit 2 test");
        assert!(cli.is_ok());
        let cli = cli.unwrap();

        assert_eq!(cli.command, "edit");
        assert_eq!(cli.task_ids, "2");
        assert_eq!(cli.contents, "test");

        let cli = get_cli("tsk append 2 test");
        assert!(cli.is_ok());
        let cli = cli.unwrap();

        assert_eq!(cli.command, "append");
        assert_eq!(cli.task_ids, "2");
        assert_eq!(cli.contents, "test");

        let cli = get_cli("tsk edit a test");
        assert!(cli.is_ok());
        let cli = cli.unwrap();

        assert_eq!(cli.command, "edit");
        assert_eq!(cli.task_ids, "a");
        assert_eq!(cli.contents, "test");

        let cli = get_cli("tsk append a test");
        assert!(cli.is_ok());
        let cli = cli.unwrap();

        assert_eq!(cli.command, "append");
        assert_eq!(cli.task_ids, "a");
        assert_eq!(cli.contents, "test");

        let cli = get_cli("tsk edit ");
        assert!(cli.is_err());

        let cli = get_cli("tsk append ");
        assert!(cli.is_err());

        let cli = get_cli("tsk edit 2 ");
        assert!(cli.is_err());

        let cli = get_cli("tsk append 3 ");
        assert!(cli.is_err());
    }

    #[test]
    fn test_unknow_command() {
        let cli = get_cli("tsk bob 50");
        assert!(cli.is_err())
    }

    // // ID PARSING
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
