use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Write},
    process,
};

pub fn show_lists() {
    // {{{
    let dir = crate::get_lists_dir();

    let entries = fs::read_dir(dir).expect("Unable to read the contents of the lists directory");
    for entry in entries {
        let entry = entry.unwrap().file_name();
        let entry = entry.to_str().expect("Unable to convert OsString to str");
        println!("{entry}");
    }
}
// }}}

pub fn create_list(path: &str) {
    // {{{
    if crate::list_exists(&path) {
        println!("This list already exists!");
        return;
    }

    File::create(path).expect("Unable to create file");
}
// }}}

pub fn remove_list(path: &str, name: &str, confirmed: bool) {
    // {{{
    if !confirmed {
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
    println!("List: [ {name} ]\n");

    let file = File::open(path).expect("Unable to open file for reading");
    let reader = BufReader::new(file);

    for (id, l) in reader.lines().map(|l| l.unwrap()).enumerate() {
        println!("{}. {l}", id + 1);
    }
}
// }}}

#[cfg(test)]
mod test {
    // {{{
    use super::*;
    use std::path::Path;

    #[test]
    #[ignore = "Use only with '--show-output'"]
    fn show_lists_ok() {
        show_lists();
    }

    #[test]
    fn create_list_ok() {
        // {{{
        let path = crate::get_path("test");
        create_list(&path);
        let exists = Path::new(&path).try_exists().unwrap();

        remove_list(&path, "test", true);
        assert_eq!(exists, true)
    }
    // }}}

    #[test]
    fn remove_list_ok() {
        // {{{
        let path = crate::get_path("test2");
        create_list(&path);
        remove_list(&path, "test2", true);
        let exists = Path::new(&path).try_exists().unwrap();

        assert!(!exists);
    }
    // }}}

    #[test]
    #[ignore = "Use only with '--show-output'"]
    fn print_list_ok() {
        print_list(&crate::get_path("t2"), "t2");
    }
}
// }}}
