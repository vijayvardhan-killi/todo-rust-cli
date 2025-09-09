use  std::io::{Error};
use crate::storage::{save_tasks , load_tasks};
use crate::tasks::utils::print_tasks;
use crate::tasks::Task;


pub  fn run(task: &str) -> Result< () , Error> {
    let mut tasks =  load_tasks();
    let task = Task::new(task);
    tasks.push(task);
    save_tasks(&tasks);
    print_tasks(&tasks);
    Ok(())
}
