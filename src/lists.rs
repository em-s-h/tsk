use std::{
    fs::{self, File},
    process,
};

const LISTS_DIR: &str = "~/.local/share/lists/";

pub fn create_list(name: &str) -> File {
    // {{{
    let path = get_path(name);
    File::create(path).expect("Unable to create file")
    // }}}
}

pub fn delete_list(name: &str, is_confirmed: bool) {
    // {{{
    use std::io::{self, Write};

    let path = get_path(name);

    if !is_confirmed {
        println!("Are you sure you want to delete {}?", name);
        print!("(y/n): ");

        io::stdout().flush().expect("Unable to flush stdout");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read from stdin");

        if input.to_lowercase().starts_with("n") {
            println!("Aborting...");
            process::exit(0);
        }
    }

    if let Err(e) = fs::remove_file(path) {
        eprintln!("Unable to delete list, please make sure the list exists.");
        eprintln!("Error: {e}");
        process::exit(1);
    } else {
        process::exit(0);
    }
    // }}}
}

pub fn print_list(name: &str) {
    // {{{
    let path = get_path(name);
    println!("[{name}]");
    // }}}
}

fn get_path(name: &str) -> String {
    format!("{}{}", LISTS_DIR, name)
}
