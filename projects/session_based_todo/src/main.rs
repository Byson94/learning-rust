use std::env;
use std::io;
use anyhow::{Result, bail, Context, Error};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args); // debug
    
    if args.len() >= 2 {
        if args[1] == "help" {
            print_help();
            Ok(())
        } else if args[1] == "version" {
            println!("session_based_todo v0.1.0");
            Ok(())
        } else {
            bail!("Invalid argument found!")
        }
    } else {
        run_app();
        Ok(())
    }
}

fn run_app() {
    let mut input_text = String::new();
    let mut tasks: Vec<String> = Vec::new();

    loop {
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read line");

        // fltering input_text
        if input_text.ends_with("\n") {
            input_text.pop();
        }

        println!("Your command: {}\n", input_text);

        let command: Vec<&str> = input_text.split_whitespace().collect();

        if command.is_empty() {
            println!("Please enter a non-empty command.\n")
        }

        if command.len() >= 1 {
            if command[0] == "add" {
                if command.len() >= 2 {
                    tasks.push(command[1].to_string());
                } else {
                    println!("Error: Invalid use of 'add'\n")
                }
            } else if command[0] == "list" {
                for (i, task) in tasks.iter().enumerate() {
                    println!("{i}. {task}");
                }
            } else if command[0] == "remove" {
                if command.len() >= 2 {
                    tasks.remove(command[1])
                } else {
                    println!("Error: Invalid use of 'remove'\n")
                }
            } else {
                println!("Error: Command not found\n");
            }
        } else {
            println!("Error: Command not found\n");
        }

        // clearing all data
        input_text = String::new();
    }
}

fn add_task(task: &str) -> Result<bool, &str> {
    Ok(true)
}

fn print_help() {
    println!("Usage: session_based_todo [argument]");
    println!("");
    println!("  help        print this help");
    println!("  version     print the version");
    println!("");
}