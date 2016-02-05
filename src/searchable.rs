use models::Person;
use fst::{Levenshtein, Set, IntoStreamer};
use regex::Regex;
use std::collections::{HashSet, HashMap};

// Uses the fst fuzzy string search in order to get possible value matches,
// and then looks up name matches in name -> id hashmaps
// Needed because fst does not support duplicate entries
pub struct SearchableList<'a > {
    first_name_fuzzy: Set,
    first_name_lookup: HashMap<String, Vec<&'a Person>>,

    last_name_fuzzy: Set,
    last_name_lookup: HashMap<String, Vec<&'a Person>>,

    company_fuzzy: Set,
    company_lookup: HashMap<String, Vec<&'a Person>>,

    phone_fuzzy: Set,
    phone_lookup: HashMap<String, Vec<&'a Person>>
}


impl<'a> SearchableList<'a> {
    pub fn new(base_list: &'a Vec<Person>) -> SearchableList<'a> {
        fn insert_item<'a>(item: &str, person: &'a Person, set: &mut HashSet<String>, map: &mut HashMap<String, Vec<&'a Person>>) {
            set.insert(item.to_owned());

            if !map.contains_key(item) {
                map.insert(item.to_owned(), Vec::new());
            }

            map.get_mut(item).unwrap().push(person);
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
                insert_item(&key, person, &mut first_name_fuzzy, &mut first_name_lookup);
            }

            if let Some(ref last_name) = person.last_name {
                let key = sanatize_name(last_name);
                insert_item(&key, person, &mut last_name_fuzzy, &mut last_name_lookup);
            }

            if let Some(ref company) = person.company {
                let key = sanatize_company(company);
                insert_item(&key, person, &mut company_fuzzy, &mut company_lookup);
            }

            if let Some(ref phone_number) = person.phone_number {
                let key = sanatize_phone(phone_number);
                insert_item(&key, person, &mut phone_fuzzy, &mut phone_lookup);
            }
        }

        // sort prior to insertion as needed by fst
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
            first_name_lookup: first_name_lookup,

            last_name_fuzzy: last_name_set,
            last_name_lookup: last_name_lookup,

            company_fuzzy: company_set,
            company_lookup: company_lookup,

            phone_fuzzy: phone_set,
            phone_lookup: phone_lookup
        }
    }

    pub fn get_first_name_matches(&self, first_name: &str) -> Vec<&'a Person> {
        self.get_matches(&self.first_name_fuzzy, &self.first_name_lookup, &sanatize_name(first_name))
    }

    pub fn get_last_name_matches(&self, last_name: &str) -> Vec<&'a Person> {
        self.get_matches(&self.last_name_fuzzy, &self.last_name_lookup, &sanatize_name(last_name))
    }

    pub fn get_companies_matches(&self, company: &str) -> Vec<&'a Person> {
        self.get_matches(&self.company_fuzzy, &self.company_lookup, &sanatize_company(company))
    }

    pub fn get_phone_numbers_matches(&self, phone_number: &str) -> Vec<&'a Person> {
        self.get_matches(&self.phone_fuzzy, &self.phone_lookup, &sanatize_phone(phone_number))
    }

    fn get_matches(&self, set: &Set, map: &HashMap<String, Vec<&'a Person>>, item: &str) -> Vec<&'a Person> {
        let lev = Levenshtein::new(item, 2).unwrap();
        let stream = set.search(lev).into_stream();

        let raw_names = stream.into_strs().unwrap();
        let mut result = Vec::new();

        for name in raw_names.into_iter() {
            let people = map.get(&name).unwrap();
            for person in people {
                result.push(*person);
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