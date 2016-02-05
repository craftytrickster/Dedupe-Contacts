use models::Person;
use fst::{Levenshtein, Set, IntoStreamer};
use regex::Regex;
use std::collections::{HashSet, HashMap};

// Uses the fst fuzzy string search in order to get possible value matches,
// and then looks up name matches in name -> id hashmaps
// Needed because fst does not support duplicate entries
pub struct SearchableList {
    first_name_fuzzy: Set,
    first_name_ids: HashMap<String, Vec<u64>>,

    last_name_fuzzy: Set,
    last_name_ids: HashMap<String, Vec<u64>>,

    company_fuzzy: Set,
    company_ids: HashMap<String, Vec<u64>>,

    phone_fuzzy: Set,
    phone_ids: HashMap<String, Vec<u64>>
}

impl SearchableList {
    pub fn new(base_list: &Vec<Person>) -> SearchableList {
        fn insert_item(item: &str, id: u64, set: &mut HashSet<String>, map: &mut HashMap<String, Vec<u64>>) {
            set.insert(item.to_owned());

            if !map.contains_key(item) {
                map.insert(item.to_owned(), Vec::new());
            }

            map.get_mut(item).unwrap().push(id);
        }

        let mut first_name_fuzzy = HashSet::new();
        let mut first_name_ids = HashMap::new();

        let mut last_name_fuzzy = HashSet::new();
        let mut last_name_ids = HashMap::new();

        let mut company_fuzzy = HashSet::new();
        let mut company_ids = HashMap::new();

        let mut phone_fuzzy = HashSet::new();
        let mut phone_ids = HashMap::new();

        for person in base_list {
            if let Some(ref first_name) = person.first_name {
                let key = sanatize_name(first_name);
                insert_item(&key, person.id, &mut first_name_fuzzy, &mut first_name_ids);
            }

            if let Some(ref last_name) = person.last_name {
                let key = sanatize_name(last_name);
                insert_item(&key, person.id, &mut last_name_fuzzy, &mut last_name_ids);
            }

            if let Some(ref company) = person.company {
                let key = sanatize_company(company);
                insert_item(&key, person.id, &mut company_fuzzy, &mut company_ids);
            }

            if let Some(ref phone_number) = person.phone_number {
                let key = sanatize_phone(phone_number);
                insert_item(&key, person.id, &mut phone_fuzzy, &mut phone_ids);
            }
        }

        // sort prior to insertion
        let mut sorted_first = first_name_fuzzy.drain().collect::<Vec<String>>();
        sorted_first.sort();
        let mut sorted_last = last_name_fuzzy.drain().collect::<Vec<String>>();
        sorted_last.sort();
        let mut sorted_company = company_fuzzy.drain().collect::<Vec<String>>();
        sorted_company.sort();
        let mut sorted_phone = phone_fuzzy.drain().collect::<Vec<String>>();
        sorted_phone.sort();

        let first_name_set = Set::from_iter(sorted_first).unwrap();
        let last_name_set = Set::from_iter(sorted_last).unwrap();
        let company_set = Set::from_iter(sorted_company).unwrap();
        let phone_set = Set::from_iter(sorted_phone).unwrap();

        SearchableList {
            first_name_fuzzy: first_name_set,
            first_name_ids: first_name_ids,

            last_name_fuzzy: last_name_set,
            last_name_ids: last_name_ids,

            company_fuzzy: company_set,
            company_ids: company_ids,

            phone_fuzzy: phone_set,
            phone_ids: phone_ids
        }
    }

    pub fn get_first_name_matches(&self, first_name: &str) -> Vec<u64> {
        self.get_matches(&self.first_name_fuzzy, &self.first_name_ids, &sanatize_name(first_name))
    }

    pub fn get_last_name_matches(&self, last_name: &str) -> Vec<u64> {
        self.get_matches(&self.last_name_fuzzy, &self.last_name_ids, &sanatize_name(last_name))
    }

    pub fn get_companies_matches(&self, company: &str) -> Vec<u64> {
        self.get_matches(&self.company_fuzzy, &self.company_ids, &sanatize_company(company))
    }

    pub fn get_phone_numbers_matches(&self, phone_number: &str) -> Vec<u64> {
        self.get_matches(&self.phone_fuzzy, &self.phone_ids, &sanatize_phone(phone_number))
    }

    fn get_matches(&self, set: &Set, map: &HashMap<String, Vec<u64>>, item: &str) -> Vec<u64> {
        let lev = Levenshtein::new(item, 2).unwrap();
        let stream = set.search(lev).into_stream();

        let raw_names = stream.into_strs().unwrap();
        let mut result = Vec::new();

        for name in raw_names.into_iter() {
            let ids = map.get(&name).unwrap();
            for id in ids.clone() {
                result.push(id);
            }
        }

        result
    }
}

fn sanatize_name(name: &str) -> String {
    let re = Regex::new("-|'| ").unwrap();
    re.replace(name, "")
}

fn sanatize_company(company: &str) -> String {
    let re = Regex::new(r"-|'|\.| ").unwrap();
    re.replace(company, "")
}

fn sanatize_phone(phone: &str) -> String {
    let re = Regex::new(r"-|\.| ").unwrap();
    re.replace(phone, "")
}
