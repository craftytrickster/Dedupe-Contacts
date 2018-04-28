use models::Person;
use fst::{Set, IntoStreamer};
use fst_levenshtein::Levenshtein;
use regex::Regex;
use std::collections::{HashSet, HashMap};

// Uses the fst fuzzy string search in order to get possible value matches,
// and then looks up name matches in name -> Person hashmaps
// Needed because fst does not support duplicate entries
pub struct FuzzyMap<'a, T: 'a> {
    set: Set,
    hashmap: HashMap<String, Vec<&'a T>>
}

impl<'a, T> FuzzyMap<'a, T> {
    pub fn new(set: Set, hashmap: HashMap<String, Vec<&'a T>>) -> Self {
        FuzzyMap { set, hashmap }
    }

    // a distance of more than 2 seems to break things
    pub fn get_matches(&self, item: &str, distance: u32) -> Vec<&'a T> {
        let mut result = Vec::new();

        if item.is_empty() {
            return result;
        }

        let lev = Levenshtein::new(item, distance).unwrap();
    //
        let stream = self.set.search(lev).into_stream();

        let raw_names = stream.into_strs().unwrap();

        for name in raw_names.into_iter() {
            let people = self.hashmap.get(&name).unwrap();
            for person in people {
                result.push(*person);
            }
        }

        result
    }
}

pub struct SearchableList<'a> {
    first_name_map: FuzzyMap<'a, Person>,
    last_name_map: FuzzyMap<'a, Person>,
    company_map: FuzzyMap<'a, Person>,
    phone_map: FuzzyMap<'a, Person>
}


impl<'a> SearchableList<'a> {
    pub fn new(base_list: &'a Vec<Person>) -> SearchableList<'a> {
        fn insert_item<'a>(item: String, person: &'a Person, set: &mut HashSet<String>, map: &mut HashMap<String, Vec<&'a Person>>) {
            if item.is_empty() {
                return; // avoid storing records for blank items
            }

            set.insert(item.clone());

            let list = map.entry(item).or_insert(Vec::new());
            list.push(person);
        }

        let mut first_name_fuzzy = HashSet::new();
        let mut first_name_lookup = HashMap::new();

        let mut last_name_fuzzy = HashSet::new();
        let mut last_name_lookup = HashMap::new();

        let mut company_fuzzy = HashSet::new();
        let mut company_lookup = HashMap::new();

        let mut phone_fuzzy = HashSet::new();
        let mut phone_lookup = HashMap::new();

        for person in base_list {
            if let Some(ref first_name) = person.first_name {
                let key = sanatize_name(first_name);
                insert_item(key, person, &mut first_name_fuzzy, &mut first_name_lookup);
            }

            if let Some(ref last_name) = person.last_name {
                let key = sanatize_name(last_name);
                insert_item(key, person, &mut last_name_fuzzy, &mut last_name_lookup);
            }

            if let Some(ref company) = person.company {
                let key = sanatize_company(company);
                insert_item(key, person, &mut company_fuzzy, &mut company_lookup);
            }

            if let Some(ref phone_number) = person.phone_number {
                let key = sanatize_phone(phone_number);
                insert_item(key, person, &mut phone_fuzzy, &mut phone_lookup);
            }
        }

        // sort prior to insertion as needed by fst
        let mut sorted_first = first_name_fuzzy.drain().collect::<Vec<_>>();
        sorted_first.sort();
        let mut sorted_last = last_name_fuzzy.drain().collect::<Vec<_>>();
        sorted_last.sort();
        let mut sorted_company = company_fuzzy.drain().collect::<Vec<_>>();
        sorted_company.sort();
        let mut sorted_phone = phone_fuzzy.drain().collect::<Vec<_>>();
        sorted_phone.sort();

        let first_name_set = Set::from_iter(sorted_first).unwrap();
        let last_name_set = Set::from_iter(sorted_last).unwrap();
        let company_set = Set::from_iter(sorted_company).unwrap();
        let phone_set = Set::from_iter(sorted_phone).unwrap();

        SearchableList {
            first_name_map: FuzzyMap::new(first_name_set, first_name_lookup),
            last_name_map: FuzzyMap::new(last_name_set, last_name_lookup),
            company_map: FuzzyMap::new(company_set, company_lookup),
            phone_map: FuzzyMap::new(phone_set, phone_lookup) 
        }
    }

    pub fn get_first_name_matches(&'a self, first_name: &str) -> Vec<&'a Person> {
        self.first_name_map.get_matches(&sanatize_name(first_name), 2)
    }

    pub fn get_last_name_matches(&self, last_name: &str) -> Vec<&'a Person> {
        self.last_name_map.get_matches(&sanatize_name(last_name), 2)
    }

    pub fn get_companies_matches(&self, company: &str) -> Vec<&'a Person> {
        self.company_map.get_matches(&sanatize_company(company), 2)
    }

    pub fn get_phone_numbers_matches(&self, phone_number: &str) -> Vec<&'a Person> {
        self.phone_map.get_matches(&sanatize_phone(phone_number), 2)
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

fn sanatize_name(name: &str) -> String {
    RE_SANATIZE.replace_all(truncate(name), "").to_lowercase()
}

fn sanatize_company(company: &str) -> String {
    RE_SANATIZE.replace_all(truncate(company), "").to_lowercase()
}

fn sanatize_phone(phone: &str) -> String {
    RE_SANATIZE.replace_all(truncate(phone), "").to_lowercase()
}
