use crate::file::FileUtil;
use crate::interface::display_execution_progress;
use crate::models::{CsvData, DedupeTask, Entry};
use crate::searchable::LocationMatcher;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

pub fn run(task: DedupeTask) -> Result<String, Box<dyn Error>> {
    let mut file_util = FileUtil::new();

    match task {
        DedupeTask::SingleFile(file) => {
            let file_data = file_util.file_to_data(&file)?;
            let searchable_base = LocationMatcher::new(&file_data.entries);

            let duplicate_ids = get_duplicate_ids_against_base(&searchable_base, &file_data);
            file_util.write_to_disk(&file, &file_data, duplicate_ids)
        }
        DedupeTask::FileComparison(base_file, comparison_file) => {
            let base_data = file_util.file_to_data(&base_file)?;
            let searchable_base = LocationMatcher::new(&base_data.entries);
            let comparison_data = file_util.file_to_data(&comparison_file)?;

            if base_data.headers.len() != comparison_data.headers.len() {
                panic!("The two specified csv files must have the same amount of columns");
            }

            let duplicate_ids = get_duplicate_ids_against_base(&searchable_base, &comparison_data);
            file_util.write_to_disk(&comparison_file, &comparison_data, duplicate_ids)
        }
    }
}

// go through items, if there are at least two field matches, then flag as possible duplicate
// unless there is only one column, in which case, we only need one match
fn get_duplicate_ids_against_base(
    searchable_base: &LocationMatcher,
    comparison_data: &CsvData,
) -> HashMap<usize, Vec<Rc<Entry>>> {
    let mut confirmed_duplicates = HashMap::new();

    let total = comparison_data.entries.len();

    for (i, entry) in comparison_data.entries.iter().enumerate() {
        display_execution_progress(i, total);

        let matches = searchable_base.find_matches(&entry.location, 0.0007);
        if !matches.is_empty() {
            if matches.len() == 1 && Rc::ptr_eq(&matches[0], entry) {
                // we do not care about the entry matching themselves
                continue;
            }

            confirmed_duplicates.insert(i, matches);
        }
    }
    confirmed_duplicates
}
