use clap::error::ErrorKind;

use crate::{storage::{load_tasks , save_tasks}, tasks::utils::print_tasks};

pub fn run(idx : usize) -> Result<() , ErrorKind> {
    let mut tasks = load_tasks();
    for (i , task) in tasks.iter_mut().enumerate()  {
        if idx == i+1 &&  task.done == false{
            task.done = true;
        }
    }
    save_tasks(&tasks);
    print_tasks(&tasks);

    return  Ok(());
}