use  std::io::{ Error};

use crate::{storage::load_tasks, tasks::utils::print_tasks};


pub fn run() -> Result<() , Error> {
    let  tasks  = load_tasks();

    print_tasks(&tasks);
    
    Ok(())

}
