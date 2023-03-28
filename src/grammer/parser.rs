#![allow(dead_code)]

use std::collections::HashMap;
use std::io::Read;
use std::fs::File;
use std::path::Path;

use crate::grammer::error::CompilerError;
use crate::grammer::error::LocType;
use crate::grammer::error::Location;
use crate::grammer::variable::make_const_variable;
use crate::grammer::variable::Variable;
use crate::grammer::variable::Attribute;

struct Parser {
    location: Location,
    attributes: HashMap<String, Attribute>,
    buffer: String  // the place to store full source code
}

impl Parser {

    pub fn new(file_name: String) -> Result<Self, CompilerError> {
        let mut attributes = HashMap::new();
        let location = Location::new(file_name.clone(), 0, 0);
        let path = Path::new(&file_name);
        let path_display  = path.display();
        let mut file = match File::open(&path) {
            Ok(file) => { file }, 
            Err(why) => { return Err(CompilerError::new(&format!("cannot open {} due to {}", path_display, why), &location)); }
        };
        let mut buffer: String = String::new();
        match file.read_to_string(&mut buffer) {
            Ok(_) => {},
            Err(why) => { return Err(CompilerError::new(&format!("cannot read file to compiler buffer duo to {}", why), &location)); }
        }

        // insert attributes into attributes
        //
        // attribute with prefix "@" means that its non-constant
        // the value in that attribute is nosense
        let attrs: Vec<(&str, Attribute)> = vec![
            ("BEGIN", Attribute::Tag("BEGIN".to_string())),
            ("END", Attribute::Tag("END".to_string())),
            ("@IntRange", Attribute::IntRange((0, 0))),
            ("@FloatRange", Attribute::FloatRange((0_f64, 0_f64))),
            ("@StrSet", Attribute::StrSet("".to_string())),
            ("\\@", Attribute::Except(Box::new(Attribute::Any(())))),
            ("UPC", Attribute::StrSet("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string())),
            ("LOC", Attribute::StrSet("abcdefghijklmnopqrstuvwxyz".to_string())),
            ("SML", Attribute::StrSet("~`!@#$%^&*()_-+=\\|[]{}:;'\"/?.>,<".to_string())),
            ("ALC", Attribute::StrSet("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz~`!@#$%^&*()_-+=\\|[]{}:;'\"/?.>,<".to_string())),
            ("SPACE", Attribute::StrSet(" ".to_string())),
            ("BSL", Attribute::StrSet("\\".to_string())),
            ("LSB", Attribute::StrSet("(".to_string())),
            ("RSB", Attribute::StrSet(")".to_string())),
            ("SEMI", Attribute::StrSet(";".to_string())),
        ];
        for attr in attrs.iter() {
            attributes.insert(attr.0.to_string(), attr.1.clone());
        }

        return Ok(Parser { location, attributes, buffer });
    }

    fn parse_variable(&mut self, code: &str) -> Result<Variable, CompilerError> {
        // use += to operate location.word_id

        let mut result: Variable = Variable::new();
        let mut attr_ranges: Vec<(LocType, LocType)> = Vec::new();

        {
            let mut semi_pos: Vec<LocType> = Vec::new(); // semicolon position -> [1, ..., len())

            semi_pos.push(1);
            for (id, iter) in code.chars().enumerate() {
                if iter == ';' {
                    semi_pos.push(id);
                }
            }
            semi_pos.push(code.len());

            for (id, iter) in semi_pos.iter().enumerate() {
                if id + 1 < semi_pos.len() {
                    if *iter + 1 <= semi_pos[id + 1] {
                        attr_ranges.push((*iter + 1, semi_pos[id + 1]));
                    }
                }
            }
        }

        return Ok(result);
    }

    fn parse_line(&mut self, code: &str) -> Result<Vec<Variable>, CompilerError> {
        let mut result: Vec<Variable> = Vec::new();
        let mut var_bracket_pairs: Vec<(LocType, LocType)> = Vec::new();

        // get all Variable bracket pairs positions
        {
            let mut bracket_stack: Vec<LocType> = Vec::new();

            for (word_id, word) in code.chars().enumerate() {
                if word == '(' {
                    bracket_stack.push(word_id);
                } else if word == ')' {
                    if bracket_stack.is_empty() {
                        self.location.word_id = word_id;
                        return Err(CompilerError::new("Unpaired right bracket", &self.location));
                    } else if bracket_stack.len() == 1 {
                        var_bracket_pairs.push((*bracket_stack.last().unwrap(), word_id));
                    }
                    bracket_stack.pop();
                }
            }
        }

        for (id, iter) in var_bracket_pairs.iter().enumerate() {
            if id == 0 && 0 < iter.0 {
                result.push(make_const_variable(&code[0..iter.0]));
            } else if id == var_bracket_pairs.len() - 1 && iter.1 < code.len() {
                result.push(make_const_variable(&code[iter.1..code.len()]))
            } else {
                if id + 1 < var_bracket_pairs.len() {
                    if iter.1 < var_bracket_pairs[id + 1].0 {
                        result.push(make_const_variable(
                            &code[iter.1..var_bracket_pairs[id + 1].0],
                        ));
                    }
                }
                self.location.word_id = iter.0;
                result.push(match self.parse_variable(&code[iter.0..iter.1 + 1]) {
                    Ok(res) => res,
                    Err(ce) => {
                        return Err(ce);
                    }
                });
            }
        }

        result.push(make_const_variable("\n"));
        return Ok(result);
    }

    fn _parse(&mut self, code: &String) -> Result<Vec<Variable>, CompilerError> {
        let mut result: Vec<Variable> = Vec::new();
        let mut lines: Vec<&str> = code.split('\n').collect();  // [bug here] -> cannot handle multiple
                                                                // line changing

        for (line_id, line) in lines.iter_mut().enumerate() {
            self.location.line_id = line_id;
            match self.parse_line(line) {
                Ok(mut res) => {
                    result.append(&mut res);
                }
                Err(ce) => {
                    return Err(ce);
                }
            };
        }

        return Ok(result);
    }

    pub fn parse(&mut self) -> Result<Vec<Variable>, CompilerError> { self._parse(&self.buffer.clone()) }
}
