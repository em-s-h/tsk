use crate::task_file::{AddPosition, Task, TaskFile};
use std::{
    env::{self, Args},
    fmt::Debug,
    iter::Peekable,
    process,
};

const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug)]
pub struct Cli {
    // {{{
    /// The task that will be added to the task file
    pub task: String,

    /// Id of the task/subtask or tasks to be operated
    pub task_ids: Vec<String>,

    /// Used when moving tasks
    pub new_id: String,

    pub colored_output: bool,
    pub print: bool,

    pub add_task: bool,
    pub add_to: AddPosition,

    pub mark_done: bool,
    pub unmark_done: bool,

    pub move_task: bool,
    pub swap_tasks: bool,

    pub append_task: bool,
    pub edit_task: bool,

    pub delete_task: bool,
    pub clear_dones: bool,
    pub enable_debug: bool,
}
// }}}

impl Cli {
    // {{{
    pub fn new() -> Self {
        // {{{
        Self {
            task: String::new(),
            task_ids: vec![String::new()],
            new_id: String::new(),

            colored_output: true,
            print: false,
            mark_done: false,
            unmark_done: false,
            add_task: false,
            add_to: AddPosition::Top,
            append_task: false,
            edit_task: false,
            move_task: false,
            swap_tasks: false,
            delete_task: false,
            clear_dones: false,
            enable_debug: false,
        }
    }
    // }}}

    fn get_task_count() -> usize {
        // {{{
        let path = crate::get_task_file_path();
        let tf = TaskFile::parse_file(&path);
        tf.tasks.len()
    }
    // }}}

    fn get_subtask_count(parent_id: &str) -> usize {
        // {{{
        let v: Vec<usize> = parent_id.split('.').map(|i| i.parse().unwrap()).collect();
        let path = crate::get_task_file_path();
        let tf = TaskFile::parse_file(&path);

        fn _get(tasks: &[Task], p_id: &[usize], depth: usize) -> Option<usize> {
            // p_id = 1.2
            for (id, t) in tasks.iter().enumerate() {
                if id + 1 == p_id[depth] && p_id.len() == depth + 1 {
                    // 1.2 <- We are here
                    return Some(t.subtasks.len());
                } else if id + 1 == p_id[depth] {
                    // 1 <- We are here
                    return _get(&t.subtasks, p_id, depth + 1);
                }
            }
            None
        }
        if let Some(r) = _get(&tf.tasks, &v, 0) {
            r
        } else {
            eprintln!("Unable to find subtasks for the id: {}", parent_id);
            process::exit(1)
        }
    }
    // }}}

