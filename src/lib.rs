use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::io::{self};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub items: Vec<Task>,
    pub next_id: usize,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            items: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_item(&mut self, action: String) {
        let item = Task {
            id: self.next_id,
            action,
            done: false,
        }; //OOF!!!
        self.items.push(item);
        self.next_id += 1;
    }

    pub fn complete_task(&mut self, id: usize) {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.done = true;
        }
    }

    pub fn save_to_file(&self, path: &Path) -> io::Result<()> {
        let serialized = serde_json::to_string(&self)?;
        fs::write(path, serialized)?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> io::Result<TaskList> {
        let file_content = fs::read_to_string(path)?;
        let task_list: TaskList = serde_json::from_str(&file_content)?;
        Ok(task_list)
    }

    //move task to the position I want
    pub fn move_task(&mut self, id: usize, new_position: usize) {
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
pub struct Task {
    pub id: usize,
    pub action: String,
    pub done: bool,
}
