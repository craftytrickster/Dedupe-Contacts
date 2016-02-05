use models::{DedupeTask, Person};
use std::collections::HashSet;
use searchable::SearchableList;
use file::FileUtil;

pub fn run(task: DedupeTask) -> String {
    let mut file_util = FileUtil::new();

    match task {
        DedupeTask::SingleFile(file) => {
            let mut single_list = file_util.file_to_list(&file);
            let searchable_base = SearchableList::new(&single_list);

            mark_duplicates_against_base(&searchable_base, &mut single_list);
            file_util.write_to_disk(&file, single_list)
        },
        DedupeTask::FileComparison(base_file, comparison_file) => {
            let base_list = file_util.file_to_list(&base_file);
            let searchable_base = SearchableList::new(&base_list);
            let mut comparison_list = file_util.file_to_list(&comparison_file);

            mark_duplicates_against_base(&searchable_base, &mut comparison_list);
            file_util.write_to_disk(&comparison_file, comparison_list)
        }
    }
}

// go through items, if there are at least two field matches, then flag as possible duplicate
fn mark_duplicates_against_base(searchable_base: &SearchableList, comparison_list: &mut Vec<Person>) {
    for person in comparison_list {
        let mut ids = Vec::new();

        if let Some(ref first_name) = person.first_name {
            let results = searchable_base.get_first_name_matches(first_name);
            ids.extend_from_slice(&results);
        }

        if let Some(ref last_name) = person.last_name {
            let results = searchable_base.get_last_name_matches(last_name);
            ids.extend_from_slice(&results);
        }

        if let Some(ref company) = person.company {
            let results = searchable_base.get_companies_matches(company);
            ids.extend_from_slice(&results);
        }

        if let Some(ref phone_number) = person.phone_number {
            let results = searchable_base.get_phone_numbers_matches(phone_number);
            ids.extend_from_slice(&results);
        }

        let mut id_matches = HashSet::new();
        let mut is_duplicate = false;
        // we do not care about the person matching themselves
        for id in ids.into_iter().filter(|id| *id != person.id) {
            if id_matches.contains(&id) {
                is_duplicate = true;
                break;
            }

            id_matches.insert(id);
        }

        person.is_duplicate = is_duplicate;
    }
}
