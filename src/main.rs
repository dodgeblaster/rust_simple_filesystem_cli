use std::env;
mod filesystem;

fn print() {
    let file_path = "test.txt";
    match filesystem::read_file(file_path) {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("Error: {}", e),
    }
}

fn list() {
    let current_dir = env::current_dir().unwrap();
    let current_dir_str = current_dir.to_str().unwrap();
    match filesystem::list_directory(current_dir_str) {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("Error: {}", e),
    }
}

fn write() {
    let current_dir = env::current_dir().unwrap();
    let file_name = current_dir.join("blog.md");
    let content = "my blog";
    match filesystem::write_file(file_name.to_str().unwrap(), content) {
        Ok(_) => println!("File {} written successfully", file_name.to_str().unwrap()),
        Err(e) => println!("Error writing file {}: {}", file_name.to_str().unwrap(), e),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run <command>");
        println!("Available commands: print, list, write");
        return;
    }

    let command = &args[1];
    match command.as_ref() {
        "print" => print(),
        "list" => list(),
        "write" => write(),
        _ => println!("Unknown command: {}", command),
    }
}