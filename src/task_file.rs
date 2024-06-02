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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    // {{{
    contents: String,
    done: bool,
    pub subtasks: Vec<Task>,
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

    fn get_subtask_id(v: &str) -> Vec<usize> {
        v.split('.').map(|i| i.parse().unwrap()).collect()
    }

    pub fn print(&self, colored: bool) {
        // {{{
        fn _print(tasks: &[Task], colored: bool, t_id: &str, depth: usize) {
            // {{{
            let t_id = t_id.trim();

            for (id, t) in tasks.iter().enumerate() {
                let mut id = {
                    if depth == 0 {
                        format!("{:>2}", id + 1)
                    } else {
                        format!("{t_id}{}", id + 1)
                    }
                };
                for _i in 0..depth {
                    id.insert_str(0, "     ");
                }

                if t.done && colored {
                    println!("{id}. \x1b[0;32m[X] {} \x1b[0m", t.contents);
                } else if colored {
                    println!("{id}. \x1b[0;31m[ ] {} \x1b[0m", t.contents);
                } else if t.done {
                    println!("{id}. [X] {}", t.contents);
                } else {
                    println!("{id}. [ ] {}", t.contents);
                }

                if t.subtasks.len() != 0 {
                    _print(&t.subtasks, colored, format!("{id}.").as_str(), depth + 1)
                }
            }
        }
        // }}}

        if self.tasks.len() == 0 {
            println!("No tasks to print");
            return;
        }
        println!("Tasks:\n");

        _print(&self.tasks, colored, "", 0)
    }
    // }}}

    pub fn add_task(&mut self, task: &str, pos: &AddPosition, id: &str) {
        // {{{
        fn _add(tasks: &mut [Task], task: Task, pos: &AddPosition, s_id: &[usize], depth: usize) {
            // {{{
            for (id, t) in tasks.iter_mut().enumerate() {
                if id + 1 == s_id[depth] && s_id.len() - 1 == depth {
                    if pos == &AddPosition::Top {
                        t.subtasks.insert(0, task)
                    } else {
                        t.subtasks.push(task)
                    }
                    t.done = false;
                    return;
                } else if id + 1 == s_id[depth] {
                    _add(&mut t.subtasks, task, pos, s_id, depth + 1);
                    t.done = false;
                    return;
                }
            }
        }
        // }}}

        println!("Adding task...");
        let task = Task {
            contents: task.to_owned(),
            done: false,
            subtasks: Vec::new(),
        };

        if id.is_empty() {
            if pos == &AddPosition::Top {
                self.tasks.insert(0, task)
            } else {
                self.tasks.push(task)
            }
            return;
        }

        let s_id = Self::get_subtask_id(&id);
        _add(&mut self.tasks, task, pos, &s_id, 0);
        return;
    }
    // }}}

    pub fn mark_tasks(&mut self, ids: &[String], done: bool) {
        // {{{
        fn _mark(tasks: &mut [Task], done: bool, p_id: &[usize], ids: &[usize], depth: usize) {
            // {{{
            for (id, t) in tasks.iter_mut().enumerate() {
                if id + 1 == p_id[depth] && p_id.len() - 1 == depth {
                    mark(&mut t.subtasks, done, ids, false);
                    t.done = t.subtasks.iter().find(|t| !t.done).is_none();
                    return;
                } else if id + 1 == p_id[depth] {
                    _mark(&mut t.subtasks, done, p_id, ids, depth + 1);
                    t.done = t.subtasks.iter().find(|t| !t.done).is_none();
                    return;
                }
            }
        }
        // }}}

        // Another recursive function is necessary in case the task being marked as done has
        // subtasks, or in case the user passes a range/list of sub-ids.
        fn mark(tasks: &mut [Task], done: bool, ids: &[usize], all: bool) {
            // {{{
            for (id, t) in tasks.iter_mut().enumerate() {
                if ids.contains(&(id + 1)) || all {
                    t.done = done;
                    if t.subtasks.len() != 0 {
                        mark(&mut t.subtasks, done, ids, true)
                    }
                }
            }
        }
        // }}}

        if done {
            println!("Marking tasks as done...");
        } else {
            println!("Unmarking tasks...");
        }
        if ids[0].contains('.') {
            if ids.len() > 1 {
                let p_id: Vec<usize> = ids[0]
                    .split_terminator('.')
                    .map(|i| i.parse().unwrap())
                    .collect();
                let s_ids: Vec<usize> = ids.iter().skip(1).map(|i| i.parse().unwrap()).collect();
                _mark(&mut self.tasks, done, &p_id, &s_ids, 0);
                return;
            }
            let ids: Vec<usize> = ids[0]
                .split('.')
                .map(|i| i.parse().unwrap())
                .collect();
            let len = ids.len();
            let p_id = &ids[..len -1];
            let s_id = &ids[len -1..];
        
            _mark(&mut self.tasks, done, &p_id, &s_id, 0);
            return;
        }

        let ids: Vec<usize> = ids.iter().map(|i| i.parse().unwrap()).collect();
        mark(&mut self.tasks, done, &ids, false)
    }
    // }}}

    pub fn move_task(&mut self, from: &str, to: &str) {
        // {{{
        fn _get(tasks: &mut [Task], s_id: &[usize], depth: usize) -> Task {
            // {{{
            for (id, t) in tasks.iter_mut().enumerate() {
                if id + 1 == s_id[depth] && s_id.len() - 1 == depth + 1 {
                    return t.subtasks.get(s_id[depth + 1]).unwrap().clone();
                } else if id + 1 == s_id[depth] {
                    return _get(&mut t.subtasks, s_id, depth + 1);
                }
            }
            panic!("This should never happen");
        }
        // }}}

        fn _put(tasks: &mut [Task], task: Task, s_id: &[usize], depth: usize) {
            // {{{
            for (id, t) in tasks.iter_mut().enumerate() {
                if id + 1 == s_id[depth] && s_id.len() - 1 == depth + 1 {
                    t.subtasks.insert(s_id[depth + 1], task);
                    return;
                } else if id + 1 == s_id[depth] {
                    _put(&mut t.subtasks, task, s_id, depth + 1);
                    return;
                }
            }
        }
        // }}}

        fn _remove(tasks: &mut [Task], s_id: &[usize], depth: usize) {
            // {{{
            for (id, t) in tasks.iter_mut().enumerate() {
                if id + 1 == s_id[depth] && s_id.len() - 1 == depth + 1 {
                    t.subtasks.remove(s_id[depth + 1]);
                    return;
                } else if id + 1 == s_id[depth] {
                    _remove(&mut t.subtasks, s_id, depth + 1);
                    return;
                }
            }
            panic!("This should never happen");
        }
        // }}}

        println!("Moving task...");
        let (from_id, from_sub_id, to_id, to_sub_id) = {
            // Initializing important variables {{{
            let (fid, fsid) = if from.contains('.') {
                let mut sub_id = Self::get_subtask_id(from);
                let last = sub_id.len() - 1;
                sub_id[last] -= 1;
                (0, sub_id)
            } else {
                let from: usize = from.parse().unwrap();
                (from - 1, Vec::new())
            };

            let (tid, tsid) = if to.contains('.') {
                let mut sub_id = Self::get_subtask_id(to);
                let last = sub_id.len() - 1;
                sub_id[last] -= 1;
                (0, sub_id)
            } else {
                let to: usize = to.parse().unwrap();
                (to - 1, Vec::new())
            };

            (fid, fsid, tid, tsid)
        };
        // }}}

        let task = if from.contains('.') {
            _get(&mut self.tasks, &from_sub_id, 0)
        } else {
            self.tasks.get(from_id).unwrap().clone()
        };

        if to.contains('.') {
            if from.contains('.')
                && from_sub_id[..from_sub_id.len() - 2] == to_sub_id[..to_sub_id.len() - 2]
            {
                _remove(&mut self.tasks, &from_sub_id, 0);
                _put(&mut self.tasks, task, &to_sub_id, 0);
                return;
            }
            _put(&mut self.tasks, task, &to_sub_id, 0);
        } else {
            if !from.contains('.') {
                self.tasks.remove(from_id);
                self.tasks.insert(to_id, task);
                return;
            }
            self.tasks.insert(to_id, task);
        }

        if !from.contains('.') {
            self.tasks.remove(from_id);
            return;
        }
        _remove(&mut self.tasks, &from_sub_id, 0)
    }
    // }}}

    pub fn swap_tasks(&mut self, t1: &str, t2: &str) {
        // {{{
        fn _get(tasks: &[Task], s_id: &[usize], depth: usize) -> Task {
            // {{{
            for (id, t) in tasks.iter().enumerate() {
                if id + 1 == s_id[depth] && s_id.len() - 1 == depth {
                    return t.clone();
                } else if id + 1 == s_id[depth] {
                    return _get(&t.subtasks, s_id, depth + 1);
                }
            }
            panic!("This shouldn't happen")
        }
        // }}}

        fn _swap(tasks: &mut [Task], task: Task, s_id: &[usize], depth: usize) {
            // {{{
            for (id, t) in tasks.iter_mut().enumerate() {
                if id + 1 == s_id[depth] && s_id.len() - 1 == depth {
                    *t = task;
                    return;
                } else if id + 1 == s_id[depth] {
                    _swap(&mut t.subtasks, task, s_id, depth + 1);
                    return;
                }
            }
            panic!("This shouldn't happen")
        }
        // }}}

        println!("Swapping tasks...");

        if !t1.contains('.') && !t2.contains('.') {
            let t1: usize = t1.parse().unwrap();
            let t2: usize = t2.parse().unwrap();
            self.tasks.swap(t1 - 1, t2 - 1);
            return;
        }

        let (t1, id1) = if t1.contains('.') {
            let s_id = Self::get_subtask_id(t1);
            (_get(&self.tasks, &s_id, 0), s_id)
        } else {
            let id: usize = t1.parse().unwrap();
            (self.tasks[id - 1].clone(), vec![id])
        };

        let (t2, id2) = if t2.contains('.') {
            let s_id = Self::get_subtask_id(t2);
            (_get(&self.tasks, &s_id, 0), s_id)
        } else {
            let id: usize = t2.parse().unwrap();
            (self.tasks[id - 1].clone(), vec![id])
        };
        _swap(&mut self.tasks, t2, &id1, 0);
        _swap(&mut self.tasks, t1, &id2, 0)
    }
    // }}}

    pub fn append_to_task(&mut self, id: &str, content: &str) {
        // {{{
        fn _append(tasks: &mut [Task], content: &str, s_id: &[usize], depth: usize) {
            // {{{
            for (id, t) in tasks.iter_mut().enumerate() {
                if id + 1 == s_id[depth] && s_id.len() - 1 == depth {
                    t.contents.push_str(content);
                    t.done = false;
                    return;
                } else if id + 1 == s_id[depth] {
                    _append(&mut t.subtasks, content, s_id, depth + 1);
                    t.done = false;
                    return;
                }
            }
        }
        // }}}

        println!("Appending content...");
        let content = format!(" {content}");
        if !id.contains('.') {
            let id: usize = id.parse().unwrap();
            self.tasks[id - 1].contents.push_str(&content);
            self.tasks[id - 1].done = false;
            return;
        }

        let s_id = Self::get_subtask_id(id);
        _append(&mut self.tasks, &content, &s_id, 0)
    }
    // }}}

    pub fn edit_task(&mut self, id: &str, new_content: &str) {
        // {{{
        fn _edit(tasks: &mut [Task], new_content: &str, s_id: &[usize], depth: usize) {
            // {{{
            for (id, t) in tasks.iter_mut().enumerate() {
                if id + 1 == s_id[depth] && s_id.len() - 1 == depth {
                    t.contents = new_content.to_owned();
                    t.done = false;
                    return;
                } else if id + 1 == s_id[depth] {
                    _edit(&mut t.subtasks, new_content, s_id, depth + 1);
                    t.done = false;
                    return;
                }
            }
        }
        // }}}

        println!("Editing task...");
        if !id.contains('.') {
            let id: usize = id.parse().unwrap();
            self.tasks[id - 1].contents = new_content.to_owned();
            self.tasks[id - 1].done = false;
            return;
        }

        let s_id = Self::get_subtask_id(id);
        _edit(&mut self.tasks, new_content, &s_id, 0)
    }
    // }}}

    pub fn delete_task(&mut self, id: &str) {
        // {{{
        fn _delete(tasks: &mut [Task], s_id: &[usize], depth: usize) {
            // {{{
            for (id, t) in tasks.iter_mut().enumerate() {
                if id + 1 == s_id[depth] && s_id.len() - 1 == depth + 1 {
                    t.subtasks.remove(s_id[depth + 1] - 1);
                    return;
                } else if id + 1 == s_id[depth] {
                    _delete(&mut t.subtasks, s_id, depth + 1);
                    return;
                }
            }
        }
        // }}}

        println!("Deleting task...");
        if !id.contains('.') {
            let id: usize = id.parse().unwrap();
            self.tasks.remove(id - 1);
            return;
        }
        let s_id = Self::get_subtask_id(id);
        _delete(&mut self.tasks, &s_id, 0)
    }
    // }}}

    pub fn clear_dones(&mut self) {
        // {{{
        fn _clear(tasks: &mut [Task]) {
            // {{{
            for t in tasks.iter_mut() {
                if t.subtasks.len() != 0 {
                    t.subtasks = t
                        .subtasks
                        .iter()
                        .filter(|t| !t.done)
                        .map(|t| t.to_owned())
                        .collect();

                    if t.subtasks.len() != 0 {
                        _clear(&mut t.subtasks)
                    }
                }
            }
        }
        // }}}

        println!("Clearing done tasks...");
        self.tasks = self
            .tasks
            .iter()
            .filter(|t| !t.done)
            .map(|t| t.to_owned())
            .collect();

        if self.tasks.len() != 0 {
            _clear(&mut self.tasks);
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
                    subtasks: vec![
                        Task {
                            contents: "one".to_owned(),
                            done: false,
                            subtasks: Vec::new(),
                        },
                        Task {
                            contents: "two".to_owned(),
                            done: false,
                            subtasks: Vec::new(),
                        },
                    ],
                },
                Task {
                    contents: "two".to_owned(),
                    done: false,
                    subtasks: vec![
                        Task {
                            contents: "one".to_owned(),
                            done: false,
                            subtasks: Vec::new(),
                        },
                        Task {
                            contents: "two".to_owned(),
                            done: false,
                            subtasks: Vec::new(),
                        },
                    ],
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
                    subtasks: vec![
                        Task {
                            contents: "one".to_owned(),
                            done: true,
                            subtasks: Vec::new(),
                        },
                        Task {
                            contents: "two".to_owned(),
                            done: true,
                            subtasks: Vec::new(),
                        },
                    ],
                },
                Task {
                    contents: "two".to_owned(),
                    done: true,
                    subtasks: vec![
                        Task {
                            contents: "one".to_owned(),
                            done: true,
                            subtasks: Vec::new(),
                        },
                        Task {
                            contents: "two".to_owned(),
                            done: true,
                            subtasks: Vec::new(),
                        },
                    ],
                },
            ],
        }
    }
    // }}}

    #[test]
    #[ignore]
    fn test_print() {
        // {{{
        let mut tf = get_test_task_file();
        tf.tasks[0].subtasks.push(Task {
            contents: "sub".to_owned(),
            done: false,
            subtasks: vec![Task {
                contents: "sub".to_owned(),
                done: false,
                subtasks: Vec::new(),
            }],
        });
        tf.print(true);
    }
    // }}}

    // // Adding tasks {{{
    #[test]
    fn test_add_task_top() {
        // {{{
        let mut tf = get_test_task_file();
        tf.add_task("three", &AddPosition::Top, "");

        assert_eq!(tf.tasks[0].contents, "three");
        assert_eq!(tf.tasks[1].contents, "one");
        assert_eq!(tf.tasks[2].contents, "two")
    }
    // }}}

    #[test]
    fn test_add_task_bot() {
        // {{{
        let mut tf = get_test_task_file();
        tf.add_task("three", &AddPosition::Bottom, "");

        assert_eq!(tf.tasks[0].contents, "one");
        assert_eq!(tf.tasks[1].contents, "two");
        assert_eq!(tf.tasks[2].contents, "three");
    }
    // }}}

    #[test]
    fn test_add_sub_top() {
        // {{{
        let mut tf = get_test_task_file();
        tf.add_task("sub", &AddPosition::Top, "1");

        assert_eq!(tf.tasks[0].contents, "one");
        assert_eq!(tf.tasks[0].subtasks[0].contents, "sub");
        assert_eq!(tf.tasks[0].subtasks[1].contents, "one");
        assert_eq!(tf.tasks[0].subtasks[2].contents, "two");
        assert_eq!(tf.tasks[1].contents, "two");
    }
    // }}}

    #[test]
    fn test_add_sub_bot() {
        // {{{
        let mut tf = get_test_task_file();
        tf.add_task("sub3", &AddPosition::Bottom, "1");

        assert_eq!(tf.tasks[0].contents, "one");
        assert_eq!(tf.tasks[0].subtasks[0].contents, "one");
        assert_eq!(tf.tasks[0].subtasks[1].contents, "two");
        assert_eq!(tf.tasks[0].subtasks[2].contents, "sub3");
        assert_eq!(tf.tasks[1].contents, "two");
    }
    // }}}

    #[test]
    fn test_add_sub_sub_bot() {
        // {{{
        let mut tf = get_test_task_file();
        tf.add_task("subsub2", &AddPosition::Top, "1.1");
        tf.add_task("subsub1", &AddPosition::Top, "1.1");
        tf.add_task("subsub3", &AddPosition::Bottom, "1.1");

        assert_eq!(tf.tasks[0].contents, "one");
        assert_eq!(tf.tasks[0].subtasks[0].contents, "one");
        assert_eq!(tf.tasks[0].subtasks[0].subtasks[0].contents, "subsub1");
        assert_eq!(tf.tasks[0].subtasks[0].subtasks[1].contents, "subsub2");
        assert_eq!(tf.tasks[0].subtasks[0].subtasks[2].contents, "subsub3");
        assert_eq!(tf.tasks[1].contents, "two");
    }
    // }}}

    #[test]
    fn test_add_sub_undo_task() {
        // {{{
        let mut tf = get_test_done_task_file();
        tf.add_task("sub", &AddPosition::Top, "2");

        assert_eq!(tf.tasks[0].contents, "one");
        assert_eq!(tf.tasks[1].contents, "two");
        assert_eq!(tf.tasks[1].done, false);
        assert_eq!(tf.tasks[1].subtasks[0].contents, "sub");

        tf.mark_tasks(&["2".to_owned()], true);
        assert_eq!(tf.tasks[1].done, true);
        assert_eq!(tf.tasks[1].subtasks[0].done, true);

        tf.add_task("subsub", &AddPosition::Top, "2.1");
        assert_eq!(tf.tasks[1].done, false);
        assert_eq!(tf.tasks[1].subtasks[0].done, false);
        assert_eq!(tf.tasks[1].subtasks[0].subtasks[0].contents, "subsub");
    }
    // }}}
    // }}}

    // Marking tasks {{{
    #[test]
    fn test_mark_tasks_done() {
        // {{{
        let mut tf = get_test_task_file();
        tf.mark_tasks(&["1".to_owned()], true);

        assert!(tf.tasks[0].done);
        assert!(!tf.tasks[1].done);

        tf = get_test_task_file();
        tf.mark_tasks(&["1".to_owned(), "2".to_owned()], true);

        assert!(tf.tasks[0].done);
        assert!(tf.tasks[1].done);

        tf = get_test_done_task_file();
        tf.mark_tasks(&["1".to_owned()], false);

        assert!(!tf.tasks[0].done);
        assert!(tf.tasks[1].done);

        tf = get_test_task_file();
        tf.mark_tasks(&["1".to_owned(), "2".to_owned()], false);

        assert!(!tf.tasks[0].done);
        assert!(!tf.tasks[1].done);
    }
    // }}}

    #[test]
    fn test_mark_sub() {
        // {{{
        let mut tf = get_test_task_file();
        tf.mark_tasks(&["1.1".to_owned()], true);

        assert!(!tf.tasks[0].done);
        assert!(tf.tasks[0].subtasks[0].done);
        assert!(!tf.tasks[0].subtasks[1].done);
        assert!(!tf.tasks[1].done);

        tf = get_test_task_file();
        tf.mark_tasks(&["1.".to_owned(), "1".to_owned(), "2".to_owned()], true);

        assert!(tf.tasks[0].done);
        assert!(tf.tasks[0].subtasks[0].done);
        assert!(tf.tasks[0].subtasks[1].done);
        assert!(!tf.tasks[1].done);

        tf = get_test_done_task_file();
        tf.mark_tasks(&["1.1".to_owned()], false);
        tf.print(true);

        assert!(!tf.tasks[0].done);
        assert!(!tf.tasks[0].subtasks[0].done);
        assert!(tf.tasks[0].subtasks[1].done);
        assert!(tf.tasks[1].done);

        tf = get_test_done_task_file();
        tf.mark_tasks(&["1.".to_owned(), "1".to_owned(), "2".to_owned()], false);

        assert!(!tf.tasks[0].done);
        assert!(!tf.tasks[0].subtasks[0].done);
        assert!(!tf.tasks[0].subtasks[1].done);
        assert!(tf.tasks[1].done);
    }
    // }}}

    #[test]
    fn test_mark_parent_mark_child() {
        // {{{
        let mut tf = get_test_task_file();
        tf.add_task("subsub", &AddPosition::Top, "2.1");
        tf.mark_tasks(&["2".to_owned()], true);

        assert!(!tf.tasks[0].done);
        assert!(!tf.tasks[0].subtasks[0].done);
        assert!(!tf.tasks[0].subtasks[1].done);
        assert!(tf.tasks[1].done);
        assert!(tf.tasks[1].subtasks[0].done);
        assert!(tf.tasks[1].subtasks[0].subtasks[0].done);
        assert!(tf.tasks[1].subtasks[1].done);

        tf = get_test_done_task_file();
        tf.mark_tasks(&["2".to_owned()], false);

        assert!(tf.tasks[0].done);
        assert!(tf.tasks[0].subtasks[0].done);
        assert!(tf.tasks[0].subtasks[1].done);
        assert!(!tf.tasks[1].done);
        assert!(!tf.tasks[1].subtasks[0].done);
        assert!(!tf.tasks[1].subtasks[1].done);
    }
    // }}}

    #[test]
    fn test_mark_same() {
        // {{{
        let mut tf = get_test_done_task_file();
        tf.mark_tasks(&["1".to_owned(), "2".to_owned()], true);

        assert!(tf.tasks[0].done);
        assert!(tf.tasks[1].done);

        tf = get_test_task_file();
        tf.mark_tasks(&["1".to_owned(), "2".to_owned()], false);

        assert!(!tf.tasks[0].done);
        assert!(!tf.tasks[1].done);
    }
    // }}}
    // }}}

    // Moving tasks {{{
    #[test]
    fn test_move_task() {
        // {{{
        let mut tf = get_test_task_file();
        tf.add_task("other", &AddPosition::Bottom, "");
        tf.move_task("3", "1");

        assert_eq!(tf.tasks[0].contents, "other");
        assert_eq!(tf.tasks[1].contents, "one");
        assert_eq!(tf.tasks[2].contents, "two");

        tf.move_task("1", "3");

        assert_eq!(tf.tasks[0].contents, "one");
        assert_eq!(tf.tasks[1].contents, "two");
        assert_eq!(tf.tasks[2].contents, "other");

        assert_eq!(tf.tasks[0].subtasks.len(), 2);
        assert_eq!(tf.tasks[1].subtasks.len(), 2);
    }
    // }}}

    #[test]
    fn test_move_subtask() {
        // {{{
        let mut tf = get_test_task_file();
        tf.add_task("other", &AddPosition::Top, "1");
        tf.move_task("1.1", "1.3");

        assert_eq!(tf.tasks[0].contents, "one");
        assert_eq!(tf.tasks[0].subtasks[0].contents, "one");
        assert_eq!(tf.tasks[0].subtasks[1].contents, "two");
        assert_eq!(tf.tasks[0].subtasks[2].contents, "other");
        assert_eq!(tf.tasks[1].contents, "two");

        tf.move_task("1.3", "1.1");

        assert_eq!(tf.tasks[0].contents, "one");
        assert_eq!(tf.tasks[0].subtasks[0].contents, "other");
        assert_eq!(tf.tasks[0].subtasks[1].contents, "one");
        assert_eq!(tf.tasks[0].subtasks[2].contents, "two");
        assert_eq!(tf.tasks[1].contents, "two");
    }
    // }}}

    #[test]
    fn test_move_between_task_and_subtask() {
        // {{{
        let mut tf = get_test_task_file();
        tf.move_task("1.1", "3");

        assert_eq!(tf.tasks[0].contents, "one");
        assert_eq!(tf.tasks[0].subtasks[0].contents, "two");
        assert_eq!(tf.tasks[1].contents, "two");
        assert_eq!(tf.tasks[1].subtasks[0].contents, "one");
        assert_eq!(tf.tasks[1].subtasks[1].contents, "two");
        assert_eq!(tf.tasks[2].contents, "one");

        tf.move_task("1", "3.1");
        tf.print(true);

        assert_eq!(tf.tasks[0].contents, "two");
        assert_eq!(tf.tasks[0].subtasks[0].contents, "one");
        assert_eq!(tf.tasks[0].subtasks[1].contents, "two");
        assert_eq!(tf.tasks[1].contents, "one");
        assert_eq!(tf.tasks[1].subtasks[0].contents, "one");
        assert_eq!(tf.tasks[1].subtasks[0].subtasks[0].contents, "two");
    }
    // }}}
    // }}}

    // Swapping tasks {{{
    #[test]
    fn test_swap_tasks() {
        // {{{
        let mut tf = get_test_task_file();
        tf.add_task("other", &AddPosition::Top, "");
        tf.swap_tasks("1", "3");

        assert_eq!(tf.tasks[0].contents, "two");
        assert_eq!(tf.tasks[1].contents, "one");
        assert_eq!(tf.tasks[2].contents, "other");

        assert_eq!(tf.tasks[0].subtasks.len(), 2);
        assert_eq!(tf.tasks[1].subtasks.len(), 2);
    }
    // }}}

    #[test]
    fn test_swap_task_subtask() {
        // {{{
        let mut tf = get_test_task_file();
        tf.swap_tasks("1.2", "2");

        assert_eq!(tf.tasks[0].contents, "one");
        assert_eq!(tf.tasks[1].contents, "two");

        assert_eq!(tf.tasks[0].subtasks.len(), 2);
        assert_eq!(tf.tasks[0].subtasks[1].subtasks.len(), 2);
        assert_eq!(tf.tasks[1].subtasks.len(), 0);

        tf.swap_tasks("2", "1.2");

        assert_eq!(tf.tasks[0].subtasks.len(), 2);
        assert_eq!(tf.tasks[0].subtasks[1].subtasks.len(), 0);
        assert_eq!(tf.tasks[1].subtasks.len(), 2);
    }
    // }}}
    // }}}

    // Appending tasks {{{
    #[test]
    fn test_append_task() {
        // {{{
        let mut tf = get_test_task_file();
        tf.append_to_task("1", "new");
        tf.append_to_task("1.1", "new");

        assert_eq!(tf.tasks[0].contents, "one new");
        assert_eq!(tf.tasks[0].subtasks[0].contents, "one new");
    }
    // }}}

    #[test]
    fn test_append_task_undo_done() {
        // {{{
        let mut tf = get_test_done_task_file();
        tf.append_to_task("1", "new");

        assert!(!tf.tasks[0].done);

        tf = get_test_done_task_file();
        tf.append_to_task("1.1", "new");

        assert!(!tf.tasks[0].done);
        assert!(!tf.tasks[0].subtasks[0].done);
    }
    // }}}
    // }}}

    // Editing tasks {{{
    #[test]
    fn test_edit_task() {
        // {{{
        let mut tf = get_test_task_file();
        tf.edit_task("1", "new");
        tf.edit_task("1.1", "new");

        assert_eq!(tf.tasks[0].contents, "new");
        assert_eq!(tf.tasks[0].subtasks[0].contents, "new");
    }
    // }}}

    #[test]
    fn test_edit_task_undo_done() {
        // {{{
        let mut tf = get_test_done_task_file();
        tf.edit_task("1", "new");

        assert!(!tf.tasks[0].done);

        tf = get_test_done_task_file();
        tf.edit_task("1.1", "new");

        assert!(!tf.tasks[0].done);
        assert!(!tf.tasks[0].subtasks[0].done);
    }
    // }}}
    // }}}

    // Deleting tasks {{{
    #[test]
    fn test_delete_task() {
        // {{{
        let mut tf = get_test_task_file();
        tf.delete_task("2.2");
        tf.delete_task("1");

        assert_eq!(tf.tasks.len(), 1);
        assert_eq!(tf.tasks[0].subtasks.len(), 1)
    }
    // }}}

    #[test]
    fn test_clear_dones() {
        // {{{
        let mut tf = get_test_task_file();
        tf.tasks[0].done = true;
        tf.tasks[1].subtasks[0].done = true;
        tf.clear_dones();

        assert_eq!(tf.tasks.len(), 1);
        assert_eq!(tf.tasks[0].subtasks.len(), 1);
    }
    // }}}
    // }}}
}
// }}}
