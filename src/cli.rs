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
    /// The task that will be added to the list
    pub task: String,

    /// Id (line number) of the task
    pub task_id: usize,

    /// Used when moving tasks
    pub new_id: usize,

    pub print_help: bool,

    pub print_version: bool,

    pub colored_output: bool,

    /// Print the contents of the list
    pub print: bool,

    /// Mark a task as done
    pub mark_done: bool,

    /// Unmark a task as done
    pub unmark_done: bool,

    /// Remove all tasks that are marked as done
    pub clear_dones: bool,

    /// Add an task to the list
    pub add: bool,

    /// Append to an task of the list
    pub append: bool,

    /// Rewrite an task of the list
    pub edit: bool,

    /// Move an task to another place
    pub move_task: bool,

    /// Delete an task from the list
    pub delete: bool,
}
// }}}

impl Cli {
    // {{{
    pub fn new() -> Self {
        // {{{
        Self {
            task: String::new(),
            task_id: 0,
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

        let arg = get_next_arg(&mut args);
        if arg == "--help" || arg == "-h" {
            self.print_help = true;
            return self;
        } else if arg == "--version" || arg == "-v" {
            self.print_version = true;
            return self;
        } else if arg == "--no-color" {
            self.colored_output = false;
        }

        if arg == "print" || arg == "p" || arg.is_empty() {
            self.print = true;
            return self;
        }

        // Parse task operation related arguments {{{
        /// Makes sure the id is not above the amount of lines in a list
        fn check_id(id: usize) {
            // {{{
            let path = crate::get_list();
            let file = File::open(&path).expect("Unable to open list for reading");
            let line_count = BufReader::new(&file).lines().count();

            if id > line_count {
                eprintln!("The id is above the last id");
                process::exit(1);
            }
        }
        // }}}

        fn get_task_id(args: &mut Args) -> usize {
            // {{{
            match get_next_arg(args).parse() {
                Ok(id) => id,
                _ => {
                    eprintln!("Please provide a valid task id");
                    process::exit(1);
                }
            }
        }
        // }}}

        fn get_task_content(args: &mut Args) -> String {
            // {{{
            let task = get_next_arg(args);

            if task.is_empty() {
                eprintln!("Please provide the task's content");
                process::exit(1);
            }
            task
        }
        // }}}

        if arg == "do" {
            self.mark_done = true;
        } else if arg == "undo" || arg == "u" {
            self.unmark_done = true;
        } else if arg == "clear" || arg == "c" {
            self.clear_dones = true;
        } else if arg == "add" || arg == "a" {
            self.add = true;
        } else if arg == "append" || arg == "ap" {
            self.append = true;
        } else if arg == "edit" || arg == "e" {
            self.edit = true
        } else if arg == "move" || arg == "m" {
            self.move_task = true;
        } else if arg == "delete" || arg == "d" {
            self.delete = true;
        }

        let requires_id = self.mark_done // {{{
            || self.unmark_done
            || self.append
            || self.edit
            || self.move_task
            || self.delete;
        // }}}

        if requires_id {
            self.task_id = get_task_id(&mut args);

            check_id(self.task_id);
            // -1 because lines are counted from 0
            self.task_id -= 1;
        }

        if self.add || self.append || self.edit {
            self.task = get_task_content(&mut args);
        }

        if self.move_task {
            self.new_id = get_task_id(&mut args);

            check_id(self.new_id);
            // -1 because lines are counted from 0
            self.new_id -= 1;

            if self.new_id == self.task_id {
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

Usage: {NAME} [Options] [Command] [Args]...

Options:
    --help      -h
        Print this message
    --version   -v
        Print the program version
    --no-color
        Don't make the output colored

Commands:
    print   p 
        Print all tasks
        Default when not passing any args
    do          <task_id>
        Mark a task as done
    undo    u   <task_id>
        Unmark a task as done
    clear   c
        Remove all tasks that are marked as done
    add     a   <task>
        Add a new task
    append  ap  <task_id> <content>
        Append content to an existing task
    edit    e   <task_id> <new_task>
        Replace the contents of an task
    move    m   <task_id> <new_task_id>
        Move a task to a new location
    delete  d   <task_id>
        Delete a task"
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
