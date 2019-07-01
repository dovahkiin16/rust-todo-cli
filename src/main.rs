use std::env;
use crate::todos::*;

mod todos;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut todo_list = TodoList::new();
    let command = match arguments[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(arguments[2].clone()),
        _ => panic!("You must provide an accepted command")
    };

    todo_list.add_to_list("Hello world!".to_string());

    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add_to_list(task);
            todo_list.print();
        }
    }
}
