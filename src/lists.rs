use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
    process,
};

// const LISTS_DIR: &str = "~/.local/share/lists/";
const LISTS_DIR: &str = "./lists/"; // For debugging

pub fn create_list(name: &str) {
    // {{{
    let path = get_path(name);
    if list_exists(&path) {
        println!("This list already exists!");
        process::exit(0);
    }

    File::create(path).expect("Unable to create file");
    // }}}
}

pub fn delete_list(name: &str, is_confirmed: bool) {
    // {{{
    use std::io::{self, Write};

    let path = get_path(name);
    if !list_exists(&path) {
        eprintln!("This list doesn't exist!");
        process::exit(1);
    }

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
    if !list_exists(&path) {
        eprintln!("This list doesn't exist!");
        process::exit(1);
    }
    println!("[{name}]");

    let file = File::open(path).expect("Unable to open file.");
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    loop {
        if let Ok(b) = reader.read_line(&mut buffer) {
            if b == 0 {
                break;
            }

            print!("{}", buffer);
            buffer.clear();
        } else {
            panic!("Unable to read file {}", name);
        }
    }
    // }}}
}

fn list_exists(path: &str) -> bool {
    // {{{
    let path = Path::new(path);
    if let Ok(exists) = path.try_exists() {
        exists
    } else {
        eprintln!("Unable to verify the existence of file");
        false
    }
    // }}}
}

fn get_path(name: &str) -> String {
    format!("{}{}", LISTS_DIR, name)
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::*;

    // {{{
    #[test]
    fn create_list_ok() {
        create_list(&"test");
        let exists = Path::new(&get_path("test")).try_exists().unwrap();

        assert!(exists);
    }

    #[test]
    fn delete_list_ok() {
        create_list_ok();
        delete_list(&"test", true);
        let exists = Path::new(&get_path("test")).try_exists().unwrap();

        assert!(!exists);
    }
    // }}}
}
