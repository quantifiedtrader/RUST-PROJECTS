// Explanation
// 1. Maintains a list of tasks using a hash map where each task has a unique ID.
// 2. Offers options to add, list, and remove tasks.
// 3. Provides a menu-driven CLI interface for user interaction.

use std::io;
use std::collections::HashMap;

//ToDoList:struct
// tasks: Stores tasks using a HashMap. The keys (u32) represent unique task IDs,
// and the values (String) represent task descriptions.
// next_id: Keeps track of the next ID for new tasks to ensure uniqueness.

struct ToDoList{
    task: HashMap<u32,String>, // Maps task ID to task description
    next_id:u32,               // Tracks the next available task ID
}

impl ToDoList {

    // new:
    // Initializes an empty ToDoList with:
    // 1. An empty HashMap.
    // 2. next_id set to 1.

    fn new() -> Self {
        ToDoList {
            task: HashMap::new(),
            next_id: 1,
        }
    }

    // add_task:
    // Accepts the task description as a String.
    // Adds a new task to the HashMap using the current next_id.
    // Increments next_id: Ensures each task gets a unique ID.

    fn add_task(&mut self, task: String) {
        self.task.insert(self.next_id,task);
        println!("Task added with ID:{}", self.next_id);
        self.next_id += 1;
    }

    // list_tasks
    //  Prints all tasks, or a message if no tasks exist.
    //  The program iterates over the HashMap and displays key-value pairs.

    fn list_tasks(&self) {
        if self.task.is_empty() {
            println!("Nothing to do");
        } else {
            for (id, task) in &self.task {
                println!("[{}]-{}", id, task);
            }
        }
    }

    // remove_task
    // Accepts the task_id of the task to be removed.
    // Deletes the corresponding entry from the HashMap if it exists.
    // Ensures the program can handle invalid IDs gracefully.


    fn remove_task(&mut self, task_id: u32) {
        if self.task.remove(&task_id).is_some() {
            println!("Task removed with ID:{}", task_id);
        } else {
            println!("Task not found with ID:{}", task_id);
        }
    }
    }
    // display_menu
    // Displays the available actions to the user.
    fn display_menu() {
        println!("\nTo Do List CLI");
        println!("1. Add a Task");
        println!("2. List Task");
        println!("3. Remove a Task");
        println!("4. Exit");
    }

    // get_input
    // Repeatedly prompts the user for input until valid data is provided.
    // Input handling ensures clean interaction

    fn get_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    }


fn main() {
    let mut todo_list = ToDoList::new(); // Initialize a new to-do list

    loop {
        display_menu();
        let choice = get_input("Your choice:");

        match choice.as_str() {
            "1" => {
                // Option to add a task
                let task = get_input("Enter the task description:");
                todo_list.add_task(task);
            }
            "2" => {
                // Option to list all tasks
                todo_list.list_tasks();
            }
            "3" => {
                // Option to remove a task
                let task_id_input = get_input("Enter the ID of the task to remove:");
                match task_id_input.parse::<u32>() {
                    Ok(task_id) => todo_list.remove_task(task_id), // Remove task if valid ID
                    Err(_) => println!("Invalid ID. Please enter a valid number."), // Handle invalid input
                }
            }
            "4" => {
                // Option to exit the application
                println!("Exiting the To-Do List CLI. Goodbye!");
                break; // Exit the loop and terminate the program
            }
            _ => {
                // Handle invalid menu choices
                println!("Invalid choice. Please enter a number between 1 and 4.");
            }
        }
    }
}

// More additions -
// Add Date of Creation in the name
// Add expected/target date of competition


























