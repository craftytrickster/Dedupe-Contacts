use crate::file::FileUtil;
use crate::interface::display_execution_progress;
use crate::models::{CsvData, DedupeTask, Entry};
use crate::searchable::SearchableList;
use std::collections::{HashMap, HashSet};
use std::error::Error;

pub fn run(task: DedupeTask) -> Result<String, Box<dyn Error>> {
    let mut file_util = FileUtil::new();

    match task {
        DedupeTask::SingleFile(file) => {
            let file_data = file_util.file_to_data(&file)?;
            let searchable_base = SearchableList::new(&file_data);

            let duplicate_ids = get_duplicate_ids_against_base(&searchable_base, &file_data);
            file_util.write_to_disk(&file, &file_data, duplicate_ids)
        }
        DedupeTask::FileComparison(base_file, comparison_file) => {
            let base_data = file_util.file_to_data(&base_file)?;
            let searchable_base = SearchableList::new(&base_data);
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
fn get_duplicate_ids_against_base<'a>(
    searchable_base: &'a SearchableList<'a>,
    comparison_data: &CsvData,
) -> HashMap<u64, Vec<&'a Entry>> {
    let mut confirmed_duplicates = HashMap::new();

    let total = comparison_data.entries.len();
    let single_column = comparison_data.headers.len() == 1;

    for (i, entry) in comparison_data.entries.iter().enumerate() {
        display_execution_progress(i, total);

        let matches = searchable_base.get_entry_matches(entry);

        let mut id_matches = HashSet::new();
        let mut already_added_duplicates = HashSet::new();
        for matched_entry in matches.into_iter() {
            if matched_entry.id == entry.id {
                // we do not care about the entry matching themselves
                continue;
            }

            if single_column || id_matches.contains(&matched_entry.id) {
                if single_column || !already_added_duplicates.contains(&matched_entry.id) {
                    let confirmed_list = confirmed_duplicates
                        .entry(entry.id)
                        .or_insert_with(Vec::new);
                    confirmed_list.push(matched_entry);
                }

                already_added_duplicates.insert(matched_entry.id);
            }

            id_matches.insert(matched_entry.id);
        }
    }
    confirmed_duplicates
}
