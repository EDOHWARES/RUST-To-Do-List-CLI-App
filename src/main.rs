use std::{fs, io};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Task {
    name: String,
    done: bool,
}

fn save_tasks(tasks: &[Task]) {
    let data = serde_json::to_string(tasks).expect("Failed to serialize");
    fs::write("tasks.json", data).expect("Failed to write file");
}

fn load_tasks() -> Vec<Task> {
    let data = fs::read_to_string("tasks.json").unwrap_or("[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks();

    loop {
        println!("\nOptions:");
        println!("1. Add task");
        println!("2. View task");
        println!("3. Mark task as done");
        println!("4. Delete task");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
                println!("Enter task name:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                tasks.push(Task {
                    name: input.trim().to_string(),
                    done: false,
                });

                save_tasks(&tasks);
                println!("✅ Task added!");
            }
            "2" => {
                println!("\nTasks:");
                if tasks.is_empty() {
                    println!("📭 No tasks yet!");
                } else {
                    for (i, task) in tasks.iter().enumerate() {
                        let status = if task.done { "✅" } else { "[ ]" };
                        println!("{}. {} {}", i + 1, status, task.name);
                    }
                }
            }
            "3" => {
                println!("Enter task number to mark as done:");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read input");

                if let Ok(i) = index.trim().parse::<usize>() {
                    if i > 0 && i <= tasks.len() {
                        tasks[i - 1].done = true;
                        save_tasks(&tasks);
                        println!("✅ Task marked as done!");
                    } else {
                        println!("❌ Invalid task number.");
                    }
                } else {
                    println!("❌ Please enter a valid number.")
                }
            }
            "4" => {
                println!("Enter a task number to delete:");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read input");

                if let Ok(i) = index.trim().parse::<usize>() {
                    if i > 0 && i <= tasks.len() {
                        tasks.remove(i - 1);
                        save_tasks(&tasks);
                        println!("🗑️ Task deleted!");
                    } else {
                        println!("❌ Invalid task number.");
                    }
                } else {
                    println!("❌ Please enter a valid number.")
                }
            }
            "5" => {
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option!"),
        }
    }
}
