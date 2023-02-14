use std::env;

mod operations;
mod task;
mod variables;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Expected some action");
    }

    let operation_type: &str = &args[1].to_string();

    match operation_type {
        "add" => operations::add_task(args),
        "help" => operations::help(),
        _ => panic!("Unknown action type: {}", operation_type),
    }
}
