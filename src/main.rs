use cli::Cli;
use std::process;
use task_file::TaskFile;

pub mod cli;
pub mod task_file;

fn main() {
    let cli = Cli::parse_args(None).unwrap_or_else(|e| {
        eprintln!("Cli error: {e}");
        process::exit(1)
    });
    let mut task_file = TaskFile::load();

    // Commands that don't need ids
    match cli.command.as_str() {
        "print" => {
            task_file.print(cli.colored_output);
            process::exit(0)
        }
        "add" if cli.task_ids.is_empty() => {
            task_file.add_task(&cli.contents, &cli.add_to, &cli.task_ids);
            task_file.save();
            task_file.print(cli.colored_output);
            process::exit(0)
        }
        "clear" => {
            task_file.clear_dones();
            task_file.save();
            task_file.print(cli.colored_output);
            process::exit(0)
        }
        _ => (),
    }

    if let Err(e) = verify_ids(&cli.task_ids, &task_file) {
        eprintln!("Id error: {e}");
        process::exit(1)
    }

    let id = if cli.task_ids.contains(',') {
        match Cli::parse_id_list(&cli.task_ids) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{e}");
                process::exit(1)
            }
        }
    } else if cli.task_ids.contains("..") {
        match Cli::parse_id_range(&cli.task_ids) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{e}");
                process::exit(1)
            }
        }
    } else if cli.task_ids == "all" {
        let i = format!("1..{}", task_file.get_task_count());
        match Cli::parse_id_range(&i) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{e}");
                process::exit(1)
            }
        }
    } else {
        cli.task_ids
    };

    if cli.command == "move" || cli.command == "swap" {
        if let Err(e) = verify_ids(&cli.move_id, &task_file) {
            eprintln!("Id error: {e}");
            process::exit(1)
        }
    }

    // Commands that need ids
    match cli.command.as_str() {
        "add" => {
            task_file.add_task(&cli.contents, &cli.add_to, &id);
        }
        "do" => {
            task_file.mark_tasks(&id, true);
        }
        "undo" => {
            task_file.mark_tasks(&id, false);
        }
        "move" => {
            task_file.move_task(&id, &cli.move_id);
        }
        "swap" => {
            task_file.swap_tasks(&id, &cli.move_id);
        }
        "append" => {
            task_file.append_to_task(&id, &cli.contents);
        }
        "edit" => {
            task_file.edit_task(&id, &cli.contents);
        }
        "delete" => {
            task_file.delete_task(&id);
        }
        _ => {
            println!("Error: Invalid command");
            process::exit(1)
        }
    }

    task_file.save();
    task_file.print(cli.colored_output);
}

fn verify_ids(ids: &str, tf: &TaskFile) -> Result<(), String> {
    if ids.is_empty() {
        return Err("No id provided".to_owned());
    }
    if ids == "all" {
        return Ok(());
    }
    if ids == "0" {
        return Err(format!("Invalid id `{ids}`"));
    }
    if ids.contains(|c: char| !c.is_digit(10) && c != '.' && c != ',') {
        return Err("Id contains invalid characters".to_owned());
    }
    if ids.matches("..").count() > 1 {
        return Err("Range pattern can only be used once".to_owned());
    }
    if ids.contains("..") && ids.contains(',') {
        return Err("Range pattern cannot be used in a list".to_owned());
    }
    if ids.contains("...") || ids.contains(",,") {
        return Err("Id list contains invalid patterns".to_owned());
    }

    let ids: Vec<&str> = if ids.contains("..") {
        ids.split("..").collect()
    } else {
        ids.split(',').collect()
    };
    let t_count = tf.get_task_count();

    for id in ids {
        if !id.contains('.') {
            match id.parse::<usize>() {
                Ok(v) if v > t_count => return Err(format!("Id `{id}` is out of bounds")),
                Ok(v) if v == 0 => return Err(format!("Invalid id `{id}`")),
                Err(e) => return Err(format!("Invalid id `{id}\nError `{e}`")),
                _ => (),
            }
            continue;
        }
        let i: Vec<&str> = id.split('.').collect();
        match i[0].parse::<usize>() {
            Ok(v) if v > t_count => return Err(format!("Id `{id}` is out of bounds")),
            Ok(v) if v == 0 => return Err(format!("Invalid id `{id}`")),
            Err(e) => return Err(format!("Invalid id `{id}\nError `{e}`")),
            _ => (),
        }

        let st_count = tf.get_subtask_count(i[0].parse::<usize>().unwrap() - 1);
        match i[1].parse::<usize>() {
            Ok(v) if v > st_count => return Err(format!("Id `{id}` is out of bounds")),
            Ok(v) if v == 0 => return Err(format!("Invalid id `{id}`")),
            Err(e) => return Err(format!("Invalid id `{id}\nError `{e}`")),
            _ => (),
        }
    }
    Ok(())
}
