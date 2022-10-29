// use std::fs;
use std::io;
// use std::io::prelude::*;
mod file_helpers;

pub fn get_initial_option() -> u8 {
    let mut user_input_buffer = String::new();
    return match io::stdin().read_line(&mut user_input_buffer) {
        Ok(_) => match user_input_buffer.trim().parse::<u8>() {
            Ok(n) => {
                if n > 0 && n < 5 {
                    n
                } else {
                    println!("Please enter one of the options: 1, 2, 3, 4");
                    0
                }
            }
            Err(_) => {
                println!("Fail to parse your input. Please enter a number from the options.");
                0
            }
        },
        Err(_) => {
            println!("hmm, something went wrong. Please enter a number from the options");
            0
        }
    };
}

pub fn read_todos() {
    match file_helpers::read_files_from_dir("todos") {
        Ok(_) => println!("--- end of todos ---"),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => println!("File not found"),
            io::ErrorKind::PermissionDenied => println!("Permission Denied."),
            _ => panic!("Unknown error."),
        },
    }
}

fn get_user_input() -> String {
    let mut user_input_buffer = String::new();
    return match io::stdin().read_line(&mut user_input_buffer) {
        Ok(_) => user_input_buffer.trim().to_string(),
        Err(_) => {
            println!("Something went wrong...");
            String::from("")
        }
    };
}

pub fn add_todo() {
    println!("Please enter a title");
    let title = get_user_input();

    println!("Please enter a content");
    let content = get_user_input();

    match file_helpers::create_and_write_to_file(&title, &content) {
        Ok(_) => println!("You have created a todo!"),
        Err(e) => match e.kind() {
            io::ErrorKind::AlreadyExists => println!("You have entered this todo title already."),
            _ => panic!("Unknown error."),
        },
    }
}

pub fn remove_todo() {
    println!("Please enter a title of the todo for deletion");
    let title = get_user_input();

    match file_helpers::remove_file(&title) {
        Ok(_) => println!("Yuu have deleted a todo."),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => println!("File not found"),
            io::ErrorKind::PermissionDenied => println!("Permission Denied."),
            _ => panic!("Unknown error."),
        },
    }
}

pub fn update_todo() {
    println!("Please enter a title of the todo for updating");
    let title = get_user_input();

    println!("Please enter the updated content");
    let content = get_user_input();

    match file_helpers::update_file(&title, &content) {
        Ok(_) => println!("You have update {}!", title),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => println!("File not found"),
            io::ErrorKind::PermissionDenied => println!("Permission Denied."),
            _ => panic!("Unknown error."),
        },
    }
}
