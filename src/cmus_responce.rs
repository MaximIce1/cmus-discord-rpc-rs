use std::collections::HashMap;

pub struct Responce {
    pub status: String,
    pub file: String,
    pub duration: u32,
    pub position: u32,
    pub tags: HashMap<String, String> = HashMap::new(),
}  

impl Responce {
    fn parseResponce () {

    }
}