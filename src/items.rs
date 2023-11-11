use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
};

pub fn add_item(path: &str, item: &str) {
    // {{{
    let item = item.to_string() + "\n";

    let mut list = OpenOptions::new()
        .append(true)
        .open(path)
        .expect("Unable to open file for writting");

    list.write_all(item.as_bytes())
        .expect("Unable to write to file");
}
// }}}

pub fn append_to_item(path: &str, id: u8, content: &str) {
    // {{{
    let out_path = path.to_string() + ".tmp";

    // Scope ensures files are closed
    {
        let (reader, mut writer) = prep_files(path, &out_path);

        for (i, ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
            if i == id.into() {
                writeln!(writer, "{ln}{content}").expect("Unable to write to tmp file");
            } else {
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            }
        }
    }
    fs::rename(out_path, path).expect("Unable to rename tmp file");
}
// }}}

pub fn edit_item(path: &str, id: u8, new_content: &str) {
    // {{{
    let out_path = path.to_string() + ".tmp";

    // Scope ensures files are closed
    {
        let (reader, mut writer) = prep_files(path, &out_path);

        for (i, ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
            if i == id.into() {
                writeln!(writer, "{new_content}").expect("Unable to write to tmp file");
            } else {
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            }
        }
    }
    fs::rename(out_path, path).expect("Unable to rename tmp file");
}
// }}}

pub fn delete_item(path: &str, id: u8) {
    // {{{
    let out_path = path.to_string() + ".tmp";

    // Scope ensures files are closed
    {
        let (reader, mut writer) = prep_files(path, &out_path);

        for (i, ln) in reader.lines().map(|l| l.unwrap()).enumerate() {
            if i != id.into() {
                writeln!(writer, "{ln}").expect("Unable to write to tmp file");
            }
        }
    }
    fs::rename(out_path, path).expect("Unable to rename tmp file");
}
// }}}

fn prep_files(read_file: &str, out_file: &str) -> (BufReader<File>, BufWriter<File>) {
    // {{{
    let file = File::open(read_file).expect("Unable to open list for reading");
    let out_file = File::create(out_file).expect("Unable to create output file");

    let reader = BufReader::new(file);
    let writer = BufWriter::new(out_file);

    (reader, writer)
}
// }}}

// pub fn move_item(path: &str, from: u8, to: u8) {
// // {{{
// }
// // }}}

#[cfg(test)]
mod test {
    // {{{
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn add_item_ok() {
        // {{{
        let path = crate::get_path("t1");
        add_item(&path, &"new item");

        let f = File::open(&path).unwrap();
        let last_line = get_last_line(f);
        delete_item(&path, 1);

        assert_eq!(last_line, "new item");
    }
    // }}}

    #[test]
    fn delete_item_ok() {
        // {{{
        let path = crate::get_path("t2");
        add_item(&path, &"new item");
        delete_item(&path, 1);

        let f = File::open(&path).unwrap();
        let last_line = get_last_line(f);

        assert_ne!(last_line, "new item");
    }
    // }}}

    #[test]
    fn append_item_ok() {
        // {{{
        let path = crate::get_path("t3");
        add_item(&path, "with");
        append_to_item(&path, 1, " addition");

        let f = File::open(&path).unwrap();
        let last_line = get_last_line(f);
        delete_item(&path, 1);

        assert_eq!(last_line, "with addition");
    }
    // }}}

    #[test]
    fn edit_item_ok() {
        // {{{
        let path = crate::get_path("t4");
        add_item(&path, "original");
        edit_item(&path, 1, "new!!");

        let f = File::open(&path).unwrap();
        let last_line = get_last_line(f);
        delete_item(&path, 1);

        assert_eq!(last_line, "new!!");
    }
    // }}}

    fn get_last_line(file: File) -> String {
        // {{{
        BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .last()
            .unwrap()
    }
    // }}}
}
// }}}
