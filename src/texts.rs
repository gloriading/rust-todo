pub const OPTION_READ: &str = "[1] read - see all todos";
pub const OPTION_ADD: &str = "[2] add - add a todo";
pub const OPTION_DELETE: &str = "[3] delete - delete a todo";
pub const OPTION_UPDATE: &str = "[4] update - update a todo";
pub const OPTION_EXIT: &str = "[5] exit the app";
pub const OPTION_INTRO: &str = "Please enter one of the options: 1, 2, 3, 4, 5";
pub const OPTION_EXIT_OR_SHOW_MENU: &str =
    "Please enter 'e' to exit or any key to display the menu";
pub const OPTION_EXIT_RETRY_SHOW_MENU: &str =
    "Please enter 'r' to retry, 'm' to show menu, or any key to exit";

pub fn options() {
    println!("{}", OPTION_READ);
    println!("{}", OPTION_ADD);
    println!("{}", OPTION_DELETE);
    println!("{}", OPTION_UPDATE);
    println!("{}", OPTION_EXIT);
}
pub fn greetings() {
    println!("Hello there! How may I help you today?");
    println!("Enter a number from the below options:");
    println!("{}", OPTION_READ);
    println!("{}", OPTION_ADD);
    println!("{}", OPTION_DELETE);
    println!("{}", OPTION_UPDATE);
    println!("{}", OPTION_EXIT);
}
