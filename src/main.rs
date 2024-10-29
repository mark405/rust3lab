use serde::{Serialize, Deserialize};
use serde_json::{self, Result as SerdeResult};
use std::fs::File;
use std::io::{self, Write, Read};
use std::path::Path;
use uuid::Uuid;

// Define a custom error type
#[derive(Debug)]
enum TaskManagerError {
    IoError(io::Error),
    SerdeError(serde_json::Error),
}

impl From<io::Error> for TaskManagerError {
    fn from(err: io::Error) -> TaskManagerError {
        TaskManagerError::IoError(err)
    }
}

impl From<serde_json::Error> for TaskManagerError {
    fn from(err: serde_json::Error) -> TaskManagerError {
        TaskManagerError::SerdeError(err)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: String,
    description: String,
    completed: bool,
}

struct TaskManager {
    tasks: Vec<Task>,
    filename: String,
}

impl TaskManager {
    // Create a new TaskManager
    fn new(filename: &str) -> Self {
        TaskManager {
            tasks: Vec::new(),
            filename: filename.to_string(),
        }
    }

    // Load tasks from the JSON file
    fn load(&mut self) -> Result<(), TaskManagerError> {
        if Path::new(&self.filename).exists() {
            let file = File::open(&self.filename)?;
            self.tasks = serde_json::from_reader(file)?;
        }
        Ok(())
    }

    // Save tasks to the JSON file
    fn save(&self) -> Result<(), TaskManagerError> {
        let file = File::create(&self.filename)?;
        serde_json::to_writer(file, &self.tasks)?;
        Ok(())
    }

    // Add a new task
    fn add_task(&mut self, description: &str) {
        let task = Task {
            id: Uuid::new_v4().to_string(),
            description: description.to_string(),
            completed: false,
        };
        self.tasks.push(task);
    }

    // Edit an existing task
    fn edit_task(&mut self, id: &str, new_description: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.description = new_description.to_string();
        }
    }

    // Delete a task
    fn delete_task(&mut self, id: &str) {
        self.tasks.retain(|t| t.id != id);
    }

    // Mark a task as completed
    fn complete_task(&mut self, id: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
        }
    }

    // Display all tasks
    fn display_tasks(&self) {
        for task in &self.tasks {
            println!("ID: {}, Description: {}, Completed: {}", task.id, task.description, task.completed);
        }
    }
}

fn main() -> Result<(), TaskManagerError> {
    let mut task_manager = TaskManager::new("tasks.json");

    // Load existing tasks
    if let Err(e) = task_manager.load() {
        eprintln!("Error loading tasks: {:?}", e);
    }

    // Example usage
    task_manager.add_task("Buy groceries");
    task_manager.add_task("Walk the dog");

    // Display current tasks
    println!("Current Tasks:");
    task_manager.display_tasks();

    // Get the ID of the first task
    let first_task_id = task_manager.tasks[0].id.clone();
    task_manager.complete_task(&first_task_id); // Call complete_task with the ID

    // Delete the second task
    let second_task_id = task_manager.tasks[1].id.clone();
    task_manager.delete_task(&second_task_id); // Call delete_task with the ID

    // Display tasks after deletion
    println!("Tasks after deletion:");
    task_manager.display_tasks();

    // Save tasks
    if let Err(e) = task_manager.save() {
        eprintln!("Error saving tasks: {:?}", e);
    }

    Ok(())
}
