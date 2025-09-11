mod err;
use err::*;
use std::error::Error;
use std::fs::read_to_string;
#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        
    let contents =match read_to_string(path) {
        Ok(result) => result, 
        Err( res) => return Err(Box::new(
            ReadErr{
            child_err: Box::new(res),
        }))
    };
     let parsing=  json::parse(&contents).map_err(|err|ParseErr::Malformed(Box::new(err)));
     if parsing["task"].is_empty() {
        return ParseErr::Empty;
     }

    println!("With text:\n{contents}");
        todo!()
    }
}