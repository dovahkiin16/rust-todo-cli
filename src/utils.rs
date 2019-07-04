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
            todo_list.set_done(position - 1);
            todo_list.print();
        }
        Command::Delete(position) => {
            todo_list.delete(position - 1);
            todo_list.print();
        }
    };
}

pub fn extract_command(args: &Vec<String>) -> Command {
    match args[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(args[2].clone()),
        "done" => Command::Done(args[2].clone().parse().expect("Expected position to be number")),
        "delete" => Command::Delete(args[2].clone().parse().expect("Expected position to be number")),
        _ => panic!("You must provide an accepted command")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_get_command() {
        let command = extract_command(&vec!["".to_string(), "get".to_string()]);

        match command {
            Command::Get => {}
            _ => panic!("Failed to extract get command")
        }
    }

    #[test]
    fn extract_add_command_with_task() {
        let args = vec!["".to_string(), "add".to_string(), "test".to_string()];
        let command = extract_command(&args);

        match command {
            Command::Add(task) => assert_eq!("test", task),
            _ => panic!("Failed to extract add command")
        }
    }

    #[test]
    #[should_panic]
    fn extract_add_command_without_task() {
        let args = vec!["".to_string(), "add".to_string()];
        let _command = extract_command(&args);
    }

    #[test]
    fn extract_done_command_with_position() {
        let args = vec!["".to_string(), "done".to_string(), "1".to_string()];
        let command = extract_command(&args);

        match command {
            Command::Done(task) => assert_eq!("task", task),
            _ => panic!("Failed to extract done command")
        }
    }

    #[test]
    #[should_panic]
    fn extract_done_command_without_position() {
        let args = vec!["".to_string(), "done".to_string()];
        let _command = extract_command(&args);
    }

    #[test]
    fn extract_delete_command_with_position() {
        let args = vec!["".to_string(), "delete".to_string(), "1".to_string()];
        let command = extract_command(&args);

        match command {
            Command::Delete(task) => assert_eq!("task", task),
            _ => panic!("Failed to extract delete task")
        }
    }

    #[test]
    #[should_panic]
    fn extract_delete_command_without_position() {
        let args = vec!["".to_string(), "delete".to_string()];
        let _command = extract_command(&args);
    }

    #[test]
    #[should_panic]
    fn extract_unknown_command() {
        let args = vec!["".to_string(), "unknown".to_string()];
        let _command = extract_command(&args);
    }
}
