use cli::Cli;
use std::process;
use task_file::TaskFile;

pub mod cli;
pub mod task_file;

fn main() {
    let cli = Cli::parse_args();
    let mut task_file = TaskFile::load();

    // Commands that don't need ids
    if cli.command == "print" {
        task_file.print(cli.colored_output);
        process::exit(0)
    } else if cli.command == "clear" {
        task_file.clear_dones();
        process::exit(0)
    }

    if let Err(e) = verify_ids(&cli.task_ids) {
        println!("Error: {e}");
        process::exit(1)
    }

    // Commands that need ids
    match cli.command.as_str() {
        "add" => {
            println!("todo");
        }
        "do" => {
            println!("todo");
        }
        "undo" => {
            println!("todo");
        }
        "move" => {
            if let Err(e) = verify_ids(&cli.move_id) {
                println!("Error: {e}");
                process::exit(1)
            }
            println!("todo");
        }
        "swap" => {
            if let Err(e) = verify_ids(&cli.move_id) {
                println!("Error: {e}");
                process::exit(1)
            }
            println!("todo");
        }
        "append" => {
            println!("todo");
        }
        "edit" => {
            println!("todo");
        }
        "delete" => {
            println!("todo");
        }
        _ => {
            println!("Error: Invalid command");
            process::exit(1)
        }
    }

    task_file.save();
}

fn verify_ids(ids: &str) -> Result<(), &'static str> {
    if ids.is_empty() {
        return Err("No id provided");
    }
    if ids.contains(|c: char| !c.is_digit(10) && c != '.' && c != ',') {
        return Err("Id contains invalid characters");
    }
    if ids.contains("..") && ids.contains(',') {
        return Err("Id list contains more than one pattern");
    }
    if ids.contains("...") || ids.contains(",,") {
        return Err("Id list contains invalid patterns");
    }
    Ok(())
}

// Required for integration tests
#[cfg(test)]
mod tests {}
