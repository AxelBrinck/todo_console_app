enum Command {
    Add(String),
    Finish(String),
    Remove(String)
}

fn process_command(command: String) {
    
}

fn main() {
    println!("To-Do App made with Rust!");
    
    let stdin = std::io::stdin();

    loop {
        let mut command_raw = String::new();

        let read_result = stdin.read_line(&mut command_raw);

        if let Err(_i) = read_result {
            println!("An error occured while trying to read from stdin.");
        }

        let command = command_raw.replace("\r\n", "");

        if command == String::from("exit") {
            break;
        }

        process_command(command);
    }
}

