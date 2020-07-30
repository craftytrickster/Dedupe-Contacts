use crate::models::{CsvData, Entry};
use fst::{Set, IntoStreamer};
use fst_levenshtein::Levenshtein;
use regex::Regex;
use std::collections::BTreeMap;

// Uses the fst fuzzy string search in order to get possible value matches,
// and then looks up name matches in name -> Person BTreeMaps
// Needed because fst does not support duplicate entries
pub struct FuzzyMap<'a, T: 'a> {
    set: Set,
    map: BTreeMap<String, Vec<&'a T>>
}

impl<'a, T> FuzzyMap<'a, T> {
    pub fn new(map: BTreeMap<String, Vec<&'a T>>) -> Self {
        let set = Set::from_iter(map.iter().map(|i| i.0)).unwrap();

        FuzzyMap { set, map }
    }

    // a distance of more than 2 seems to break things
    pub fn get_matches(&self, item: &str, distance: u32) -> Vec<&'a T> {
        let mut result = Vec::new();

        if item.is_empty() {
            return result;
        }

        let lev = Levenshtein::new(item, distance).unwrap();

        let stream = self.set.search(lev).into_stream();

        let raw_names = stream.into_strs().unwrap();

        for name in raw_names.into_iter() {
            let entries = self.map.get(&name).unwrap();
            for entry in entries {
                result.push(*entry);
            }
        }

        result
    }
}

pub struct SearchableList<'a> {
    fuzzy_data_per_column: Vec<FuzzyMap<'a, Entry>>
}


impl<'a> SearchableList<'a> {
    pub fn new(csv_data: &'a CsvData) -> SearchableList<'a> {
        let mut map_per_column_list = Vec::new();
        for _ in &csv_data.headers {
            map_per_column_list.push(BTreeMap::new());
        }

        for entry in &csv_data.entries {
            for (map, col_item) in map_per_column_list.iter_mut().zip(entry.row.iter()) {
                let key = sanatize(col_item);
                if !key.is_empty() {
                    let list = map.entry(key).or_insert_with(Vec::new);
                    list.push(entry);
                }
            }
        }

        SearchableList {
            fuzzy_data_per_column: map_per_column_list.into_iter().map(FuzzyMap::new).collect()
        }
    }

    pub fn get_entry_matches(&self, entry: &Entry) -> Vec<&'a Entry> {
        let mut matches = Vec::new();

        for (fuzz_map, col_item) in self.fuzzy_data_per_column.iter().zip(entry.row.iter()) {
            let key = sanatize(col_item);

            // TODO: In future, actually make distance configurable,
            // all callers just hardcode 2 right now
            let results = fuzz_map.get_matches(&key, 2);
            matches.extend_from_slice(&results);
        }

        matches
    }
}

lazy_static! {
    static ref RE_SANATIZE: Regex = Regex::new("[^A-Za-z0-9]").unwrap();
}

// avoid overly long strings
fn truncate(input: &str) -> &str {
    let max_len = if input.len() > 25 { 25 } else { input.len() };

    &input[0..max_len]
}

fn sanatize(name: &str) -> String {
    RE_SANATIZE.replace_all(truncate(name), "").to_lowercase()
}
