use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    path::{Path, PathBuf},
    process,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskFile {
    task_file_path: PathBuf,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    contents: String,
    done: bool,
    pub subtasks: Vec<SubTask>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubTask {
    contents: String,
    done: bool,
}

impl TaskFile {
    pub fn load() -> Self {
        let proj = ProjectDirs::from("tsk", "Emilly", "tsk").unwrap_or_else(|| {
            eprintln!("Unable to retrieve/create the project directory");
            process::exit(1)
        });

        let data_dir = proj.data_local_dir();
        let mut file = data_dir.to_path_buf();
        file.push("tasks.json");

        match Path::try_exists(&file) {
            Ok(false) | Err(_) => {
                println!("Creating new 'tasks' file\n");

                let f = format!("{}/tasks.json", data_dir.to_str().unwrap());
                File::create(f).is_err().then(|| {
                    eprintln!("Unable to create 'tasks' file");
                    process::exit(1);
                });
            }
            Ok(true) => match OpenOptions::new().append(true).read(true).open(&file) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Unable to open 'tasks' file for reading & writting");
                    eprintln!("Err: {e}");
                    process::exit(1)
                }
            },
        }

        // If the file is empty return a default config.
        if fs::metadata(&file)
            .expect("File has been verified to be readable")
            .len()
            == 0
        {
            return Self {
                task_file_path: file,
                tasks: vec![Task {
                    contents: "Create a new task file".to_owned(),
                    done: true,
                    subtasks: Vec::new(),
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

    pub fn save(&self) {
        match serde_json::to_string(&self) {
            Ok(json) => fs::write(&self.task_file_path, json)
                .expect("File has been verified to be writable"),
            Err(err) => {
                eprintln!("Unable to serialize TaskFile struct");
                eprintln!("Err: {err}");
                process::exit(1)
            }
        };
    }

    pub fn get_task_count(&self) -> usize {
        self.tasks.iter().count()
    }

    pub fn get_subtask_count(&self, id: usize) -> usize {
        self.tasks[id].subtasks.iter().count()
    }

    fn parse_id(id: &str) -> usize {
        let id: usize = id.parse().expect("`id` verified to be usize in `main.rs`");
        id - 1
    }

    fn parse_sub_id(id: &str) -> Vec<usize> {
        let mut id: Vec<usize> = id
            .split('.')
            .map(|i| i.parse().expect("`i` verified to be usize in `main.rs`"))
            .collect();
        id[0] -= 1;
        id[1] -= 1;
        id
    }

    pub fn print(&self, colored: bool) {
        if self.tasks.len() == 0 {
            println!("No tasks to print");
            return;
        }
        let _print = |id: &str, mark: &str, cont: &str| {
            if mark.contains('X') && colored {
                println!("{id}. \x1b[0;32m{mark} {cont} \x1b[0m");
            } else if colored {
                println!("{id}. \x1b[0;31m{mark} {cont} \x1b[0m");
            } else {
                println!("{id}. {mark} {cont}");
            }
        };

        for (id, t) in self.tasks.iter().enumerate() {
            let done = if t.done { "[X]" } else { "[ ]" };
            let id = format!("{}", id + 1);
            _print(&id, done, &t.contents);

            if t.subtasks.len() == 0 {
                continue;
            }

            for (s_id, s_t) in t.subtasks.iter().enumerate() {
                let done = if s_t.done { "[X]" } else { "[ ]" };
                let s_id = format!("\t{}.{}", id, s_id + 1);
                _print(&s_id, done, &s_t.contents);
            }
        }
    }

    pub fn add_task(&mut self, contents: &str, to: &str, task_id: &str) {
        println!("Adding task...");
        let task = Task {
            contents: contents.to_owned(),
            done: false,
            subtasks: Vec::new(),
        };

        if task_id.is_empty() {
            if to == "top" {
                self.tasks.insert(0, task);
            } else {
                self.tasks.push(task);
            }
            return;
        }
        let task = SubTask::from_task(task);
        let task_id = Self::parse_id(task_id);

        for (id, t) in self.tasks.iter_mut().enumerate() {
            if id != task_id {
                continue;
            }
            if to == "top" {
                t.subtasks.insert(0, task);
            } else {
                t.subtasks.push(task);
            }
            t.done = false;
            return;
        }
    }

    pub fn mark_tasks(&mut self, ids: &str, done: bool) {
        if done {
            println!("Marking tasks as done...");
        } else {
            println!("Unmarking tasks...");
        }

        let (ids, sub_ids) = if ids.contains('.') {
            let id1: Vec<usize> = ids
                .split(',')
                .filter(|s| !s.contains('.'))
                .map(|s| s.parse().expect("ids verified to be usize in main.rs"))
                .collect();

            let id2: Vec<String> = ids
                .split(',')
                .filter(|s| s.contains('.'))
                .map(|s| s.to_string())
                .collect();
            (id1, id2)
        } else {
            let ids = ids
                .split(',')
                .map(|s| s.parse().expect("ids verified to be usize in main.rs"))
                .collect();
            (ids, vec![])
        };

        for i in sub_ids {
            let i = Self::parse_sub_id(&i);
            let task = &mut self.tasks[i[0]];
            task.subtasks[i[1]].done = done;

            // If all the subtasks are done, so will the task.
            let done_count = task.subtasks.iter().filter(|t| t.done).count();
            task.done = done_count == task.subtasks.len()
        }

        for i in ids {
            let i = i - 1;
            let task = &mut self.tasks[i];
            task.done = done;

            // If a task is marked done, so will it's subtasks.
            if task.subtasks.is_empty() || !done {
                continue;
            }
            for t in task.subtasks.iter_mut() {
                t.done = true
            }
        }
    }

    pub fn move_task(&mut self, from: &str, to: &str) {
        println!("Moving task...");
        if !from.contains('.') {
            let from = Self::parse_id(from);
            let task = self.tasks.remove(from);

            if !to.contains('.') {
                let to: usize = Self::parse_id(to);

                // Adjust for when `from` gets removed from the array.
                self.tasks.insert(to, task);
                return;
            }
            let sub_task = SubTask::from_task(task);
            let to = Self::parse_sub_id(to);

            let task_id = if from > to[0] { to[0] } else { to[0] - 1 };
            self.tasks[task_id].subtasks.insert(to[1], sub_task);
            return;
        }
        let from = Self::parse_sub_id(from);
        let sub_task = self.tasks[from[0]].subtasks.remove(from[1]);

        if !to.contains('.') {
            let to = Self::parse_id(to);
            let task = Task::from_sub_task(sub_task);
            self.tasks.insert(to, task);
            return;
        }
        let to = Self::parse_sub_id(to);
        self.tasks[to[0]].subtasks.insert(to[1], sub_task);
    }

    pub fn swap_tasks(&mut self, task1: &str, task2: &str) {
        println!("Swapping tasks...");
        let t1 = if task1.contains('.') {
            let id = Self::parse_sub_id(task1);

            let t = self.tasks[id[0]].subtasks[id[1]].clone();
            Task::from_sub_task(t)
        } else {
            let id = Self::parse_id(task1);
            self.tasks[id].clone()
        };

        let t2 = if task2.contains('.') {
            let id = Self::parse_sub_id(task2);
            let t = self.tasks[id[0]].subtasks[id[1]].clone();

            self.tasks[id[0]].subtasks[id[1]] = SubTask::from_task(t1);
            Task::from_sub_task(t)
        } else {
            let id = Self::parse_id(task2);
            let t = self.tasks[id].clone();

            self.tasks[id] = t1;
            t
        };

        if task1.contains('.') {
            let id = Self::parse_sub_id(task1);
            self.tasks[id[0]].subtasks[id[1]] = SubTask::from_task(t2);
        } else {
            let id = Self::parse_id(task1);
            self.tasks[id] = t2;
        }
    }

    pub fn append_to_task(&mut self, id: &str, content: &str) {
        println!("Appending content...");
        let content = format!(" {content}");

        if !id.contains('.') {
            let id = Self::parse_id(id);

            self.tasks[id].contents.push_str(&content);
            self.tasks[id].done = false;
            return;
        }
        let id = Self::parse_sub_id(id);
        let task = &mut self.tasks[id[0]];

        task.subtasks[id[1]].contents.push_str(&content);
        task.subtasks[id[1]].done = false;
        task.done = false
    }

    pub fn edit_task(&mut self, id: &str, new_content: &str) {
        println!("Editing task...");
        let new_content = new_content.to_owned();

        if !id.contains('.') {
            let id = Self::parse_id(id);
            self.tasks[id].contents = new_content;
            self.tasks[id].done = false;
            return;
        }
        let id = Self::parse_sub_id(id);
        let task = &mut self.tasks[id[0]];

        task.subtasks[id[1]].contents = new_content;
        task.subtasks[id[1]].done = false;
        task.done = false;
    }

    pub fn delete_task(&mut self, id: &str) {
        println!("Deleting task...");
        if !id.contains('.') {
            let id = Self::parse_id(id);
            self.tasks.remove(id);
            return;
        }
        let id = Self::parse_sub_id(id);
        self.tasks[id[0]].subtasks.remove(id[1]);
    }

    pub fn clear_dones(&mut self) {
        println!("Clearing done tasks...");
        self.tasks = self
            .tasks
            .iter()
            .filter(|t| !t.done)
            .map(|t| t.to_owned())
            .collect();

        for t in self.tasks.iter_mut() {
            if t.subtasks.is_empty() {
                continue;
            }
            t.subtasks = t
                .subtasks
                .iter()
                .filter(|st| !st.done)
                .map(|st| st.to_owned())
                .collect()
        }
    }
}

impl Task {
    pub fn from_sub_task(sub: SubTask) -> Self {
        Self {
            contents: sub.contents,
            done: sub.done,
            subtasks: vec![],
        }
    }
}

impl SubTask {
    pub fn from_task(task: Task) -> Self {
        return Self {
            contents: task.contents,
            done: task.done,
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_task_file() -> TaskFile {
        TaskFile {
            task_file_path: PathBuf::new(),
            tasks: vec![
                Task {
                    contents: "one".to_owned(),
                    done: false,
                    subtasks: vec![
                        SubTask {
                            contents: "one".to_owned(),
                            done: false,
                        },
                        SubTask {
                            contents: "two".to_owned(),
                            done: false,
                        },
                    ],
                },
                Task {
                    contents: "two".to_owned(),
                    done: false,
                    subtasks: vec![
                        SubTask {
                            contents: "one".to_owned(),
                            done: false,
                        },
                        SubTask {
                            contents: "two".to_owned(),
                            done: false,
                        },
                    ],
                },
            ],
        }
    }

    fn get_done_test_task_file() -> TaskFile {
        TaskFile {
            task_file_path: PathBuf::new(),
            tasks: vec![
                Task {
                    contents: "one".to_owned(),
                    done: true,
                    subtasks: vec![
                        SubTask {
                            contents: "one".to_owned(),
                            done: true,
                        },
                        SubTask {
                            contents: "two".to_owned(),
                            done: true,
                        },
                    ],
                },
                Task {
                    contents: "two".to_owned(),
                    done: true,
                    subtasks: vec![
                        SubTask {
                            contents: "one".to_owned(),
                            done: true,
                        },
                        SubTask {
                            contents: "two".to_owned(),
                            done: true,
                        },
                    ],
                },
            ],
        }
    }

    // I've not found a way to automatically test `print` so for now it's ignored by default
    // and requires manual testing, or just do `cargo r`
    #[test]
    #[ignore = "Manually view the output of this test"]
    fn test_print() {
        let tf = get_test_task_file();
        tf.print(true);
    }

    // ADDING TASKS
    #[test]
    fn test_add_task_top() {
        let mut tf = get_test_task_file();
        tf.add_task("three", "top", "");

        let v: Vec<&str> = tf.tasks.iter().map(|t| t.contents.as_str()).collect();
        assert_eq!(v, ["three", "one", "two"]);
    }

    #[test]
    fn test_add_task_bottom() {
        let mut tf = get_test_task_file();
        tf.add_task("three", "bot", "");
        tf.add_task("four", "bottom", "");

        let v: Vec<&str> = tf.tasks.iter().map(|t| t.contents.as_str()).collect();
        assert_eq!(v, ["one", "two", "three", "four"]);
    }

    #[test]
    fn test_add_sub_top() {
        let mut tf = get_test_task_file();
        tf.add_task("sub", "top", "2");

        let v: Vec<&str> = tf.tasks.iter().map(|t| t.contents.as_str()).collect();
        let sv0: Vec<&str> = tf.tasks[0]
            .subtasks
            .iter()
            .map(|t| t.contents.as_str())
            .collect();
        let sv1: Vec<&str> = tf.tasks[1]
            .subtasks
            .iter()
            .map(|t| t.contents.as_str())
            .collect();

        assert_eq!(v, ["one", "two"]);
        assert_eq!(sv0, ["one", "two"]);
        assert_eq!(sv1, ["sub", "one", "two"]);
    }

    #[test]
    fn test_add_sub_bottom() {
        let mut tf = get_test_task_file();
        tf.add_task("sub3", "bot", "2");
        tf.add_task("sub4", "bottom", "2");

        let v: Vec<&str> = tf.tasks.iter().map(|t| t.contents.as_str()).collect();
        let sv0: Vec<&str> = tf.tasks[0]
            .subtasks
            .iter()
            .map(|t| t.contents.as_str())
            .collect();
        let sv1: Vec<&str> = tf.tasks[1]
            .subtasks
            .iter()
            .map(|t| t.contents.as_str())
            .collect();

        println!("{:#?}", tf);
        assert_eq!(v, ["one", "two"]);
        assert_eq!(sv0, ["one", "two"]);
        assert_eq!(sv1, ["one", "two", "sub3", "sub4"]);
    }

    #[test]
    fn test_adding_sub_undoes_task() {
        let mut tf = get_done_test_task_file();

        assert_eq!(tf.tasks[1].done, true);
        tf.add_task("sub", "top", "2");
        assert_eq!(tf.tasks[1].done, false);
    }

    // MARKING TASKS
    #[test]
    fn test_mark_tasks() {
        let mut tf = get_test_task_file();
        tf.mark_tasks("1", true);
        tf.mark_tasks("2", true);

        assert!(tf.tasks[0].done);
        assert!(tf.tasks[1].done);

        tf.mark_tasks("1", false);
        tf.mark_tasks("2", false);

        assert!(!tf.tasks[0].done);
        assert!(!tf.tasks[1].done);
    }

    #[test]
    fn test_mark_sub() {
        let mut tf = get_test_task_file();
        tf.mark_tasks("1.1", true);
        assert!(!tf.tasks[0].done);
        assert!(tf.tasks[0].subtasks[0].done);

        tf.mark_tasks("1.2", true);
        assert!(tf.tasks[0].done);
        assert!(tf.tasks[0].subtasks[1].done);
        assert!(!tf.tasks[1].done);

        tf.mark_tasks("1.1", false);
        assert!(!tf.tasks[0].done);
        assert!(!tf.tasks[0].subtasks[0].done);

        tf.mark_tasks("1.2", false);
        assert!(!tf.tasks[0].subtasks[1].done);
    }

    #[test]
    fn test_marking_task_done_marks_subtasks() {
        let mut tf = get_test_task_file();

        tf.mark_tasks("2", true);
        assert!(!tf.tasks[0].done);
        assert!(tf.tasks[1].done);
        assert!(tf.tasks[1].subtasks[0].done);
        assert!(tf.tasks[1].subtasks[1].done);

        tf.mark_tasks("2", false);
        assert!(!tf.tasks[1].done);
        assert!(tf.tasks[1].subtasks[0].done);
        assert!(tf.tasks[1].subtasks[1].done);
    }

    // MOVING TASKS
    #[test]
    fn test_move_task() {
        let mut tf = get_test_task_file();
        tf.add_task("other", "bottom", "");
        tf.add_task("more", "bottom", "");
        tf.move_task("3", "1");

        let v: Vec<&str> = tf.tasks.iter().map(|t| t.contents.as_str()).collect();
        assert_eq!(v, ["other", "one", "two", "more"]);

        tf.move_task("1", "3");

        let v: Vec<&str> = tf.tasks.iter().map(|t| t.contents.as_str()).collect();
        assert_eq!(v, ["one", "two", "other", "more"]);
    }

    #[test]
    fn test_move_subtask() {
        let mut tf = get_test_task_file();
        tf.add_task("other", "top", "1");
        tf.move_task("1.1", "1.3");

        let sv0: Vec<&str> = tf.tasks[0]
            .subtasks
            .iter()
            .map(|t| t.contents.as_str())
            .collect();
        assert_eq!(sv0, ["one", "two", "other"]);

        tf.move_task("1.3", "1.1");
        let sv0: Vec<&str> = tf.tasks[0]
            .subtasks
            .iter()
            .map(|t| t.contents.as_str())
            .collect();
        assert_eq!(sv0, ["other", "one", "two"]);
    }

    #[test]
    fn test_move_task_to_subtask_vice_versa() {
        let mut tf = get_test_task_file();
        tf.add_task("other", "top", "1");
        tf.move_task("1.1", "3");

        assert_eq!(tf.tasks[0].contents, "one");
        assert_eq!(tf.tasks[0].subtasks[0].contents, "one");
        assert_eq!(tf.tasks[0].subtasks[1].contents, "two");
        assert_eq!(tf.tasks[1].contents, "two");
        assert_eq!(tf.tasks[2].contents, "other");

        tf.move_task("1", "3.1");
        assert_eq!(tf.tasks[0].contents, "two");
        assert_eq!(tf.tasks[1].contents, "other");
        assert_eq!(tf.tasks[1].subtasks[0].contents, "one");

        tf.move_task("2", "1.1");
        assert_eq!(tf.tasks[0].contents, "two");
        assert_eq!(tf.tasks[0].subtasks[0].contents, "other");
        assert_eq!(tf.tasks[0].subtasks[1].contents, "one");
        assert_eq!(tf.tasks[0].subtasks[2].contents, "two");
    }

    // SWAPPING TASKS
    #[test]
    fn test_swap_tasks() {
        let mut tf = get_test_task_file();
        tf.add_task("other", "top", "");
        tf.swap_tasks("1", "3");

        let v: Vec<&str> = tf.tasks.iter().map(|t| t.contents.as_str()).collect();
        assert_eq!(v, ["two", "one", "other"]);
    }

    #[test]
    fn test_swap_subtasks() {
        let mut tf = get_test_task_file();
        tf.add_task("other", "top", "1");
        tf.swap_tasks("1.1", "1.3");

        let sv0: Vec<&str> = tf.tasks[0]
            .subtasks
            .iter()
            .map(|t| t.contents.as_str())
            .collect();
        assert_eq!(sv0, ["two", "one", "other"]);
    }

    #[test]
    fn test_swap_task_subtask() {
        let mut tf = get_test_task_file();
        tf.add_task("other", "top", "");
        tf.swap_tasks("1", "2.2");

        assert_eq!(tf.tasks[0].contents, "two");
        assert_eq!(tf.tasks[1].subtasks[1].contents, "other");

        tf.swap_tasks("2.2", "1");
        assert_eq!(tf.tasks[0].contents, "other");
        assert_eq!(tf.tasks[1].subtasks[1].contents, "two");
    }

    // APPENDING TASK
    #[test]
    fn test_append_task() {
        let mut tf = get_test_task_file();
        tf.append_to_task("1", "new");
        tf.append_to_task("1.1", "new");

        assert_eq!(tf.tasks[0].contents, "one new");
        assert_eq!(tf.tasks[0].subtasks[0].contents, "one new");
    }

    #[test]
    fn test_append_task_undo_done() {
        let mut tf = get_done_test_task_file();
        tf.append_to_task("1", "new");
        assert!(!tf.tasks[0].done);

        tf = get_done_test_task_file();
        tf.append_to_task("1.1", "new");

        assert!(!tf.tasks[0].done);
        assert!(!tf.tasks[0].subtasks[0].done);
    }

    // EDITING TASK
    #[test]
    fn test_edit_task() {
        let mut tf = get_test_task_file();
        tf.edit_task("1", "new");
        tf.edit_task("1.1", "newer");

        assert_eq!(tf.tasks[0].contents, "new");
        assert_eq!(tf.tasks[0].subtasks[0].contents, "newer");
    }

    #[test]
    fn test_edit_task_undo_done() {
        let mut tf = get_done_test_task_file();
        tf.edit_task("1", "new");
        assert!(!tf.tasks[0].done);

        tf = get_done_test_task_file();
        tf.edit_task("1.1", "new");

        assert!(!tf.tasks[0].done);
        assert!(!tf.tasks[0].subtasks[0].done);
    }

    // DELETING TASK
    #[test]
    fn test_delete_task() {
        let mut tf = get_test_task_file();
        tf.delete_task("2.2");
        tf.delete_task("1");

        assert_eq!(tf.tasks.len(), 1);
        assert_eq!(tf.tasks[0].subtasks.len(), 1)
    }

    #[test]
    fn test_clear_dones() {
        let mut tf = get_test_task_file();
        tf.tasks[1].subtasks[0].done = true;
        tf.tasks[0].done = true;
        tf.clear_dones();

        assert_eq!(tf.tasks.len(), 1);
        assert_eq!(tf.tasks[0].subtasks.len(), 1);
    }
}
