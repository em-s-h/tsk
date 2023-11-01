use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::Path,
    process,
};

// const LISTS_DIR: &str = "~/.local/share/lists/";
const LISTS_DIR: &str = "./lists/"; // For debugging

pub fn create_list(path: &str) {
    // {{{
    if list_exists(&path) {
        println!("This list already exists!");
        return;
    }

    File::create(path).expect("Unable to create file");
}
// }}}

pub fn remove_list(path: &str, name: &str, is_confirmed: bool) {
    // {{{
    check_list(&path);

    if !is_confirmed {
        println!("Are you sure you want to delete {name}?");
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
        eprintln!("Unable to delete {name}.");
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
// }}}

pub fn print_list(path: &str, name: &str) {
    // {{{
    check_list(&path);
    println!("List: [ {name} ]\n");

    let file = File::open(path).expect("Unable to open file for reading");
    let reader = BufReader::new(file);

    for (id, l) in reader.lines().map(|l| l.unwrap()).enumerate() {
        println!("{}. {l}", id + 1);
    }
}
// }}}

pub fn add_item(path: &str, item: &str) {
    // {{{
    let item = item.to_string() + "\n";
    check_list(&path);

    let mut list = OpenOptions::new()
        .append(true)
        .open(path)
        .expect("Unable to open file for writting");

    list.write_all(item.as_bytes())
        .expect("Unable to write to file");
}
// }}}

pub fn delete_item(path: &str, id: u8) {
    // {{{
    // Scope ensures files are closed
    let out_path = path.to_string() + ".tmp";
    {
        let file = File::open(path).expect("Unable to open list for reading");
        let out_file = File::create(&out_path).expect("Unable to create output file");

        let reader = BufReader::new(&file);
        let mut writer = BufWriter::new(&out_file);
        let id = id - 1;

        for (i, ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
            if i != id.into() {
                writeln!(writer, "{}", ln).expect("Unable to write to tmp file");
            }
        }
    }
    fs::rename(out_path, path).expect("Unable to rename tmp file");
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

pub fn get_path(list: &str) -> String {
    format!("{}{}", LISTS_DIR, list)
}

#[cfg(test)]
mod test {
    // {{{
    use super::*;

    #[test]
    fn create_list_ok() {
        // {{{
        let path = get_path("test");
        create_list(&path);
        let exists = Path::new(&path).try_exists().unwrap();

        remove_list(&path, "test", true);
        assert_eq!(exists, true)
    }
    // }}}

    #[test]
    fn remove_list_ok() {
        // {{{
        let path = get_path("t1");
        create_list(&path);
        remove_list(&path, "t1", true);
        let exists = Path::new(&path).try_exists().unwrap();

        assert!(!exists);
    }
    // }}}

    #[test]
    fn add_item_ok() {
        // {{{
        let path = get_path("t2");
        add_item(&path, &"new item");

        let f = File::open(&path).unwrap();
        let last_line = BufReader::new(f)
            .lines()
            .map(|l| l.unwrap())
            .last()
            .unwrap();

        assert_eq!(last_line, "new item");
    }
    // }}}

    #[test]
    fn delete_item_ok() {
        // {{{
        let path = get_path("t3");
        add_item(&path, &"new item");
        delete_item(&path, 8);

        let f = File::open(&path).unwrap();
        let last_line = BufReader::new(f)
            .lines()
            .map(|l| l.unwrap())
            .last()
            .unwrap();

        assert_ne!(last_line, "new item");
    }
    // }}}

    #[test]
    #[ignore = "Use only with '--show-output'"]
    fn print_list_ok() {
        print_list(&get_path("t2"), "t2");
    }
}
// }}}
