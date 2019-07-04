pub enum Command {
    Get,
    Add(String),
    Done(usize),
    Delete(usize),
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

    pub fn print(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!("[{}] - {}. {}", item.done, (index+1), item.title);
        }
    }

    pub fn set_done(&mut self, position: usize) {
        match self.list.get_mut(position) {
            Some(task) => task.done = 'x',
            None => println!("Task not found.")
        }
    }

    pub fn delete(&mut self, position: usize) {
        self.list.remove(position);
    }
}
