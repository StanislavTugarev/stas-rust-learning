#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

struct TaskList {
    tasks: Vec<Task>,
}

impl Task {
    fn new(id: u32, title: String, done: bool) -> Self {
        Task { id, title, done }
    }
}

impl TaskList {
    fn new() -> Self {
        TaskList { tasks: vec![] }
    }

    fn add(&mut self, task: Task) -> &mut Self {
        self.tasks.push(task);
        self
    }

    fn complete(&mut self, id: u32) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|x| x.id == id) {
            if !task.done {
                task.done = true;
                println!("Task {} completed", task.id);
                Ok(())
            } else {
                Err("Already marked as done".to_string())
            }
        } else {
            Err("Error, no task with this id".to_string())
        }
    }

    fn pending(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|x| !x.done).collect()
    }

    fn find(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|x| x.id == id)
    }
}

fn main() {
    let mut task_list = TaskList::new();
    task_list
        .add(Task::new(1, "first".to_string(), false))
        .add(Task::new(2, "second".to_string(), false));
    let _ = task_list.complete(1);

    let pending_tasks = task_list.pending();
    println!("Pending tasks: {:?}", pending_tasks);

    let find_task = task_list.find(1);
    match find_task {
        Some(find_task) => println!("task is {}", find_task.title),
        None => println!("no task with this id"),
    }
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, vec};

use super::*;

    #[test]
    fn create_task_and_list() {
        let task = Task::new(1, "first".to_string(), false);
        assert_eq!(task.id, 1);

        let mut task_list = TaskList::new();
        task_list.add(task);
        assert_eq!(task_list.tasks[0].id, 1);
    }

    #[test]
    fn complete_the_task() {
        let task = Task::new(1, "first".to_string(), false);

        let mut task_list = TaskList::new();
        task_list.add(task);
        assert_eq!(task_list.tasks[0].done, false);

        let res = task_list.complete(1);
        assert_eq!(res, Ok(()));
        assert_eq!(task_list.tasks[0].done, true);

        let res = task_list.complete(1);
        assert_eq!(res, Err("Already marked as done".to_string()));

        let res = task_list.complete(2);
        assert_eq!(res, Err("Error, no task with this id".to_string()));
    }

    #[test]
    fn pending_tasks() {
        let task_1 = Task::new(1, "first".to_string(), false);
        let task_2 = Task::new(2, "second".to_string(), false);

        let mut task_list = TaskList::new();
        task_list.add(task_1).add(task_2);

        let res = task_list.pending();
        assert_eq!(res[0].id, 1);

        let _ = task_list.complete(1);
        let res = task_list.pending();
        assert_eq!(res[0].id, 2);    
    }

    #[test]
    fn find_task() {
        let task_1 = Task::new(1, "first".to_string(), false);
        let task_2 = Task::new(2, "second".to_string(), false);

        let mut task_list = TaskList::new();
        task_list.add(task_1).add(task_2);

        let res = task_list.find(1);
        assert!(res.is_some());

        let res = task_list.find(3);
        assert!(res.is_none());
    }
}