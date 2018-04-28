#[derive(Debug)]
pub enum DedupeTask {
    SingleFile(String),
    FileComparison(String, String)
}

#[derive(Debug)]
pub struct Entry {
    pub id: u64,
    pub row: Vec<String>
}

#[derive(Debug)]
pub struct CsvData {
    pub headers: Vec<String>,
    pub entries: Vec<Entry>
}