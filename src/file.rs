use models::Person;
use std::collections::HashMap;
use csv;

pub struct FileUtil {
    last_id_created: u64
}

impl FileUtil {
    pub fn new() -> FileUtil {
        FileUtil { last_id_created: 0 }
    }

    pub fn file_to_list(&mut self, file: &str) -> Vec<Person> {
        let mut list = Vec::new();

        let mut rdr = csv::Reader::from_file(file).unwrap();

        for record in rdr.decode() {
            match record {
                Err(e) => { println!("Error trying to parse illegal text row - {}", e); },
                Ok(record) => {
                    let (last_name, first_name, company, phone_number):
                        (Option<String>, Option<String>, Option<String>, Option<String>) = record;

                        self.last_id_created += 1;

                        let person = Person {
                            id: self.last_id_created,
                            first_name: first_name,
                            last_name: last_name,
                            company: company,
                            phone_number: phone_number
                        };

                        list.push(person);
                }
            }
        }

        list
    }

    pub fn write_to_disk<'a>(&self, file: &str, list: &'a Vec<Person>, duplicate_ids: HashMap<u64, Vec<&'a Person>>) -> String {
        let mut new_file: String = {
            if file.ends_with(".csv") {
                &file[0..file.len() - 4] // truncate csv
            }
            else { file }
        }.to_owned();

        new_file.push_str("-DUPLICATE-FLAG.csv");

        let mut writer = csv::Writer::from_file(&new_file).unwrap();
        writer.encode(("Last Name", "First Name", "Company", "Phone Number", "Duplicate")).unwrap();

        for person in list {
            let duplicate_string = get_duplicate_string(&person.id, &duplicate_ids);
            writer.encode(
                (&person.last_name, &person.first_name, &person.company, &person.phone_number, duplicate_string)
            ).unwrap();
        }

        new_file
    }
}

fn get_duplicate_string<'a>(id: &u64, duplicate_ids: &HashMap<u64, Vec<&'a Person>>) -> String {
    let mut result = String::new();

    match duplicate_ids.get(id) {
        Some(matches) => {
            for (i, person) in matches.iter().enumerate() {
                if i > 5 { // after displaying several matches, no need to show all
                    result.push_str(&format!("and {} more ...", matches.len() - i));
                    return result;
                }

                let last_name = person.last_name.clone().unwrap_or(String::new());
                let first_name = person.first_name.clone().unwrap_or(String::new());

                result.push_str(&format!("{}, {} | ", last_name, first_name));
            }

            // remove trailing chars
            result.pop();
            result.pop();
            result.pop();
        }
        None => { result.push_str("false"); }
    }
    result
}
