use crate::models::DedupeTask;
use std::env;
use std::io::{self, Write};
use std::process::exit;

const LINE_CLEAR: &[u8] = b"\r                                                                 ";

pub fn read_user_input() -> DedupeTask {
    let user_args = env::args().skip(1);
    handle_args(user_args)
}

pub fn confirm_success(file_name: &str) {
    println!(
        "Dedupe has generated the following for your viewing pleasure: {}",
        file_name
    );
}

pub fn display_execution_progress(cur_item: usize, total_count: usize) {
    if cur_item == total_count - 1 {
        io::stdout().write_all(LINE_CLEAR).unwrap();
        io::stdout()
            .write_all(b"\rDuplicate Processing has been completed.")
            .unwrap();
        println!();
        println!();
        return;
    }

    let progress = format!("{:.*}", 2, (cur_item as f64 / total_count as f64) * 100.0);
    let msg = format!("\rCurrent progress of Duplicate Processing: {}%", progress);

    io::stdout().write_all(&msg.into_bytes()).unwrap();
}

fn handle_args<T>(mut user_args: T) -> DedupeTask
where
    T: Iterator<Item = String>,
{
    let first = user_args.next();
    let second = user_args.next();
    let nth = user_args.next();

    match (first, second, nth) {
        (None, ..) => {
            handle_no_args();
            exit(0);
        }
        (Some(file), None, None) => handle_single_file(file),
        (Some(file1), Some(file2), None) => handle_two_files(file1, file2),
        _ => {
            handle_too_many_inputs();
            exit(1);
        }
    }
}

fn handle_no_args() {
    println!("Welcome to \"Magic Dedupe\"");
    println!("*************************");
    println!();
    println!("This program can do the following:");
    println!("1. Flag duplicates on a single csv file");
    println!("2. Given a base csv file, flag duplicates in a second csv file,");
    println!("   keeping in mind that the files must have the same amount of columns");
    println!();
    println!("Examples:");
    println!();
    println!("Single File Example:");
    println!("dedupe random-base-file.csv");
    println!();
    println!("Multi File Example:");
    println!("dedupe random-base-file.csv random-second-file.csv");
}

fn handle_single_file(file: String) -> DedupeTask {
    println!("You have chosen to flag duplicates on a single file:");
    println!("{}", file);
    println!();
    println!("If this is correct, please enter (y)es to continue");

    assert_user_continue();

    DedupeTask::SingleFile(file)
}

fn handle_two_files(base_file: String, comparison_file: String) -> DedupeTask {
    println!("You have chosen to flag duplicates on a comparison file:");
    println!("The base file is {}", base_file);
    println!("The file to flag duplicates on is {}", comparison_file);
    println!("If this is correct, please enter (y)es to continue");

    assert_user_continue();

    DedupeTask::FileComparison(base_file, comparison_file)
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
