mod execution;
mod file;
mod interface;
mod models;
mod searchable;

fn main() {
    let task = interface::read_user_input();
    let created_file = execution::run(task).unwrap();
    interface::confirm_success(&created_file);
}
