#![allow(dead_code)]

pub type LocType = usize;

#[derive(Clone)]
pub struct Location {
    pub file_name: String,
    pub line_id: LocType,
    pub word_id: LocType
}

impl Location {
    
    pub fn new(file_name: String, line_id: LocType, word_id: LocType) -> Self {
        Location{ file_name, line_id, word_id }
    }
}

pub struct CompilerError {
    msg: String,
    location: Location
}

impl CompilerError {

    pub fn new(msg: &str, location: &Location) -> Self {
        CompilerError { 
            msg: msg.to_string(), 
            location: Location::new(location.file_name.clone(), location.line_id, location.word_id) 
        }
    }

    pub fn get_msg(&self) -> String {
        format!("[CompilerError] {}:{}:{} => {}", self.location.file_name, self.location.line_id, self.location.word_id, self.msg)
    }
}
