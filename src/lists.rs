use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, BufWriter, Write},
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
}
// }}}

pub fn remove_list(name: &str, is_confirmed: bool) {
    // {{{
    let path = get_path(name);
    check_list(&path);

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
        eprintln!("Unable to delete list.");
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
// }}}

pub fn print_list(name: &str) {
    // {{{
    let path = get_path(name);
    check_list(&path);
    println!("List: [ {name} ]\n");

    let file = File::open(path).expect("Unable to open file.");
    let reader = BufReader::new(file);
    let mut id: u8 = 0;

    for l in reader.lines().map(|l| l.unwrap_or_default()) {
        id += 1;
        println!("{}. {}", id, l);
    }
}
// }}}

pub fn add_item(name: &str, item: &str) {
    // {{{
    let item = item.to_string() + "\n";
    let path = get_path(name);
    check_list(&path);

    let mut list = OpenOptions::new()
        .append(true)
        .open(path)
        .expect("Unable to open file for writting");

    list.write_all(item.as_bytes())
        .expect("Unable to write to file");
}
// }}}

pub fn delete_item(name: &str, id: u8) {
    // {{{
}
// }}}

/// Check if the list exists and warn the user
fn check_list(path: &str) {
    // {{{
    if !list_exists(&path) {
        eprintln!("This list doesn't exist!");
        process::exit(1);
    }
}
// }}}

fn list_exists(path: &str) -> bool {
    // {{{
    let path = Path::new(path);
    if let Ok(exists) = path.try_exists() {
        exists
    } else {
        eprintln!("Unable to verify the existence of file");
        false
    }
}
// }}}

fn get_path(name: &str) -> String {
    format!("{}{}", LISTS_DIR, name)
}

#[cfg(test)]
mod test {
    // {{{
    use super::*;

    #[test]
    fn create_list_ok() {
        // {{{
        create_list(&"test");
        let exists = Path::new(&get_path("test")).try_exists().unwrap();

        remove_list(&"test", true);
        assert_eq!(exists, true)
    }
    // }}}

    #[test]
    fn remove_list_ok() {
        // {{{
        create_list(&"t1");
        remove_list(&"t1", true);
        let exists = Path::new(&get_path("t1")).try_exists().unwrap();

        assert!(!exists);
    }
    // }}}

    #[test]
    fn add_item_ok() {
        // {{{
        add_item(&"t2", &"new item");

        let f = File::open(&get_path("t2")).unwrap();
        let last_line = BufReader::new(f)
            .lines()
            .map(|l| l.unwrap())
            .last()
            .unwrap();

        assert_eq!(last_line, "new item");
    }
    // }}}

    #[test]
    #[ignore = "Use only with '--show-output'"]
    fn print_list_ok() {
        print_list(&"t2");
    }
}
// }}}
