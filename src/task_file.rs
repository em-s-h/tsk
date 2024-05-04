use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process};

#[derive(Debug, PartialEq)]
pub enum AddPosition {
    // {{{
    Top,
    Bottom,
}
// }}}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskFile {
    // {{{
    pub tasks: Vec<Task>,
}
// }}}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    // {{{
    // id: usize,
    contents: String,
    done: bool,
}
// }}}

impl TaskFile {
    // {{{
    pub fn parse_file(file: &PathBuf) -> Self {
        // {{{
        if fs::metadata(&file)
            .expect("File has been verified to be readable")
            .len()
            == 0
        {
            return TaskFile {
                tasks: vec![Task {
                    contents: "Create a new task file".to_owned(),
                    done: true,
                }],
            };
        }

        let json = fs::read_to_string(file).expect("File has been verified to be readable");
        match serde_json::from_str::<TaskFile>(&json) {
            Ok(tf) => tf,
            Err(err) => {
                eprintln!("Unable to deserialize json string");
                eprintln!("Err: {err}");
                process::exit(1)
            }
        }
    }
    // }}}

    pub fn save_to_file(&self, file: &PathBuf) {
        // {{{
        match serde_json::to_string(&self) {
            Ok(json) => fs::write(file, json).expect("File has been verified to be writable"),
            Err(err) => {
                eprintln!("Unable to serialize TaskFile struct");
                eprintln!("Err: {err}");
                process::exit(1)
            }
        }
    }
    // }}}

    pub fn print(&self, colored: bool) {
        // {{{
        if self.tasks.len() == 0 {
            println!("No tasks to print");
            return;
        }
        println!("Tasks:\n");

        for (id, t) in self.tasks.iter().enumerate() {
            let id = format!("{:>2}", id + 1);
            let task = if t.done {
                format!("[X] {}", t.contents)
            } else {
                format!("[ ] {}", t.contents)
            };
            if t.done && colored {
                println!("{id}. \x1b[0;32m{task} \x1b[0m");
            } else if colored {
                println!("{id}. \x1b[0;31m{task} \x1b[0m");
            } else {
                println!("{id}. {task}");
            }
        }
    }
    // }}}

    pub fn add_task(&mut self, task: &str, pos: &AddPosition) {
        // {{{
        println!("Adding task...");
        let task = Task {
            contents: task.to_owned(),
            done: false,
        };
        if pos == &AddPosition::Top {
            self.tasks.insert(0, task)
        } else {
            self.tasks.push(task)
        }
    }
    // }}}

    pub fn mark_tasks(&mut self, ids: &[usize], done: bool) {
        // {{{
        if done {
            println!("Marking tasks as done...");
        } else {
            println!("Unmarking tasks...");
        }
        for (id, t) in self.tasks.iter_mut().enumerate() {
            if ids.contains(&id) {
                t.done = done
            }
        }
    }
    // }}}

    pub fn move_task(&mut self, from: usize, to: usize) {
        // {{{
        println!("Moving task...");
        if from == to {
            return;
        }
        let task = self.tasks.remove(from);
        self.tasks.insert(to, task);
    }
    // }}}

    pub fn swap_tasks(&mut self, t1: usize, t2: usize) {
        // {{{
        println!("Swapping tasks...");
        self.tasks.swap(t1, t2)
    }
    // }}}

    pub fn append_to_task(&mut self, id: usize, content: &str) {
        // {{{
        println!("Appending content...");
        self.tasks[id].contents.push_str(content);
        self.tasks[id].done = false
    }
    // }}}

    pub fn edit_task(&mut self, id: usize, new_content: &str) {
        // {{{
        println!("Editing task...");
        self.tasks[id].contents = new_content.to_owned();
        self.tasks[id].done = false
    }
    // }}}

    pub fn delete_task(&mut self, id: usize) {
        // {{{
        println!("Deleting task...");
        self.tasks.remove(id);
    }
    // }}}

    pub fn clear_dones(&mut self) {
        // {{{
        println!("Clearing done tasks...");
        let mut dones: Vec<usize> = Vec::new();
        for (id, t) in self.tasks.iter().enumerate() {
            if t.done {
                dones.push(id)
            }
        }
        dones.reverse();

        for id in dones {
            self.tasks.remove(id);
        }
    }
    // }}}
}
// }}}

#[cfg(test)]
mod test {
    // {{{
    use super::*;

    fn get_test_task_file() -> TaskFile {
        // {{{
        TaskFile {
            tasks: vec![
                Task {
                    contents: "one".to_owned(),
                    done: false,
                },
                Task {
                    contents: "two".to_owned(),
                    done: false,
                },
            ],
        }
    }
    // }}}

    fn get_test_done_task_file() -> TaskFile {
        // {{{
        TaskFile {
            tasks: vec![
                Task {
                    contents: "one".to_owned(),
                    done: true,
                },
                Task {
                    contents: "two".to_owned(),
                    done: true,
                },
            ],
        }
    }
    // }}}

    // Adding tasks {{{
    #[test]
    fn test_add_task_top() {
        // {{{
        let mut tf = get_test_task_file();
        tf.add_task("three", &AddPosition::Top);

        assert_eq!(tf.tasks.len(), 3);
        assert_eq!(tf.tasks[0].contents, "three");
        assert_eq!(tf.tasks[1].contents, "one");
        assert_eq!(tf.tasks[2].contents, "two")
    }
    // }}}

    #[test]
    fn test_add_task_bot() {
        // {{{
        let mut tf = get_test_task_file();
        tf.add_task("three", &AddPosition::Bottom);

        assert_eq!(tf.tasks.len(), 3);
        assert_eq!(tf.tasks[0].contents, "one");
        assert_eq!(tf.tasks[1].contents, "two");
        assert_eq!(tf.tasks[2].contents, "three");
    }
    // }}}
    // }}}

