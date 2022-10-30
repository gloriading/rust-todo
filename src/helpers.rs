mod file_helpers;
mod input_output_helpers;
use crate::texts;
use std::io;

pub fn get_initial_option() -> u8 {
    let mut user_input_buffer = String::new();
    return match io::stdin().read_line(&mut user_input_buffer) {
        Ok(_) => match user_input_buffer.trim().parse::<u8>() {
            Ok(n) => {
                if n > 0 && n < 6 {
                    n
                } else {
                    println!("{}", texts::OPTION_INTRO);
                    0
                }
            }
            Err(_) => {
                println!("Fail to parse your input. {}", texts::OPTION_INTRO);
                0
            }
        },
        Err(_) => {
            println!("hmm, something went wrong. {}", texts::OPTION_INTRO);
            0
        }
    };
}

pub fn read_todos() {
    match file_helpers::read_files_from_dir("todos") {
        Ok(_) => {
            println!("\n--- End of todos --- \n");
            println!("{}", texts::OPTION_INTRO);
            texts::options();
        }
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => println!("File not found"),
            io::ErrorKind::PermissionDenied => println!("Permission Denied."),
            _ => panic!("Unknown error."),
        },
    }
}

pub fn add_todo() {
    println!("Please enter a title");
    let mut title = input_output_helpers::get_user_input();

    loop {
        if title.is_empty() {
            input_output_helpers::handle_empty_target(&mut title, "title");
        } else {
            break;
        }
    }

    println!("Please enter the content");
    let mut content = input_output_helpers::get_user_input();

    loop {
        if content.is_empty() {
            input_output_helpers::handle_empty_target(&mut content, "content");
        } else {
            break;
        }
    }

    match file_helpers::create_and_write_to_file(&title, &content) {
        Ok(_) => {
            println!("\nYou have created a todo! \n");
            input_output_helpers::exit_or_display_menu();
        }
        Err(e) => match e.kind() {
            io::ErrorKind::AlreadyExists => {
                println!("You have entered this todo title already.");
                input_output_helpers::exit_retry_menu(add_todo);
            }
            _ => panic!("Unknown error."),
        },
    }
}

pub fn remove_todo() {
    println!("Please enter a title of the todo for deletion");
    let title = input_output_helpers::get_user_input();

    match file_helpers::remove_file(&title) {
        Ok(_) => {
            println!("\nYuu have deleted a todo. \n");
            input_output_helpers::exit_or_display_menu();
        }
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                println!("File not found.");
                input_output_helpers::exit_retry_menu(remove_todo);
            }
            io::ErrorKind::PermissionDenied => {
                println!("Permission Denied.");
                input_output_helpers::exit_retry_menu(remove_todo);
            }
            _ => panic!("Unknown error."),
        },
    }
}

pub fn update_todo() {
    println!("Please enter a title of the todo for updating");
    let mut title = input_output_helpers::get_user_input();

    loop {
        if title.is_empty() {
            input_output_helpers::handle_empty_target(&mut title, "title");
        } else {
            break;
        }
    }

    println!("Please enter the updated content");
    let mut content = input_output_helpers::get_user_input();

    loop {
        if content.is_empty() {
            input_output_helpers::handle_empty_target(&mut content, "content");
        } else {
            break;
        }
    }

    match file_helpers::update_file(&title, &content) {
        Ok(_) => {
            println!("\nYou have update the todo: {}! \n", title);
            input_output_helpers::exit_or_display_menu();
        }
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                println!("File not found");
                input_output_helpers::exit_retry_menu(update_todo);
            }
            io::ErrorKind::PermissionDenied => {
                println!("Permission Denied.");
                input_output_helpers::exit_retry_menu(update_todo);
            }
            _ => panic!("Unknown error."),
        },
    }
}
