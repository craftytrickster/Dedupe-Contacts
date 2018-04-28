pub enum DedupeTask {
    SingleFile(String),
    FileComparison(String, String)
}

pub struct Entry {
    pub id: u64,
    pub row: Vec<String>
}

pub struct CsvData {
    pub headers: Vec<String>,
    pub entries: Vec<Entry>
}