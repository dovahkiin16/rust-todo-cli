use std::env;

mod todos;
mod utils;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut todo_list = todos::TodoList::new();
    let command = utils::extract_command(&arguments);

    todo_list.add_to_list("Hello world!".to_string());

    utils::exec_command(&command, &mut todo_list);
}
