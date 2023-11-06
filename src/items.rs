use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
};

pub struct Item {
    pub contents: String,
    pub id: u8,
}

pub fn add_item(path: &str, item: &str) {
    // {{{
    let item = item.to_string() + "\n";
    crate::check_list(&path);

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
    let out_path = path.to_string() + ".tmp";

    // Scope ensures files are closed
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

pub fn edit_item(path: &str, id: u8, new_content: &str) {
    // {{{
}
// }}}

pub fn append_to_item(path: &str, id: u8, content: &str) {
    // {{{
    crate::check_list(path);
    let out_path = path.to_string() + ".tmp";

    // Scope ensures files are closed
    {
        let file = File::open(path).expect("Unable to open list for reading");
        let out_file = File::create(&out_path).expect("Unable to create output file");

        let reader = BufReader::new(&file);
        let mut writer = BufWriter::new(&out_file);
        let id = id - 1;

        for (i, ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
            if i != id.into() {
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            } else {
                writeln!(writer, "{ln}{content}").expect("Unable to write to tmp file");
            }
        }
    }
    fs::rename(out_path, path).expect("Unable to rename tmp file");
}
// }}}

pub fn move_item(path: &str, from: u8, to: u8) {
    // {{{
}
// }}}

#[cfg(test)]
mod test {
    // {{{
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn add_item_ok() {
        // {{{
        let path = crate::get_path("t2");
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
        let path = crate::get_path("t3");
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
}
// }}}
