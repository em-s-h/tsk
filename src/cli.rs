use std::{
    env::{self, Args},
    fs::File,
    io::{BufRead, BufReader},
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

    /// Id (line number) of the task or tasks to be operated
    pub task_ids: Vec<usize>,

    /// Used when moving tasks
    pub new_id: usize,

    pub print_help: bool,

    pub print_version: bool,

    pub colored_output: bool,

    /// Print the contents of the task file
    pub print: bool,

    /// Mark a task as done
    pub mark_done: bool,

    /// Unmark a task as done
    pub unmark_done: bool,

    /// Delete all tasks that are marked as done
    pub clear_dones: bool,

    /// Add an task to the task file
    pub add: bool,

    /// Append to an task of the task file
    pub append: bool,

    /// Rewrite an task of the task file
    pub edit: bool,

    /// Move an task to another place
    pub move_task: bool,

    /// Delete an task from the task file
    pub delete: bool,
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

            print_help: false,
            print_version: false,
            colored_output: true,

            print: false,
            mark_done: false,
            unmark_done: false,
            clear_dones: false,
            add: false,
            append: false,
            edit: false,
            move_task: false,
            delete: false,
        }
    }
    // }}}

    pub fn parse_args(mut self) -> Self {
        // {{{
        fn get_next_arg(args: &mut Args) -> String {
            // {{{
            match args.next() {
                Some(a) => a,
                _ => String::new(),
            }
        }
        // }}}

        let mut args = env::args();
        args.next(); // First argument is unneeded

        let mut arg = get_next_arg(&mut args);
        if arg == "--help" || arg == "-h" {
            self.print_help = true;
            return self;
        } else if arg == "--version" || arg == "-v" {
            self.print_version = true;
            return self;
        } else if arg == "--no-color" {
            self.colored_output = false;
            arg = get_next_arg(&mut args);
        }

        if arg == "print" || arg.is_empty() {
            self.print = true;
            return self;
        } else if arg == "clear" {
            self.clear_dones = true;
            return self;
        }

        // Parse task operation related arguments {{{
        /// Makes sure the id is not above the amount of lines in the task file
        fn check_ids(ids: &[usize]) {
            // {{{
            let path = crate::get_task_file();
            let file = File::open(&path).expect("Unable to open task file for reading");
            let line_count = BufReader::new(&file).lines().count();

            for id in ids {
                if id.to_owned() > line_count {
                    eprintln!("The id:{id} is above the last id");
                    process::exit(1);
                }
            }
        }
        // }}}

        fn get_task_ids(args: &mut Args) -> Vec<usize> {
            // {{{
            let mut ids: Vec<usize> = get_next_arg(args)
                .split(',')
                .map(|id| {
                    id.trim().parse().unwrap_or_else(|_| {
                        eprintln!("Please make sure all ids are valid");
                        process::exit(1)
                    })
                })
                .collect();

            ids.sort();
            ids.dedup();
            ids
        }
        // }}}

        fn get_task_content(args: &mut Args) -> String {
            // {{{
            let task = get_next_arg(args);

            if task.is_empty() {
                eprintln!("Please provide the content of the task");
                process::exit(1);
            }
            task
        }
        // }}}

        match arg.as_str() {
            "do" => self.mark_done = true,
            "undo" => self.unmark_done = true,
            "add" => self.add = true,
            "append" => self.append = true,
            "edit" => self.edit = true,
            "move" => self.move_task = true,
            "delete" => self.delete = true,
            _ => {
                eprintln!("{arg} is not a valid argument");
                process::exit(1)
            }
        }

        let requires_id = self.mark_done // {{{
            || self.unmark_done
            || self.append
            || self.edit
            || self.move_task
            || self.delete;
        // }}}

        if requires_id {
            self.task_ids = get_task_ids(&mut args);

            check_ids(&self.task_ids);
            // -1 because lines are counted from 0
            self.task_ids = self.task_ids.iter().map(|id| id - 1).collect();
        }

        if self.add || self.append || self.edit {
            self.task = get_task_content(&mut args);
        }

        if self.move_task {
            self.new_id = get_task_ids(&mut args)[0];

            check_ids(&[self.new_id]);
            self.new_id -= 1;

            if self.new_id == self.task_ids[0] {
                eprintln!("Please provide different task ids");
                process::exit(1);
            }
        }
        // }}}

        self
    }
    // }}}

    pub fn print_help() {
        // {{{
        println!(
            "{NAME}: {DESCRIPTION}
Made by: {AUTHOR}

Usage: {NAME} [Options] [Command] [Args]

Options:
    --help      -h
        Print this message
    --version   -v
        Print the program version
    --no-color
        Don't make the output colored

Commands:
    print
        Print all tasks
        Default when not passing any args
    clear   
        Delete all tasks that are marked as done
    do      <task_ids>
        Mark 1 or more tasks as done
    undo    <task_ids>
        Unmark 1 or more tasks as done
    add     <task>
        Add a new task
    append  <task_id> <content>
        Append content to an existing task
    edit    <task_id> <new_task>
        Replace the contents of a task
    move    <task_id> <new_task_id>
        Move a task to a new location
    delete  <task_ids>
        Delete 1 or more tasks"
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
