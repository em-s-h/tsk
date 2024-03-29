use std::{
    env::{self},
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, PartialEq)]
/// Indicates the position to add the new task
pub enum AddOpt {
    // {{{
    Top,
    Bottom,
}
// }}}

#[derive(Debug)]
pub struct Cli {
    // {{{
    /// The task that will be added to the task file
    pub task: String,

    /// Id (line number) of the task or tasks to be operated
    pub task_ids: Vec<usize>,

    /// Used when moving tasks
    pub new_id: usize,

    pub colored_output: bool,
    pub print: bool,

    pub add_task: bool,
    pub add_to: AddOpt,

    pub mark_done: bool,
    pub unmark_done: bool,

    pub move_task: bool,
    pub swap_tasks: bool,

    pub append_task: bool,
    pub edit_task: bool,

    pub delete_task: bool,
    pub clear_dones: bool,
}
// }}}

impl Cli {
    // {{{
    pub fn new() -> Self {
        // {{{
        Self {
            task: String::new(),
            task_ids: vec![0],
            new_id: 0,

            colored_output: true,
            print: false,
            mark_done: false,
            unmark_done: false,
            add_task: false,
            add_to: AddOpt::Top,
            append_task: false,
            edit_task: false,
            move_task: false,
            swap_tasks: false,
            delete_task: false,
            clear_dones: false,
        }
    }
    // }}}

    pub fn parse_args(mut self) -> Self {
        // {{{
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

        let mut args = env::args().peekable();
        args.next(); // First argument is unneeded

        let opt = get_next(&mut args);

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
        // }}}

        // In case the user passes the '--no-color' option
        let arg = if opt.starts_with('-') {
            get_next(&mut args)
        } else {
            opt
        };

        if arg == "print" || arg.is_empty() {
            self.print = true;
            return self;
        } else if arg == "clear" {
            self.clear_dones = true;
            return self;
        }

        fn get_line_count() -> usize {
            // {{{
            let path = crate::get_task_file();
            let file = File::open(&path).expect("File has been verified to be readable");
            BufReader::new(&file).lines().count()
        }
        // }}}

        fn get_task_ids<T>(args: &mut T) -> Vec<usize>
        // {{{
        where
            T: Iterator<Item = String> + Debug,
        {
            let arg = get_next(args);
            let get_vec = |pat: &str| {
                // {{{
                arg.split(pat)
                    .map(|id| {
                        id.trim().parse().unwrap_or_else(|_| {
                            eprintln!("Invalid id: '{id}'");
                            process::exit(1)
                        })
                    })
                    .collect()
            };
            // }}}

            if arg == "-all" {
                let ln_count = get_line_count();
                return (1..=ln_count).collect();
            } else if arg.starts_with('-') {
                eprintln!("Invalid operation option: '{arg}'");
                process::exit(1);
            }

            let ids: Vec<usize> = if arg.contains("..") {
                // {{{
                let v: Vec<usize> = get_vec("..");
                (v[0]..=v[1]).collect()
            } else if arg.contains(',') {
                let mut v: Vec<usize> = get_vec(",");
                v.sort();
                v.dedup();
                v
            } else {
                vec![arg.trim().parse().unwrap_or_else(|_| {
                    eprintln!("Invalid id: {arg}");
                    process::exit(1)
                })]
            };
            // }}}

            let line_count = {
                let path = crate::get_task_file();
                let file = File::open(&path).expect("File has been verified to be readable");
                BufReader::new(&file).lines().count()
            };
            let invalid_id = ids.iter().find(|&id| *id > line_count);

            if let Some(id) = invalid_id {
                eprintln!("Invalid id: {id}");
                eprintln!("Please make sure ids are bellow {line_count}");
                process::exit(1)
            }
            ids
        }
        // }}}

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
            self.task_ids = get_task_ids(&mut args);

            // -1 because lines are counted from 0
            self.task_ids = self.task_ids.iter().map(|id| id - 1).collect();
        }

        let is_opt = {
            let def = String::new();
            let next = args.peek().unwrap_or(&def);
            next.starts_with('-')
        };

        if self.add_task && is_opt {
            // 'add' options {{{
            let opt = get_next(&mut args);

            if opt == "-bot" {
                self.add_to = AddOpt::Bottom;
            } else if opt != "-top" {
                eprintln!("Invalid operation option: '{opt}'");
                eprintln!("Valid options are '-bot' and '-top'");
                process::exit(1)
            }
        }
        // }}}

        if self.add_task || self.append_task || self.edit_task {
            let task = get_next(&mut args);
            if task.is_empty() {
                eprintln!("Please provide the content of the task");
                process::exit(1);
            }
            self.task = task.replace("[ ]", "").replace("[X]", "").trim().to_owned()
        }

        if self.move_task || self.swap_tasks {
            self.new_id = get_task_ids(&mut args)[0];
            self.new_id -= 1;

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
        Print tasks
        Default when not passing any args
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
        Used by 'add' to add tasks to the top of the task list
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
