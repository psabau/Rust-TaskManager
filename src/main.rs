use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::io::{self};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct TaskList {
    items: Vec<Task>,
    next_id: usize,
}

impl TaskList {
    fn new() -> TaskList {
        TaskList {
            items: Vec::new(),
            next_id: 1,
        }
    }

    fn add_item(&mut self, action: String) {
        let item = Task {
            id: self.next_id,
            action,
            done: false,
        }; //OOF!!!
        self.items.push(item);
        self.next_id += 1;
    }

    fn complete_task(&mut self, id: usize) {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.done = true;
        }
    }

    fn save_to_file(&self, path: &Path) -> io::Result<()> {
        let serialized = serde_json::to_string(&self)?;
        fs::write(path, serialized)?;
        Ok(())
    }

    fn load_from_file(path: &Path) -> io::Result<TaskList> {
        let file_content = fs::read_to_string(path)?;
        let task_list: TaskList = serde_json::from_str(&file_content)?;
        Ok(task_list)
    }

    //move task to the position I want
    fn move_task(&mut self, id: usize, new_position: usize) {
        if let Some(index) = self.items.iter().position(|item| item.id == id) {
            //if new pos is out of bound, move at the end
            let new_position = new_position.min(self.items.len() - 1);

            let task = self.items.remove(index);

            self.items.insert(new_position, task);

            self.update_ids();
        }
    }

    fn update_ids(&mut self) {
        for (new_id, item) in self.items.iter_mut().enumerate() {
            item.id = (new_id as usize) + 1;
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    action: String,
    done: bool,
}

fn main() {
    let path = Path::new("todo_list.json");
    let mut task_list = match TaskList::load_from_file(path) {
        Ok(list) => list,
        Err(_) => TaskList::new(),
    };

    println!("Todo List CLI");
    println!("Type 'help' to see all available commands\n");

    loop {
        println!("Enter command:");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let parts: Vec<&str> = command.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["add", action] => {
                let action = action.to_string();
                task_list.add_item(action);
                println!("Task added.");
            }
            ["complete", id] => {
                let id: usize = id.parse().expect("Please provide a valid ID");
                task_list.complete_task(id);
                println!("Task {} marked as completed.", id);
            }
            ["list"] => {
                if task_list.items.is_empty() {
                    println!("The task list is empty.");
                } else {
                    for item in &task_list.items {
                        println!(
                            "{}: {} - {}",
                            item.id,
                            if item.done { "Done" } else { "Not done" },
                            item.action
                        );
                    }
                }
            }
            ["save"] => match task_list.save_to_file(path) {
                Ok(_) => println!("List saved."),
                Err(e) => println!("Error saving list: {:?}", e),
            },
            ["help"] => {
                println!("Available commands:");
                println!("add <task> - Add a new task");
                println!("complete <id> - Mark a task as completed by its ID");
                println!("move <id of task to move> <id where you want to move it to> - move task to a different position in the list");
                println!("list - List all tasks");
                println!("save - Save the current task list to file");
                println!("exit - Exit the program");
            }
            ["exit"] => {
                println!("Exiting...");
                break;
            }

            ["move", id, new_position] => {
                let id: usize = id.parse().expect("Please provide a valid ID");
                let new_position: usize = new_position
                    .parse()
                    .expect("Please provide a valid position");
                task_list.move_task(id, new_position);
                println!("Item {} moved to position {}.", id, new_position);
            }

            _ => println!("Unknown command, type 'help' for a list of commands."),
        }
    }
}
