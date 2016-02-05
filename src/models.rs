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
    pub is_duplicate: bool
}

impl Person  {
    pub fn new(id: u64,
           first_name: Option<String>,
           last_name: Option<String>,
           company: Option<String>,
           phone_number: Option<String>) -> Person {

        Person {
            id: id,
            first_name: first_name,
            last_name: last_name,
            company: company,
            phone_number: phone_number,
            is_duplicate: false
        }
    }
}
