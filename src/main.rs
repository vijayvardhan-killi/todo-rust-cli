mod tasks;
mod storage;

use tasks::{add , delete , list , done};
//Parser from 
use clap::{Parser, Subcommand };



#[derive(Parser)]
#[command(name = "todo" , version , )]
struct Cli {
    #[command(subcommand)]
    command : Commands
}

#[derive(Subcommand)]
enum Commands {
    // Add new task
    Add {
        // task descrition
        task : Vec<String>,
    },

    // List all the tasks
    List ,
    // Delete task by number
    Delete {
        // The task number (1-based index)
        index : usize,
    },
    
    Done {
        // The task number (1-based index)
        index : usize,
    }
}


fn main() {
        let cli = Cli::parse();

    //match the command
    match  cli.command {
        Commands::Add { task } => {
            //  If the task_name is not available return 
            let task_desc = task.join(" ");
            if let Err(e) =  add::run(&task_desc){
                println!("{}",e);
            }
        },
        Commands::List =>{
            //show the tasks in the list
            if let Err(e) = list::run() {
                println!("Error {}, Thats all folks",e)
            } 
        },

        Commands::Delete { index } => {
            //delete the task with given number
            
            if let Err(e) = delete::run(index) {
                println!("Error {} while Deleting Task {}",e ,index);
                return;
            }
            println!("Deleted Task - {} from TODO List",index);

        },
        Commands::Done {index} => {
            if let Err(e) = done::run(index) {
                println!("An error occured ! {}",e);
            }
        }

    }
}
