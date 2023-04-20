#![allow(dead_code)]

#[derive(Debug)]
pub struct RuntimeError {
    msg: String
}

impl RuntimeError {

    pub fn new(msg: &str) -> Self {
        RuntimeError { msg: msg.to_string() }
    }

    pub fn get_msg(&self) -> String {
        format!("[RuntimeError] {}", &self.msg)
    }
}
