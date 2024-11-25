use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct ToDo {
    tasks: HashMap<u32, (String, bool)>, // Map of task ID to (task description, completion status)
    next_id: u32,                        // Counter for the next unique task ID
}

impl ToDo {
    // Create a new ToDo instance
    fn new() -> ToDo {
        ToDo {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    // Add a new task
    fn add_task(&mut self, task: String) {
        self.tasks.insert(self.next_id, (task, false)); // Insert task with status set to false (incomplete)
        println!("Task added with ID: {}", self.next_id);
        self.next_id += 1; // Increment the task ID counter
    }

    // List all tasks
    fn list_tasks(&self) {
        println!("\nTo-Do List:");
        for (id, (task, done)) in &self.tasks {
            let status = if *done { "✔" } else { "❌" }; // Display task status as a symbol
            println!("{}: [{}] {}", id, status, task);
        }
    }

    // Mark a task as complete
    fn complete_task(&mut self, task_id: u32) {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.1 = true; // Set task status to true (complete)
            println!("Task {} marked as complete!", task_id);
        } else {
            println!("Task ID {} not found!", task_id);
        }
    }
}

fn main() {
    let mut todo = ToDo::new(); // Initialize a new ToDo instance
    loop {
        // Print menu options
        println!("\nWhat would you like to do?");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Exit");

        // Read user input
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        // Match user choice to corresponding action
        match choice.trim() {
            "1" => {
                println!("Enter the task description:");
                let mut task = String::new();
                io::stdin()
                    .read_line(&mut task)
                    .expect("Failed to read input");
                todo.add_task(task.trim().to_string()); // Add task to ToDo list
            }
            "2" => todo.list_tasks(), // List all tasks
            "3" => {
                println!("Enter the task ID to mark as complete:");
                let mut task_id = String::new();
                io::stdin()
                    .read_line(&mut task_id)
                    .expect("Failed to read input");
                if let Ok(id) = task_id.trim().parse::<u32>() {
                    todo.complete_task(id); // Mark task as complete
                } else {
                    println!("Invalid task ID!");
                }
            }
            "4" => {
                println!("Goodbye!");
                break; // Exit the loop
            }
            _ => println!("Invalid option! Please choose 1, 2, 3, or 4."),
        }
    }
}
