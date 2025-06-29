mod tasks;

use tasks::{add_task , list_tasks , delete_task};

//import env module to get cli args
use std::env;



fn main() {
    //Get standard input from cli and covert to Vector<String>
    let command_arguments :Vec<String> =  env::args().collect();
    if command_arguments.len() < 2{ //if no arguments provided , return 
        eprintln!("Provide enough arguments");
        return;
    }

    //extract command as string slice
    let command = command_arguments[1].as_str();

    //match the command
    match  command {
        "add" => {
            //  If the task_name is not available return 
            if command_arguments.len() < 3{
                println!("Name of the task should be provided")
            }
            
            
            let task  = command_arguments[2..].join(" ");

            //add the task to the list
            if let Err(e) = add_task(&task) {
                println!("{}",e);
                return;
            }

            println!("{} added to TODO list" , task);
        },
        "list" =>{
            //show the tasks in the list
            if let Err(e) = list_tasks() {
                print!("Error {}, Thats all folks",e)
            } 
        },

        "delete" => {
            //delete the task with given number
            let x  = command_arguments[2].parse().unwrap_or(-1);
            if x==-1 {
                println!("Enter Valid Index of the task");
                return;
            }
            if let Err(e) = delete_task(x) {
                println!("Error {} while Deleting Task {}",e ,x);
                return;
            }
            println!("Deleted Task - {} from TODO List",x);

        },
        "help" => {
            //Show the options / commands available
            println!("Usage:");
            println!("  todo_cli add <task description>   # Add a new task");
            println!("  todo_cli delete <task number>     # Delete task");
            println!("  todo_cli list                     # List all tasks");
            println!("  todo_cli help                     # Show this help");
        },
        _ => {
            //if not matched show default instruction for getting help
            println!("  I dont know what you are saying ");
            println!("  todo_cli help                     # Show this help");
        }
    }
}
