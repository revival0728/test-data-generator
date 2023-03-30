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
        let materials: Vec<(&str, Attribute)> = vec![
            
            ("BEGIN", Attribute::Tag("BEGIN".to_string())),
            ("END", Attribute::Tag("END".to_string())),
            ("@IntRange", Attribute::IntRange((0, 0))),
            ("@FloatRange", Attribute::FloatRange((0_f64, 0_f64))),
            ("@StrSet", Attribute::StrSet("".to_string())),
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

        let types: Vec<(&str, Attribute)> = vec![

            ("int", Attribute::Type("int".to_string())),
            ("float", Attribute::Type("float".to_string())),
            ("string", Attribute::Type("string".to_string())),
            ("=", Attribute::Type("ANY".to_string())),

        ];

        // ( attribute, need_exception )
        let mut insert_attrubtes = | a: &Vec<(&str, Attribute)>, ne: bool | {
            for i in a.iter() {
                attributes.insert(i.0.to_string(), i.1.clone());
                if ne {
                    let mut j: String = "\\".to_string();
                    j.push_str(i.0);
                    attributes.insert(j, Attribute::Except(Box::from(i.1.clone())));
                }
            }
        };

        insert_attrubtes(&materials, true);
        insert_attrubtes(&types, false);

        return Ok(Parser { location, attributes, buffer });
    }

    // code = "( ... ; ... ; ... )"
    fn parse_variable(&mut self, code: &str) -> Result<Variable, CompilerError> {
        // use += to operate location.word_id

        let mut result: Variable = Variable::new();
        let mut attr_ranges: Vec<(LocType, LocType)> = Vec::new();  // [)

        {
            let mut semi_pos: Vec<LocType> = Vec::new(); // semicolon position -> [0, ..., len())

            semi_pos.push(0);
            for (id, iter) in code.chars().enumerate() {
                if iter == ';' {
                    semi_pos.push(id);
                }
            }
            semi_pos.push(code.len());

            if semi_pos.len() > 5 {
                self.location.word_id += semi_pos[4];
                return Err(CompilerError::new("too much attributes in one variable", &self.location));
            }

            for (id, iter) in semi_pos.iter().enumerate() {
                if id + 1 < semi_pos.len() {
                    if *iter + 1 <= semi_pos[id + 1] {
                        attr_ranges.push((*iter + 1, semi_pos[id + 1]));
                    }
                }
            }
        }

        let mut materials: Vec<Attribute> = Vec::new();
        let mut types: Vec<Attribute> = Vec::new();
        let mut quantity: u64 = 1;
        let mut end_char: String = "".to_string();

        'process_materials: {
            let m_range: (LocType, LocType) = attr_ranges[0];

            if m_range.0 == m_range.1 {
                self.location.word_id += m_range.0;
                return Err(CompilerError::new("no materials in this variable", &self.location));
            }

            let unproc_materials: Vec<&str> = code[m_range.0 .. m_range.1].split(' ').collect();

            for upm in unproc_materials.iter() {
                let upm = String::from(*upm);

                if self.attributes.contains_key(&upm) {
                    materials.push(self.attributes.get(&upm).unwrap().clone());
                } else {
                    materials.push(Attribute::Uncertain(upm.clone()));
                    // let dots_pos = match upm.find("..") {
                    //     Some(p) => { p },
                    //     None => { 
                    //         self.location.word_id += code.find(&upm).unwrap();
                    //         return Err(CompilerError::new("unknown material", &self.location)); 
                    //     }
                    // };
                }
            }

            break 'process_materials;
        }

        'process_types: {
            let m_range: (LocType, LocType) = attr_ranges[1];

            if m_range.0 == m_range.1 {
                break 'process_types;
            }

            let unproc_types: Vec<&str> = code[m_range.0 .. m_range.1].split(' ').collect();

            for upt in unproc_types.iter() {
                let upt = String::from(*upt);

                if self.attributes.contains_key(&upt) {
                    types.push(self.attributes.get(&upt).unwrap().clone());
                } else {
                    self.location.word_id += m_range.0;
                    return Err(CompilerError::new("has unkown type in this variable", &self.location));
                }
            }

            break 'process_types;
        }

        'process_quantity: {
            let m_range: (LocType, LocType) = attr_ranges[1];

            if m_range.0 == m_range.1 {
                break 'process_quantity;
            }

            quantity = match code[m_range.0 .. m_range.1].trim().parse() {
                Ok(v) => { v },
                Err(_) => { 
                    self.location.word_id += m_range.0;
                    return Err(CompilerError::new("the argument quantity must be positive integer", &self.location)); 
                }
            };

            break 'process_quantity;
        }

        'process_end_char: {
            let m_range: (LocType, LocType) = attr_ranges[1];

            if m_range.0 == m_range.1 {
                break 'process_end_char;
            }

            let uncheck = code[m_range.0 .. m_range.1].trim();

            if self.attributes.contains_key(uncheck) {
                end_char = match self.attributes.get(uncheck).unwrap() {
                    Attribute::StrSet(v) => { v.clone() },
                    _ => { 
                        self.location.word_id += m_range.0;
                        return Err(CompilerError::new("end_char cannot be a sytanx character", &self.location)) 
                    },
                }
            }

            break 'process_end_char;
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
