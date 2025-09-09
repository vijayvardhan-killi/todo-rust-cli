use super::Task;
pub fn print_tasks(tasks : &Vec<Task>) {
    for (i,task) in tasks.iter().enumerate(){
        let status = if task.done {"X"} else {" "};
        println!("{} [{}] {}",i+1 , status , task.description);
    }
}