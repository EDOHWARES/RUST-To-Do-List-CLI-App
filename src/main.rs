use std::io;

#[derive(Debug)]
struct Task {
    name: String,
    done: bool,
}
fn main() {
    let mut tasks: Vec<Task> = Vec::new();

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

                let task = Task {
                    name: input.trim().to_string(),
                    done: false,
                };

                tasks.push(task);
            },
            "2" => {
                println!("\nTasks:");
                for (i, task) in tasks.iter().enumerate()  {
                    let status = if task.done {"✅"} else {"[ ]"};
                    println!("{}. {} {}", i + 1, status, task.name);
                }
            }, 
            "3" => {
                println!("Enter task number to mark as done:");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to read input");

                if let Ok(i) = index.trim().parse::<usize>() {
                    if i > 0 && i <= tasks.len() {
                        tasks[i - 1].done = true;
                    } else {
                        println!("Invalid task number.");
                    }
                } else {
                    println!("Please enter a valid number.")
                }
            }, 
            "4" => {
                println!("Enter a task number to delete:");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to read input");

                if let Ok(i) = index.trim().parse::<usize>() {
                    if i > 0 && i <=tasks.len() {
                        tasks.remove(i - 1);
                    } else {
                        println!("Invalid task number.");
                    }
                } else {
                    println!("Invalid task number.");
                }
            },
            "5" => break,
            _=> println!("Invalid option"),
        }
    }
}
