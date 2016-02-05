use models::Person;
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
            // Last Name | First Name | Company | Phone Number
            let (last_name, first_name, company, phone_number):
                (Option<String>, Option<String>, Option<String>, Option<String>) = record.unwrap();

            self.last_id_created += 1;

            let person = Person::new(
                self.last_id_created,
                first_name,
                last_name,
                company,
                phone_number
            );

            list.push(person);
        }

        list
    }

    pub fn write_to_disk(&self, file: &str, list: Vec<Person>) -> String {
        let mut new_file: String = {
            if file.contains(".") {
                file.split(".").next().unwrap()
            }
            else { file }
        }.to_owned();

        new_file.push_str("-DUPLICATE-FLAG.csv");

        let mut writer = csv::Writer::from_file(&new_file).unwrap();
        writer.encode(("Last Name", "First Name", "Company", "Phone Number", "Duplicate")).unwrap();

        for person in list {
            writer.encode(
                (person.last_name, person.first_name, person.company, person.phone_number, person.is_duplicate)
            ).unwrap();
        }

        new_file
    }
}
