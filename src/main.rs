use std::{fs, io, io::Write};

#[derive(Debug)]
struct TodoItem {
    id: u16,
    title: String,
    completed: bool
}

struct TodoList<'a> {
    items: Vec<TodoItem>,
    file: &'a str
}

impl TodoList<'_> {
    fn new(file: &str) -> TodoList {
        TodoList {
            items: Vec::new(),
            file
        }
    }

    fn load(&mut self) {
        match fs::read_to_string(self.file) {
            Ok(database) => {
                for line in database.lines() {
                    let parts: Vec<&str> = line.split('-').collect();
                    if parts.len() == 3 {
                        let id = parts[0].parse().unwrap();
                        let title = parts[1].to_string();
                        let status = parts[2].parse().unwrap();
                        let item = TodoItem {id, title, completed: status};
                        self.items.push(item);
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to read from the file: {}", e);
            }
        }

    }

    fn save(&self){
        if self.items.is_empty() {
            println!("List is empty, not saving shit bruh")
        } else {
            let mut fd = fs::File::create(self.file).expect("Error opening file!");
            for item in &self.items {
                // println!("item: {}", item.title);
                writeln!(fd, "{:?}-{}-{:?}", item.id, item.title, item.completed).expect("Error saving data to file!");
            }
        }
    }

    fn add_item(&mut self, title: String) {
        let id = self.items.len() as u16 + 1;
        let new = TodoItem {
            id,
            title: title.clone(),
            completed: false
        };
        self.items.push(new);
        println!("Added {}", title);
    }

    fn list_items(&self) {
        if self.items.is_empty() {
            println!("The list is empty");
        } else {
            println!("The todo list:");
            for item in &self.items {
                let status = if item.completed {"[X]"} else {"[ ]"};
                println!("{} {} - {}", status, item.id, item.title);
            }
        }
    }

    fn complete_item(&mut self, id: u16) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.completed = true;
            println!("Changed status for: {}", item.title);
        } else {
            println!("Item with the id: {} was not found!", id);
        }
    }
}

fn get_string_input<'a>() -> String {
    let mut choice = String::new();
    let bytes_read = io::stdin().read_line(&mut choice).unwrap_or_else(|err| {
        panic!("Error reading input: {}", err);
    });

    if bytes_read == 0 {
        println!("Ctrl + D was pressed, fuck off");
        std::process::exit(1);
    }
    return choice.trim().to_string();
}

fn get_short_input<'a>() -> u16 {
    let mut choice = String::new();
    let bytes_read = io::stdin().read_line(&mut choice).unwrap_or_else(|err| {
        panic!("Error reading input: {}", err);
    });

    if bytes_read == 0 {
        println!("Ctrl + D was pressed, fuck off");
        std::process::exit(1);
    }

    match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error parsing input");
            std::process::exit(1);
        },
    }
}

fn main() {

    let mut list = TodoList::new("test.txt");

    list.load();
    loop {
        println!("1. Add new item");
        println!("2. List items");
        println!("3. Change item status");
        println!("4. Exit");

        let choice: u16 = get_short_input();

        match choice {
            1 => {
                println!("Enter the title of the new item: ");
                let title = get_string_input();
                list.add_item(title);
            }

            2 => {
                list.list_items();
            }

            3 => {
                println!("Enter the id, of the item to change: ");
                let id = get_short_input();
                list.complete_item(id);
            }

            4 => {
                println!("Exiting program");
                list.save();
                break;
            }

            _ => {
                println!("Invalid option, give a number from 1 to 4 bitch");
            }
        };
    }
}
