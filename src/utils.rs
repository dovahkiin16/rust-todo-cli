use crate::todos::Command;
use crate::todos::TodoList;

pub fn exec_command(command: &Command, todo_list: &mut TodoList) {
    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add_to_list(task.to_owned());
            todo_list.print();
        }
        Command::Done(position) => {
            let pos_int = position.parse::<usize>().expect("Invalid task position");
            todo_list.set_done(pos_int - 1);
            todo_list.print();
        }
        Command::Delete(position) => {
            let pos_int = position.parse::<usize>().expect("Invalid task position");
            todo_list.delete(pos_int - 1);
            todo_list.print();
        }
    };
}

pub fn extract_command(args: &Vec<String>) -> Command {
    match args[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(args[2].clone()),
        "done" => Command::Done(args[2].clone()),
        "delete" => Command::Delete(args[2].clone()),
        _ => panic!("You must provide an accepted command")
    }
}
