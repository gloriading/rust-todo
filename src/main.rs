mod helpers;
pub mod texts;

fn main() {
    texts::greetings();
    loop {
        let initial_option = helpers::get_initial_option();
        match initial_option {
            1 => helpers::read_todos(),
            2 => helpers::add_todo(),
            3 => helpers::remove_todo(),
            4 => helpers::update_todo(),
            5 => break,
            _ => {
                texts::options();
                continue;
            }
        }
    }
}
