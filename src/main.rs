mod helpers;
pub mod texts;

fn main() {
    texts::greetings();
    loop {
        let initial_option = helpers::get_initial_option();
        match initial_option {
            1 => helpers::read_todos(),
            2 => helpers::read_todo(),
            3 => helpers::add_todo(),
            4 => helpers::remove_todo(),
            5 => helpers::update_todo(),
            6 => helpers::input_output_helpers::exit_process(),
            _ => {
                texts::options();
                continue;
            }
        }
    }
}
