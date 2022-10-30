use crate::texts;
use std::io;
use std::process;

pub fn get_user_input() -> String {
    let mut user_input_buffer = String::new();
    return match io::stdin().read_line(&mut user_input_buffer) {
        Ok(_) => user_input_buffer.trim().to_string(),
        Err(_) => {
            println!("Something went wrong...");
            process::exit(1);
        }
    };
}

pub fn handle_empty_target(target: &mut String, target_name: &str) {
    println!(
        "Please enter a {} or type \"exit\" to leave the program.",
        target_name
    );
    target.clear();
    target.push_str(&get_user_input());

    if target == &String::from("exit") {
        exit_process();
    }
}

pub fn exit_process() {
    println!("Got it! See ya~");
    process::exit(1);
}

pub fn exit_or_display_menu() {
    println!("{}", texts::OPTION_EXIT_OR_SHOW_MENU);
    let option = get_user_input();
    if option == "e" {
        exit_process();
    } else {
        texts::options();
    }
}

pub fn exit_retry_menu(cb: fn()) {
    println!("{}", texts::OPTION_EXIT_RETRY_SHOW_MENU);
    let option = get_user_input();
    if option == "r" {
        cb();
    } else if option == "m" {
        texts::options();
    } else {
        exit_process();
    }
}
