pub enum DedupeTask {
    SingleFile(String),
    FileComparison(String, String)
}

pub struct Person {
    pub id: u64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company: Option<String>,
    pub phone_number: Option<String>,
}
