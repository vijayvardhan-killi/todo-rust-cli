use  std::io::{Error };

use crate::{storage::{load_tasks, save_tasks}, tasks::utils::print_tasks};

pub fn run(idx : usize) -> Result<() , Error> {

    let mut tasks = load_tasks();

    tasks.remove(idx-1);
    
    save_tasks(&tasks);
    
    print_tasks(&tasks);
    println!();

    Ok(())
}
