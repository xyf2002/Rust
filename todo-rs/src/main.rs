use std::env;

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // If the user has not provided any arguments
    if args.len() < 2 {
        println!("Usage: todo [add|rm|ls] [args]");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: todo add [contents]");
                return;
            }
            println!("Add");
        }
        "rm" => {
            if args.len() < 3 {
                println!("Usage: todo rm [id]");
                return;
            }
            println!("Remove");
        }
        "ls" => {
            println!("List");
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}

