enum Command {
    Add(String),
    Finish(String),
    Remove(String)
}

// TODO: Evaluate commands that has no arguments.
// TODO: Implement display command.
fn deserialize_command(serialized_command: String) -> Option<Command> {
    let mut split = serialized_command.split_whitespace();
    let command = split.next();
    let argument = split.next();

    if let (Some(command), Some(argument)) = (command, argument) {
        let command = command.to_lowercase();
        let argument = argument.to_lowercase();

        if command == "add" {
            return Some(Command::Add(argument))
        }

        if command == "remove" {
            return Some(Command::Remove(argument))
        }

        if command == "finish" {
            return Some(Command::Finish(argument))
        }
        
        None
    } else {
        None
    }
}

// TODO: Handle command actions.
fn process_command(command: String) {
    let command = deserialize_command(command);

    if let None = command {
        println!("Unrecognized command.");
        return;
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

        process_command(command);
    }
}

