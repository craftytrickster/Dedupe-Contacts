#[macro_use]
extern crate lazy_static;

mod interface;
mod models;
mod execution;
mod searchable;
mod file;

fn main() {
    let task = interface::read_user_input();
    let created_file = execution::run(task).unwrap();
    interface::confirm_success(&created_file);
}
