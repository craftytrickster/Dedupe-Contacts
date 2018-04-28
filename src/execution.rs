use models::{DedupeTask, Entry, CsvData};
use std::collections::{HashSet, HashMap};
use searchable::SearchableList;
use file::FileUtil;
use interface::display_execution_progress;
use std::error::Error;

pub fn run(task: DedupeTask) -> Result<String, Box<Error>> {
    let mut file_util = FileUtil::new();

    match task {
        DedupeTask::SingleFile(file) => {
            let file_data = file_util.file_to_data(&file)?;
            let searchable_base = SearchableList::new(&file_data.entries);

            let duplicate_ids = get_duplicate_ids_against_base(&searchable_base, &file_data);
            file_util.write_to_disk(&file, &file_data, duplicate_ids)
        },
        DedupeTask::FileComparison(base_file, comparison_file) => {
            let base_data = file_util.file_to_data(&base_file)?;
            let searchable_base = SearchableList::new(&base_data.entries);
            let comparison_data = file_util.file_to_data(&comparison_file)?;

            let duplicate_ids = get_duplicate_ids_against_base(&searchable_base, &comparison_data);
            file_util.write_to_disk(&comparison_file, &comparison_data, duplicate_ids)
        }
    }
}

// go through items, if there are at least two field matches, then flag as possible duplicate
fn get_duplicate_ids_against_base<'a>(searchable_base: &'a SearchableList<'a>, comparison_data: &CsvData) -> HashMap<u64, Vec<&'a Entry>> {
    let mut confirmed_duplicates = HashMap::new();

    let total = comparison_data.entries.len();

    for (i, entry) in comparison_data.entries.iter().enumerate() {
        display_execution_progress(i, total);

        let mut matches = Vec::new();

        if let Some(first_name) = entry.row.get(0) {
            let results = searchable_base.get_first_name_matches(first_name);
            matches.extend_from_slice(&results);
        }

        if let Some(last_name) = entry.row.get(1) {
            let results = searchable_base.get_last_name_matches(last_name);
            matches.extend_from_slice(&results);
        }

        if let Some(company) = entry.row.get(2) {
            let results = searchable_base.get_companies_matches(company);
            matches.extend_from_slice(&results);
        }

        if let Some(phone_number) = entry.row.get(3) {
            let results = searchable_base.get_phone_numbers_matches(phone_number);
            matches.extend_from_slice(&results);
        }

        let mut id_matches = HashSet::new();
        let mut already_added_duplicates = HashSet::new();
        for matched_entry in matches.into_iter() {
            if matched_entry.id == entry.id {
                // we do not care about the entry matching themselves
                continue;
            }

            if id_matches.contains(&matched_entry.id) {
                if !confirmed_duplicates.contains_key(&entry.id) {
                    confirmed_duplicates.insert(entry.id, Vec::new());
                }

                if !already_added_duplicates.contains(&matched_entry.id) {
                    confirmed_duplicates.get_mut(&entry.id).unwrap().push(matched_entry);
                }

                already_added_duplicates.insert(matched_entry.id);
            }

            id_matches.insert(matched_entry.id);
        }
    }
    confirmed_duplicates
}
