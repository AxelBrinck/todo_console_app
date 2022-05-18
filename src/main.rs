enum Command {
    Add(String),
    Remove(String),
    Display
}

fn load_tasks() -> Vec<String> {
    let file = std::fs::read_to_string("tasks.txt");
    
    if let Ok(file) = file {
        let tasks_file = file.split("\r\n");
        let mut tasks: Vec<String> = Vec::new();

        for task in tasks_file {
            if task != "" {
                tasks.push(task.to_string());
            }
        }

        return tasks;
    }

    Vec::new()
}

fn save_tasks(tasks: &Vec<String>) {
    let mut file_contents = String::new();

    for task in tasks {
        file_contents.push_str(&(task.to_string() + "\r\n"));
    }

    let write_result = std::fs::write("tasks.txt", file_contents);

    if let Err(_i) = write_result {
        panic!("Could not write to file.");
    }
}

fn add_task(task: &str) {
    let mut tasks = load_tasks();

    tasks.push(task.to_string());

    save_tasks(&tasks);
}

fn remove_task(task: &str) -> bool {
    let mut tasks = load_tasks();

    let index = tasks.iter().position(|x| x.to_lowercase() == task.to_lowercase());

    if let Some(x) = index {
        tasks.remove(x);

        save_tasks(&tasks);

        return true;
    }

    return false;
}

fn display_tasks() {
    let tasks = load_tasks();

    if tasks.len() == 0 {
        print!("No elements.");
        return;
    }

    for task in tasks {
        println!("{}", task);
    }
}

fn deserialize_command(serialized_command: &str) -> Option<Command> {

    let serialized_command = serialized_command.trim();

    if let Some(command) = serialized_command.split_whitespace().next() {
        let command = command.trim().to_lowercase();
        let argument = &serialized_command[command.chars().count()..].trim();

        if command == "add" {
            return Some(Command::Add(argument.to_string()));
        }

        if command == "remove" {
            return Some(Command::Remove(argument.to_string()))
        }

        if command == "display" {
            return Some(Command::Display)
        }
    }

    None
}

fn process_command(command: &str) {
    let command = deserialize_command(command);

    if let Some(command) = command {
        match command {
            Command::Add(task) => add_task(&task),
            Command::Remove(task) => {
                if remove_task(&task) {
                    println!("OK!");
                } else {
                    println!("Task not found.");
                }
            },
            Command::Display => display_tasks()
        }
    } else {
        println!("Unrecognized command.");
    }
}

fn main() {
    println!("To-Do App made with Rust!");
    
    let stdin = std::io::stdin();

    loop {
        let mut input = String::new();

        stdin.read_line(&mut input).expect("An error occured while trying to read from stdin.");

        let command = input.replace("\r\n", "");

        if command == String::from("exit") {
            break;
        }

        process_command(&command);
    }
}