    fn get_task_ids<T>(args: &mut T) -> Vec<String>
    // {{{
    where
        T: Iterator<Item = String> + Debug,
    {
        let arg = Self::get_next(args);
        let prep = |pat: &str| -> (Vec<String>, (String, String)) {
            // {{{
            // "1.2.3,5,..." -> "1.2.3", "5" "..."
            let v: Vec<String> = arg.split(pat).map(|s| s.to_owned()).collect();
            // "1.2.3" -> "1.2", "3"
            let t = {
                if let Some(t) = v[0].rsplit_once('.') {
                    t
                } else {
                    (v[0].as_str(), "0")
                }
            };

            let t = (t.0.to_owned(), t.1.to_owned());

            let f: f32 = t.0.parse().unwrap_or_else(|_| {
                eprintln!("Invalid subtask id: {arg}");
                process::exit(1)
            });
            if f < 1.0 {
                eprintln!("Invalid subtask id: {arg}");
                process::exit(1)
            }
            let _: usize = t.1.parse().unwrap_or_else(|_| {
                eprintln!("Invalid subtask id: '{arg}'");
                process::exit(1)
            });
            (v, t)
        };
        // }}}

        if arg == "-all" {
            return (1..=Self::get_task_count()).map(|i| i.to_string()).collect();
        } else if arg.is_empty() || arg == "0" {
            eprintln!("Invalid id: '{arg}'");
            process::exit(1);
        } else if arg.starts_with('-') {
            eprintln!("Invalid sub-option: '{arg}'");
            process::exit(1);
        }

        if !arg.contains(|c: char| c.is_ascii_digit() || c == '.' || c == ',') {
            eprintln!("Invalid id: '{arg}'");
            process::exit(1);
        }

        let ids: Vec<String> = if arg.contains("..") {
            // {{{
            // Range of ids {{{
            let (v, t) = prep("..");
            if v.len() != 2 || v[1].contains('.') {
                eprintln!("Invalid subtask id: '{arg}'");
                process::exit(1)
            }

            let from: usize = t.1.parse().unwrap();
            let to: usize = v[1].parse().unwrap_or_else(|_| {
                eprintln!("Invalid subtask id: '{arg}'");
                process::exit(1)
            });

            let last_id = if from == 0 {
                Self::get_task_count()
            } else {
                Self::get_subtask_count(&t.0)
            };

            if to > last_id {
                eprintln!("Invalid range: {to}, value above last id");
                process::exit(1)
            }
            let mut ret: Vec<String> = Vec::new();
            let from = if from == 0 {
                t.0.parse().unwrap()
            } else {
                ret.push(format!("{}.", t.0));
                from
            };

            for i in from..=to {
                ret.push(i.to_string())
            }
            ret
            // }}}
        } else if arg.contains(',') {
            // List of ids {{{
            let (v, t) = prep(",");
            let mut ret: Vec<String> = Vec::new();
            if t.1 == "0" {
                ret.push(t.0);
                for i in 1..v.len() {
                    if v[i].contains('.')
                        || v[i].parse::<usize>().is_err()
                        || v[i].parse::<usize>().unwrap() > Self::get_task_count()
                    {
                        eprintln!("Invalid task id: '{}'", v[1]);
                        process::exit(1)
                    }
                    ret.push(v[i].to_string())
                }
                ret.sort();
                ret.dedup();
                return ret;
            }
            ret.push(t.1);

            for i in 1..v.len() {
                if v[i].contains('.')
                    || v[i].parse::<usize>().is_err()
                    || v[i].parse::<usize>().unwrap() > Self::get_subtask_count(&t.0)
                {
                    eprintln!("Invalid subtask id: '{}.{}'", t.0, v[1]);
                    process::exit(1)
                }
                ret.push(v[i].to_string())
            }
            ret.sort();
            ret.insert(0, format!("{}.", t.0));
            ret.dedup();
            ret
            // }}}
        } else {
            // Single id {{{
            if let Some(_) = arg.split('.').find(|i| i.parse::<usize>().is_err()) {
                eprintln!("Invalid subtask id: '{arg}'");
                process::exit(1)
            }
            if arg.contains('.') {
                let (_, t) = prep(",");
                if t.1.parse::<usize>().unwrap() > Self::get_subtask_count(&t.0) {
                    eprintln!("Invalid subtask id: '{arg}'");
                    process::exit(1)
                }
            } else {
                if arg.parse::<usize>().unwrap() > Self::get_task_count() {
                    eprintln!("Invalid task id: '{arg}'");
                    process::exit(1)
                }
            }
            vec![arg]
            // }}}
        };
        // }}}
        ids
    }
    // }}}

    fn get_next<T>(args: &mut T) -> String
    // {{{
    where
        T: Iterator<Item = String>,
    {
        match args.next() {
            Some(a) => a.trim().to_owned(),
            _ => String::new(),
        }
    }
    // }}}

