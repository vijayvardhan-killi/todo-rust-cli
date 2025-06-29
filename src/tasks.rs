use  std::fs::{ write, File, OpenOptions};
use  std::io::{Read , Write , Error , ErrorKind};
use  std::path::Path;

pub  fn add_task(task: &str) -> Result< () , Error> {
    let  file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("todo.txt");

    match writeln!(file? , "{}" , task) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::new(ErrorKind::Other, "My god , something went wrong when reading file"))
    }

}


pub fn list_tasks() -> Result<() , Error> {
    let mut file = File::open("todo.txt")?; //Open File or Return Error
    let mut content = String::new() ;
    file.read_to_string(&mut content)?; //Read File Contents as String

    //split by lines , trim empty ones , collect into Vec<String>
    let lines : Vec<String> = content
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    for (i , task) in lines.iter().enumerate() {
        println!("{}.  {}",i+1 ,task);
    }

    Ok(())

}



pub fn delete_task(idx : i64) -> Result<() , Error> {

    //Check if file Alrady exists using Path struct else create todo.txt
    if ! Path::new("todo.txt").exists() {
        File::create("todo.txt")?;
    }

    //Open File in read mode
    let  file = OpenOptions::new()
    .read(true)
    .open("todo.txt");

    let mut file = match file {
        Ok(file) => file,
        Err(e) =>  return Err(e)
    };

    //Read the file to the buffer
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    

    //Generate new Content without idx'th line
    let mut new_content = String::new();
    for (i , line) in buf.lines().enumerate(){
        if i != (idx-1) as usize{
            new_content.push_str(line);
            new_content.push('\n');
        }
    }
    // println!("{}",new_content.trim());

    //Close the file by dropping
    drop(file);


    //Write the new_Content to the file
    // let file = OpenOptions::new()
    // .write(true)
    // .open("todo.txt");

    write("todo.txt",new_content )?;
    Ok(())
}


