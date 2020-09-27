use crate::models::{CsvData, Entry};
use csv::{Reader, Writer};
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

pub struct FileUtil {
    last_id_created: u64,
}

impl FileUtil {
    pub fn new() -> FileUtil {
        FileUtil { last_id_created: 0 }
    }

    pub fn file_to_data(&mut self, file: &str) -> Result<CsvData, Box<dyn Error>> {
        let mut entries = Vec::new();

        let mut rdr = Reader::from_file(file)?;

        let headers: Vec<String> = rdr.headers()?;

        for record in rdr.decode() {
            if let Ok(record) = record {
                let record: Vec<Option<String>> = record;
                let row: Vec<String> = record.into_iter().map(|x| x.unwrap_or_default()).collect();

                self.last_id_created += 1;

                let entry = Rc::new(Entry {
                    id: self.last_id_created,
                    row,
                });

                entries.push(entry);
            }
        }

        Ok(CsvData { headers, entries })
    }

    pub fn write_to_disk<'a>(
        &self,
        file: &str,
        data: &'a CsvData,
        duplicate_ids: HashMap<u64, Vec<Rc<Entry>>>,
    ) -> Result<String, Box<dyn Error>> {
        let mut new_file: String = {
            if file.ends_with(".csv") {
                &file[0..file.len() - 4] // truncate csv
            } else {
                file
            }
        }
        .to_owned();

        new_file.push_str("-DUPLICATE-FLAG.csv");

        let mut writer = Writer::from_file(&new_file)?;
        let mut new_headers = data.headers.clone();
        new_headers.push(String::from("Duplicate"));

        writer.encode(&new_headers)?;

        for entry in &data.entries {
            let duplicate_string = get_duplicate_string(&entry.id, &duplicate_ids);
            let mut data = entry.row.clone();
            data.push(duplicate_string);
            writer.encode(&data)?;
        }

        Ok(new_file)
    }
}

fn get_duplicate_string<'a>(id: &u64, duplicate_ids: &HashMap<u64, Vec<Rc<Entry>>>) -> String {
    let mut result = String::new();

    if let Some(matches) = duplicate_ids.get(id) {
        for (i, entry) in matches.iter().enumerate() {
            if i > 5 {
                // after displaying several matches, no need to show all
                result.push_str(&format!("and {} more ...", matches.len() - i));
                return result;
            }

            if entry.row.len() == 1 {
                let first = &entry.row[0];

                result.push_str(&format!("{} | ", first));
            } else {
                let first = &entry.row[0];
                let second = &entry.row[1];

                result.push_str(&format!("{}, {} | ", first, second));
            }
        }

        // remove trailing chars " | "
        result.pop();
        result.pop();
        result.pop();
    }

    result
}
