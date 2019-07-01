pub mod todo_item;
pub mod todo_list;

pub enum Command {
    Get,
    Add(String),
}

#[derive(Debug)]
pub struct TodoItem {
    pub title: String,
    pub done: char,
}

impl TodoItem {
    pub fn new(title: String) -> TodoItem {
        return TodoItem {
            title,
            done: ' ',
        };
    }
}

#[derive(Debug)]
pub struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { list: Vec::new() }
    }

    pub fn add_to_list(&mut self, title: String) {
        self.list.push(TodoItem::new(title));
    }

    pub fn print(self) {
        for item in self.list {
            println!("[{}] - {}", item.done, item.title);
        }
    }
}
