extern crate csv;
extern crate fst;
extern crate regex;

mod interface;
mod models;
mod execution;
mod searchable;
mod file;

fn main() {
    let task = interface::read_user_input();
    let created_file = execution::run(task);
    interface::confirm_success(&created_file);
}
