#[cfg(test)]
mod tests {
    extern crate untitled;
    use untitled::TaskList;
    #[test]
    fn test_new_task_list_is_empty() {
        let task_list = TaskList::new();
        assert!(task_list.items.is_empty());
        assert_eq!(task_list.next_id, 1);
    }

    #[test]
    fn test_add_item() {
        let mut task_list = TaskList::new();
        task_list.add_item("Learn Rust".to_string());
        assert_eq!(task_list.items.len(), 1);
        assert_eq!(task_list.items[0].action, "Learn Rust");
        assert_eq!(task_list.items[0].done, false);
    }

    #[test]
    fn test_complete_task() {
        let mut task_list = TaskList::new();
        task_list.add_item("Learn Rust".to_string());
        task_list.complete_task(1);
        assert!(task_list.items[0].done);
    }

    #[test]
    fn test_move_task() {
        let mut task_list = TaskList::new();
        task_list.add_item("Task 1".to_string());
        task_list.add_item("Task 2".to_string());
        task_list.move_task(1, 1); // Should move Task 1 to the position of Task 2.
        assert_eq!(task_list.items[1].action, "Task 1");
    }
}
