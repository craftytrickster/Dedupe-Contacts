use std::rc::Rc;

#[derive(Debug)]
pub enum DedupeTask {
    SingleFile(String),
    FileComparison(String, String),
}

#[derive(Debug, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug)]
pub struct Entry {
    pub location: Location,
    pub row: Vec<String>,
}

#[derive(Debug)]
pub struct CsvData {
    pub headers: Vec<String>,
    pub entries: Vec<Rc<Entry>>,
}

impl Location {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Location {
            latitude,
            longitude,
        }
    }
}
