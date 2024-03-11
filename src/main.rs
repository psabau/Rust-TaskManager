extern crate untitled;
use untitled::TaskList;

use std::io::{self};
use std::path::Path;

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