    pub fn parse_args(mut self) -> Self {
        // {{{
        let mut args = env::args().peekable();
        args.next(); // First argument is unneeded

        let opt = Self::get_next(&mut args);

        // Parse the first option passed to the program {{{
        if opt == "--help" || opt == "-h" {
            Cli::print_help();
            process::exit(0)
        } else if opt == "--version" || opt == "-v" {
            Cli::print_version();
            process::exit(0)
        } else if opt == "--no-color" {
            self.colored_output = false;
        } else if opt.starts_with('-') {
            eprintln!("Invalid option '{opt}'");
            process::exit(1)
        }

        // In case the user passes the '--no-color' option
        let arg = if opt.starts_with('-') {
            Self::get_next(&mut args)
        } else {
            opt
        };
        // }}}

        if arg == "print" || arg.is_empty() {
            self.print = true;
            return self;
        } else if arg == "clear" {
            self.clear_dones = true;
            return self;
        }

        match arg.as_str() {
            // Operations {{{
            "do" => self.mark_done = true,
            "undo" => self.unmark_done = true,
            "add" => self.add_task = true,
            "append" => self.append_task = true,
            "edit" => self.edit_task = true,
            "move" => self.move_task = true,
            "swap" => self.swap_tasks = true,
            "delete" => self.delete_task = true,
            _ => {
                eprintln!("'{arg}' is not a valid argument");
                process::exit(1)
            }
        }
        // }}}

        let requires_id = self.mark_done
        // {{{
            || self.unmark_done
            || self.append_task
            || self.edit_task
            || self.move_task
            || self.swap_tasks
            || self.delete_task;
        // }}}

        if requires_id {
            self.task_ids = Self::get_task_ids(&mut args)
        }

        let is_opt = |a: &mut Peekable<Args>| {
            let def = String::new();
            let next = a.peek().unwrap_or(&def);
            next.starts_with('-')
        };

        if self.add_task && is_opt(&mut args) {
            // 'add' options {{{
            let opt = Self::get_next(&mut args);
            let mut _opt = |opt: &str, args: &mut Peekable<Args>| {
                if opt == "-top" {
                    self.add_to = AddPosition::Top;
                } else if opt == "-bot" {
                    self.add_to = AddPosition::Bottom;
                } else if opt == "-sub" {
                    self.task_ids = Self::get_task_ids(args);
                } else {
                    eprintln!("Invalid sub-option: '{opt}'");
                    eprintln!("Valid options are: '-bot', '-top' and '-sub'");
                    process::exit(1)
                }
            };
            _opt(&opt, &mut args);

            if is_opt(&mut args) {
                let opt2 = Self::get_next(&mut args);
                if opt == opt2 {
                    eprintln!("Duplicate sub-options, processing last.")
                }
                _opt(&opt2, &mut args)
            }
        }
        // }}}

        if self.add_task || self.append_task || self.edit_task {
            self.task = Self::get_next(&mut args);
            if self.task.is_empty() {
                eprintln!("Please provide the content of the task");
                process::exit(1);
            }
        }

        if self.move_task || self.swap_tasks {
            let v = Self::get_task_ids(&mut args);

            // Get task ids of the 2 ids passed
            let p_id1: usize = v[0].split('.').next().unwrap().parse().unwrap();
            let p_id2: usize = self.task_ids[0].split('.').next().unwrap().parse().unwrap();

            // Check if we are operating with a task and a sub-task, or vice-versa.
            let task_and_sub = (v[0].contains('.') && !self.task_ids[0].contains('.'))
                || (!v[0].contains('.') && self.task_ids[0].contains('.'));

            if p_id1 == p_id2 && task_and_sub {
                eprintln!("Cannot operate on both a task and its subtasks");
                process::exit(1);
            }

            self.new_id = v[0].clone();

            if self.new_id == self.task_ids[0] {
                eprintln!("Please provide different task ids");
                process::exit(1);
            }
        }
        self
    }
    // }}}

    pub fn print_help() {
        // {{{
        println!(
            "{NAME}: {DESCRIPTION}
Made by: {AUTHOR}

Usage: {NAME} [Options] [Command] [Sub-Options] [Args]

Options:
    --help      -h
        Print this message
    --version   -v
        Print the program version
    --no-color
        Don't make the output colored

Commands:
    print
        Print tasks, default when not passing any args
    add     <task>
        Add a new task
    do      <task_ids>
        Mark 1 or more tasks as done
    undo    <task_ids>
        Unmark 1 or more tasks as done
    move    <task_id> <new_task_id>
        Move a task to a new location
    swap    <task_id> <other_task_id>
        Swap the places of two tasks
    append  <task_id> <content>
        Append content to an existing task
    edit    <task_id> <new_task>
        Replace the contents of a task
    delete  <task_ids>
        Delete 1 task
    clear   
        Delete all tasks that are marked as done

Sub-Options:
    -top
        Used by 'add' to add tasks to the top of the task list (default)
    -bot
        Used by 'add' to add tasks to the bottom of the task list
    -all
        Used by 'do' & 'undo' to un/mark all tasks as done"
        );
    }
    // }}}

    pub fn print_version() {
        // {{{
        println!("{NAME}: {DESCRIPTION}\n{VERSION}");
    }
    // }}}
}
// }}}

#[cfg(test)]
mod test {
    // {{{
}
// }}}
