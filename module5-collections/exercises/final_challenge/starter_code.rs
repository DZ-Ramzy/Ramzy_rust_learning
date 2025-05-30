// Starter code for the Rust Task Manager challenge

use std::fs;
use std::io::{self, Write};
use std::collections::HashMap;
use chrono::{DateTime, Local, NaiveDate};

// Task status enum
#[derive(Debug, Clone, PartialEq)]
enum TaskStatus {
    Pending,
    Completed,
}

// Task struct to store task information
#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    description: String,
    due_date: Option<NaiveDate>,  // Using chrono's NaiveDate for proper date handling
    status: TaskStatus,
}

// TaskManager to handle operations on tasks
struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    // Create a new TaskManager
    fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
    
    // Add a new task to the task manager
    fn add_task(&mut self, title: String, description: String, due_date: Option<String>) -> Result<&Task, String> {
        let parsed_due_date = if let Some(date_str) = due_date {
            NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .map_err(|e| format!("Invalid date format. Use YYYY-MM-DD: {}", e))?
                .into()
        } else {
            None
        };

        let task = Task {
            id: self.next_id,
            title,
            description,
            due_date: parsed_due_date,
            status: TaskStatus::Pending,
        };
        
        self.next_id += 1;
        self.tasks.push(task);
        Ok(self.tasks.last().unwrap())
    }

    // List all tasks
    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
            return;
        }

        for task in &self.tasks {
            println!("ID: {}", task.id);
            println!("Title: {}", task.title);
            println!("Description: {}", task.description);
            println!("Status: {:?}", task.status);
            if let Some(due_date) = task.due_date {
                println!("Due Date: {}", due_date.format("%Y-%m-%d"));
            }
            println!("-------------------");
        }
    }

    // Mark a task as complete
    fn complete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = TaskStatus::Completed;
            Ok(())
        } else {
            Err(format!("Task with ID {} not found", id))
        }
    }

    // Delete a task
    fn delete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            Ok(())
        } else {
            Err(format!("Task with ID {} not found", id))
        }
    }

    // Save tasks to a file
    fn save_tasks(&self, filename: &str) -> Result<(), String> {
        let mut content = String::new();
        for task in &self.tasks {
            content.push_str(&format!("{},{},{},{},{}\n",
                task.id,
                task.title,
                task.description,
                task.due_date.map_or("".to_string(), |d| d.format("%Y-%m-%d").to_string()),
                match task.status {
                    TaskStatus::Pending => "Pending",
                    TaskStatus::Completed => "Completed",
                }
            ));
        }
        fs::write(filename, content)
            .map_err(|e| format!("Failed to save tasks: {}", e))
    }

    // Load tasks from a file
    fn load_tasks(&mut self, filename: &str) -> Result<(), String> {
        let content = fs::read_to_string(filename)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        self.tasks.clear();
        for line in content.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 5 {
                let due_date = if parts[3].is_empty() {
                    None
                } else {
                    NaiveDate::parse_from_str(parts[3], "%Y-%m-%d")
                        .ok()
                };

                let task = Task {
                    id: parts[0].parse().unwrap_or(self.next_id),
                    title: parts[1].to_string(),
                    description: parts[2].to_string(),
                    due_date,
                    status: if parts[4] == "Completed" { TaskStatus::Completed } else { TaskStatus::Pending },
                };
                self.next_id = task.id + 1;
                self.tasks.push(task);
            }
        }
        Ok(())
    }

    // Filter tasks by status
    fn filter_by_status(&self, status: TaskStatus) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|task| task.status == status)
            .collect()
    }

    // Filter tasks by due date
    fn filter_by_due_date(&self, date: NaiveDate) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|task| task.due_date.map_or(false, |d| d == date))
            .collect()
    }
}

// Command enum to represent user commands
#[derive(Debug)]
enum Command {
    Add { title: String, description: String, due_date: Option<String> },
    List,
    Complete { id: u32 },
    Delete { id: u32 },
    Save { filename: String },
    Load { filename: String },
    FilterByDate { date: String },
    Quit,
    Unknown,
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.split_whitespace().collect();
    match parts.get(0) {
        Some(&"add") => {
            if parts.len() < 3 {
                return Command::Unknown;
            }
            let title = parts[1].to_string();
            let description = parts[2].to_string();
            let due_date = parts.get(3).map(|&s| s.to_string());
            Command::Add { title, description, due_date }
        }
        Some(&"list") => Command::List,
        Some(&"complete") => {
            if let Some(id_str) = parts.get(1) {
                if let Ok(id) = id_str.parse() {
                    return Command::Complete { id };
                }
            }
            Command::Unknown
        }
        Some(&"delete") => {
            if let Some(id_str) = parts.get(1) {
                if let Ok(id) = id_str.parse() {
                    return Command::Delete { id };
                }
            }
            Command::Unknown
        }
        Some(&"save") => {
            if let Some(filename) = parts.get(1) {
                return Command::Save { filename: filename.to_string() };
            }
            Command::Unknown
        }
        Some(&"load") => {
            if let Some(filename) = parts.get(1) {
                return Command::Load { filename: filename.to_string() };
            }
            Command::Unknown
        }
        Some(&"filter") => {
            if let Some(date_str) = parts.get(1) {
                return Command::FilterByDate { date: date_str.to_string() };
            }
            Command::Unknown
        }
        Some(&"quit") => Command::Quit,
        _ => Command::Unknown,
    }
}

fn main() {
    let mut task_manager = TaskManager::new();
    
    println!("Welcome to Rust Task Manager!");
    println!("Available commands:");
    println!("  add <title> <description> [due_date]");
    println!("  list");
    println!("  complete <id>");
    println!("  delete <id>");
    println!("  save <filename>");
    println!("  load <filename>");
    println!("  filter <YYYY-MM-DD>");
    println!("  quit");
    
    loop {
        let input = get_user_input("\nEnter command: ");
        let command = parse_command(&input);
        
        match command {
            Command::Add { title, description, due_date } => {
                match task_manager.add_task(title, description, due_date) {
                    Ok(_) => println!("Task added successfully!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Command::List => {
                task_manager.list_tasks();
            }
            Command::Complete { id } => {
                match task_manager.complete_task(id) {
                    Ok(_) => println!("Task marked as complete!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Command::Delete { id } => {
                match task_manager.delete_task(id) {
                    Ok(_) => println!("Task deleted successfully!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Command::Save { filename } => {
                match task_manager.save_tasks(&filename) {
                    Ok(_) => println!("Tasks saved successfully!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Command::Load { filename } => {
                match task_manager.load_tasks(&filename) {
                    Ok(_) => println!("Tasks loaded successfully!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Command::FilterByDate { date } => {
                match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
                    Ok(parsed_date) => {
                        let filtered_tasks = task_manager.filter_by_due_date(parsed_date);
                        println!("Tasks due on {}:", date);
                        for task in filtered_tasks {
                            println!("ID: {}", task.id);
                            println!("Title: {}", task.title);
                            println!("Description: {}", task.description);
                            println!("Status: {:?}", task.status);
                            println!("-------------------");
                        }
                    }
                    Err(e) => println!("Error: Invalid date format. Use YYYY-MM-DD: {}", e),
                }
            }
            Command::Quit => {
                println!("Goodbye!");
                break;
            }
            Command::Unknown => {
                println!("Unknown command. Please try again.");
            }
        }
    }
}