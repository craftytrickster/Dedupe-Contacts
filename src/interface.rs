use std::process::exit;
use std::env;
use std::io::{self, Write};
use models::DedupeTask;

const LINE_CLEAR: &'static [u8] = b"\r                                                                 ";

pub fn read_user_input() -> DedupeTask {
    let user_args = env::args().skip(1).collect::<Vec<String>>();
    handle_args(user_args)
}

pub fn confirm_success(file_name: &str) {
    println!("Dedupe has generated the following for your viewing pleasure: {}", file_name);
}

pub fn display_execution_progress(cur_item: usize, total_count: usize) {
    if cur_item == total_count - 1 {
        io::stdout().write(LINE_CLEAR).unwrap();
        io::stdout().write(b"\rDuplicate Processing has been completed.").unwrap();
        println!("");
        println!("");
        return;
    }

    let progress = format!("{:.*}", 2, (cur_item as f64 / total_count as f64) * 100.0);
    let msg = format!("\rCurrent progress of Duplicate Processing: {}%", progress);

    io::stdout().write(&msg.into_bytes()).unwrap();
}

fn handle_args(user_args: Vec<String>) -> DedupeTask {
    match user_args.len() {
        0 => {
            handle_no_args();
            exit(0);
        },
        1 => handle_single_file(&user_args[0]),
        2 => handle_two_files(&user_args[0], &user_args[1]),
        _ => {
            handle_too_many_inputs();
            exit(1);
        }
    }
}

fn handle_no_args() {
    println!("Welcome to \"Magic Dedupe\"");
    println!("*************************");
    println!("");
    println!("This program can do the following:");
    println!("1. Flag duplicates on a single csv file");
    println!("2. Given a base csv file, flag duplicates a second csv file ");
    println!("");
    println!("In order to use, please run the program followed by the name of one or two files");
    println!("Ex: dedupe random-base-file.csv random-second-file.csv");
    println!("");
    println!("The csv files should contain columns in the following order:");
    println!("Last Name | First Name | Company | Phone Number");
}

fn handle_single_file(file: &str) -> DedupeTask {
    println!("You have chosen to flag duplicates on a single file:");
    println!("{}", file);
    println!("");
    println!("If this is correct, please enter (y)es to continue");

    assert_user_continue();

    DedupeTask::SingleFile(file.to_owned())
}

fn handle_two_files(base_file: &str, comparison_file: &str) -> DedupeTask {
    println!("You have chosen to flag duplicates on a comparison file:");
    println!("The base file is {}", base_file);
    println!("The file to flag duplicates on is {}", comparison_file);
    println!("If this is correct, please enter (y)es to continue");

    assert_user_continue();

    DedupeTask::FileComparison(base_file.to_owned(), comparison_file.to_owned())
}

fn handle_too_many_inputs() {
    println!("You have entered too many filenames");
    println!("Please enter a maximum of two filenames:");
    println!("Either one file for a single dedupe task or two files for a comparison task");
}

fn assert_user_continue() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim().to_lowercase();

    if !(buffer == "yes" || buffer == "y") {
        println!("You have chosen \"{}\" to exit the program.", buffer);
        exit(0);
    }
}
