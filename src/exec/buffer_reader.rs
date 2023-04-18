#![allow(dead_code)]

use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

use super::error::RuntimeError;

pub struct BufferReader {
    buffer: Vec<String>,
    buf_pointer: usize
}

impl BufferReader {
    
    pub fn new(file_name: String) -> Result<Self, RuntimeError> {
        let mut buffer: Vec<String> = Vec::new();
        let mut bufin: String = String::new();
        let buf_pointer: usize = 0;
        let path = Path::new(&file_name);
        let path_display = path.display();
        let mut file = match File::open(&path) {
            Ok(file) => { file }, 
            Err(why) => { return Err(RuntimeError::new(&format!("cannot open {} due to {}", path_display, why))); }
        };
        match file.read_to_string(&mut bufin) {
            Ok(_) => {},
            Err(why) => { return Err(RuntimeError::new(&format!("cannot read file to virtual machine buffer duo to {}", why))); }
        }
        bufin = bufin.replace("\r", "");

        {
            let lines: Vec<&str> = bufin.trim().split('\n').collect();

            for line in lines.iter() {
                let tokens: Vec<&str> = line.split(' ').collect();

                for token in tokens.iter() {
                    buffer.push(token.to_string());
                }
            }
        }

        Ok(BufferReader { buffer, buf_pointer })
    }

    pub fn read_cmd(&mut self) -> Result<&str, RuntimeError> {
        if self.buffer.len() <= self.buf_pointer {
            return Err(RuntimeError::new("Internal RuntimeError: buffer out of empty before virtual machine closes. may cause by self.move_reader()"));
        }

        self.buf_pointer += 1;

        Ok(&self.buffer[self.buf_pointer])
    }

    pub fn read_token(&mut self) -> Result<&str, RuntimeError> { self.read_cmd() }

    pub fn read_arg<T>(&mut self) -> Result<T, RuntimeError> where T: FromStr {
        let res: T = match self.read_cmd()?.parse() {
            Ok(v) => { v }
            Err(_) => { return Err(RuntimeError::new("Internal RuntimeError: cannot convert this token to type T")); }
        };

        Ok(res)
    }

    pub fn get_reader_index(&self) -> usize { self.buf_pointer }
    pub fn move_reader(&mut self, position: usize) { self.buf_pointer = position }
}
