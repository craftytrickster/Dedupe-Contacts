use models::{DedupeTask, Person};
use std::collections::{HashSet, HashMap};
use searchable::SearchableList;
use file::FileUtil;
use interface::display_execution_progress;
use std::error::Error;

pub fn run(task: DedupeTask) -> Result<String, Box<Error>> {
    let mut file_util = FileUtil::new();

    match task {
        DedupeTask::SingleFile(file) => {
            let single_list = file_util.file_to_list(&file)?;
            let searchable_base = SearchableList::new(&single_list);

            let duplicate_ids = get_duplicate_ids_against_base(&searchable_base, &single_list);
            file_util.write_to_disk(&file, &single_list, duplicate_ids)
        },
        DedupeTask::FileComparison(base_file, comparison_file) => {
            let base_list = file_util.file_to_list(&base_file)?;
            let searchable_base = SearchableList::new(&base_list);
            let comparison_list = file_util.file_to_list(&comparison_file)?;

            let duplicate_ids = get_duplicate_ids_against_base(&searchable_base, &comparison_list);
            file_util.write_to_disk(&comparison_file, &comparison_list, duplicate_ids)
        }
    }
}

// go through items, if there are at least two field matches, then flag as possible duplicate
fn get_duplicate_ids_against_base<'a>(searchable_base: &'a SearchableList<'a>, comparison_list: &Vec<Person>) -> HashMap<u64, Vec<&'a Person>> {
    let mut confirmed_duplicates = HashMap::new();

    for (i, person) in comparison_list.iter().enumerate() {
        display_execution_progress(i, comparison_list.len());

        let mut matches = Vec::new();

        if let Some(ref first_name) = person.first_name {
            let results = searchable_base.get_first_name_matches(first_name);
            matches.extend_from_slice(&results);
        }

        if let Some(ref last_name) = person.last_name {
            let results = searchable_base.get_last_name_matches(last_name);
            matches.extend_from_slice(&results);
        }

        if let Some(ref company) = person.company {
            let results = searchable_base.get_companies_matches(company);
            matches.extend_from_slice(&results);
        }

        if let Some(ref phone_number) = person.phone_number {
            let results = searchable_base.get_phone_numbers_matches(phone_number);
            matches.extend_from_slice(&results);
        }

        let mut id_matches = HashSet::new();
        let mut already_added_duplicates = HashSet::new();
        for matched_person in matches.into_iter() {
            if matched_person.id == person.id {
                // we do not care about the person matching themselves
                continue;
            }

            if id_matches.contains(&matched_person.id) {
                if !confirmed_duplicates.contains_key(&person.id) {
                    confirmed_duplicates.insert(person.id, Vec::new());
                }

                if !already_added_duplicates.contains(&matched_person.id) {
                    confirmed_duplicates.get_mut(&person.id).unwrap().push(matched_person);
                }

                already_added_duplicates.insert(matched_person.id);
            }

            id_matches.insert(matched_person.id);
        }
    }
    confirmed_duplicates
}
