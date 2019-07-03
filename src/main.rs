use std::env;
use crate::todos::*;

mod todos;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut todo_list = TodoList::new();
    let command = extract_command(&arguments);

    todo_list.add_to_list("Hello world!".to_string());

    exec_command(&command, &mut todo_list);
}

fn extract_command(args: &Vec<String>) -> Command {
    match args[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(args[2].clone()),
        "done" => Command::Done(args[2].clone()),
        "delete" => Command::Delete(args[2].clone()),
        _ => panic!("You must provide an accepted command")
    }
}

fn exec_command(command: &Command, todo_list: &mut TodoList) {
    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add_to_list(task.to_owned());
            todo_list.print();
        }
        Command::Done(position) => {
            let pos_int = position.parse::<usize>().unwrap();
            todo_list.set_done(pos_int - 1);
            todo_list.print();
        }
        Command::Delete(position) => {
            let pos_int = position.parse::<usize>().unwrap();
            todo_list.delete(pos_int - 1);
            todo_list.print();
        }
    };
}
