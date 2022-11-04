pub const OPTION_READ_ALL: &str = "[1] read - see all todos";
pub const OPTION_READ: &str = "[2] read - read a todo";
pub const OPTION_ADD: &str = "[3] add - add a todo";
pub const OPTION_DELETE: &str = "[4] delete - delete a todo";
pub const OPTION_UPDATE: &str = "[5] update - update a todo";
pub const OPTION_EXIT: &str = "[6] exit the app";
pub const OPTION_INTRO: &str = "Please enter one of the options:";
pub const OPTION_EXIT_OR_SHOW_MENU: &str =
    "Please enter 'e' to exit or any key to display the menu";
pub const OPTION_EXIT_RETRY_SHOW_MENU: &str =
    "Please enter 'r' to retry, 'm' to show menu, or any key to exit";
pub const OPTION_READ_A_TODO: &str =
    "[u] update [d] delete this todo \n[m] back to menu or any key to exis";

pub const LINE_BLANK: &str = "\n-----^--------------------^-----\n";
pub const LINE_END: &str = "\n-----^--------end---------^-----\n";

pub fn options() {
    println!("{}", OPTION_READ_ALL);
    println!("{}", OPTION_READ);
    println!("{}", OPTION_ADD);
    println!("{}", OPTION_DELETE);
    println!("{}", OPTION_UPDATE);
    println!("{}", OPTION_EXIT);
}
pub fn greetings() {
    println!("Hello there! How may I help you today?");
    println!("Enter a number from the below options:");
    options();
}