    // Marking tasks {{{
    #[test]
    fn test_mark_tasks_done() {
        // {{{
        let mut tf = get_test_task_file();
        println!("Marking one");
        tf.mark_tasks(&[0], true);

        assert!(tf.tasks[0].done);
        assert!(!tf.tasks[1].done);

        let mut tf = get_test_task_file();
        println!("Marking multiple");
        tf.mark_tasks(&[0, 1], true);

        assert!(tf.tasks[0].done);
        assert!(tf.tasks[1].done);
    }
    // }}}

    #[test]
    fn test_mark_tasks_not_done() {
        // {{{
        let mut tf = get_test_done_task_file();
        println!("Unmarking one");
        tf.mark_tasks(&[0], false);

        assert!(!tf.tasks[0].done);
        assert!(tf.tasks[1].done);

        let mut tf = get_test_task_file();
        println!("Unmarking multiple");
        tf.mark_tasks(&[0, 1], false);

        assert!(!tf.tasks[0].done);
        assert!(!tf.tasks[1].done);
    }
    // }}}

    #[test]
    fn test_mark_done_as_done() {
        // {{{
        let mut tf = get_test_done_task_file();
        tf.mark_tasks(&[0, 1], true);

        assert!(tf.tasks[0].done);
        assert!(tf.tasks[1].done);
    }
    // }}}

    #[test]
    fn test_mark_not_done_as_not_done() {
        // {{{
        let mut tf = get_test_task_file();
        tf.mark_tasks(&[0, 1], false);

        assert!(!tf.tasks[0].done);
        assert!(!tf.tasks[1].done);
    }
    // }}}
    // }}}

    // Moving tasks {{{
    #[test]
    fn test_move_task_up() {
        // {{{
        let mut tf = get_test_task_file();
        tf.add_task("other", &AddPosition::Bottom);
        tf.move_task(2, 0);

        assert_eq!(tf.tasks.len(), 3);
        assert_eq!(tf.tasks[0].contents, "other");
        assert_eq!(tf.tasks[1].contents, "one");
        assert_eq!(tf.tasks[2].contents, "two")
    }
    // }}}

    #[test]
    fn test_move_task_down() {
        // {{{
        let mut tf = get_test_task_file();
        tf.add_task("other", &AddPosition::Top);
        tf.move_task(0, 2);

        assert_eq!(tf.tasks.len(), 3);
        assert_eq!(tf.tasks[0].contents, "one");
        assert_eq!(tf.tasks[1].contents, "two");
        assert_eq!(tf.tasks[2].contents, "other");
    }
    // }}}

    #[test]
    fn test_move_task_same_ids() {
        // {{{
        let mut tf = get_test_task_file();
        tf.add_task("other", &AddPosition::Top);
        tf.move_task(0, 0);

        assert_eq!(tf.tasks.len(), 3);
        assert_eq!(tf.tasks[0].contents, "other");
        assert_eq!(tf.tasks[1].contents, "one");
        assert_eq!(tf.tasks[2].contents, "two");
    }
    // }}}
    // }}}

    // Swapping tasks {{{
    #[test]
    fn test_swap_tasks() {
        // {{{
        let mut tf = get_test_task_file();
        tf.add_task("other", &AddPosition::Top);
        tf.swap_tasks(0, 2);

        assert_eq!(tf.tasks.len(), 3);
        assert_eq!(tf.tasks[0].contents, "two");
        assert_eq!(tf.tasks[1].contents, "one");
        assert_eq!(tf.tasks[2].contents, "other");
    }
    // }}}

    #[test]
    fn test_swap_tasks_same_ids() {
        // {{{
        let mut tf = get_test_task_file();
        tf.add_task("other", &AddPosition::Top);
        tf.swap_tasks(0, 0);

        assert_eq!(tf.tasks.len(), 3);
        assert_eq!(tf.tasks[0].contents, "other");
        assert_eq!(tf.tasks[1].contents, "one");
        assert_eq!(tf.tasks[2].contents, "two");
    }
    // }}}
    // }}}

    // Appending tasks {{{
    #[test]
    fn test_append_task() {
        // {{{
        let mut tf = get_test_task_file();
        tf.append_to_task(0, "new");

        assert_eq!(tf.tasks[0].contents, "onenew");
    }
    // }}}

    #[test]
    fn test_append_task_undo_done() {
        // {{{
        let mut tf = get_test_done_task_file();
        tf.append_to_task(0, "new");

        assert_eq!(tf.tasks[0].contents, "onenew");
        assert!(!tf.tasks[0].done);
    }
    // }}}
    // }}}

    // Editing tasks {{{
    #[test]
    fn test_edit_task() {
        // {{{
        let mut tf = get_test_task_file();
        tf.edit_task(0, "new");

        assert_eq!(tf.tasks[0].contents, "new");
    }
    // }}}

    #[test]
    fn test_edit_task_undo_done() {
        // {{{
        let mut tf = get_test_done_task_file();
        tf.edit_task(0, "new");

        assert_eq!(tf.tasks[0].contents, "new");
        assert!(!tf.tasks[0].done);
    }
    // }}}
    // }}}

    // Deleting tasks {{{
    #[test]
    fn test_delete_task() {
        // {{{
        let mut tf = get_test_task_file();
        tf.delete_task(0);

        assert_eq!(tf.tasks.len(), 1);
    }
    // }}}

    #[test]
    fn test_clear_dones() {
        // {{{
        let mut tf = get_test_task_file();
        tf.tasks[0].done = true;
        tf.clear_dones();

        assert_eq!(tf.tasks.len(), 1);
    }
    // }}}
    // }}}
}
// }}}
