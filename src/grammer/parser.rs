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

pub struct Parser {
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
            ("DOT", Attribute::StrSet(".".to_string())),
            ("COL", Attribute::StrSet(":".to_string())),

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

        #[cfg(test)]
        {
            println!("In parse_variable(): code = [{}]", code);
        }

        let mut result: Variable = Variable::new();
        let mut attr_ranges: Vec<(LocType, LocType)> = Vec::new();  // [)

        'divide_attributes: {
            let mut semi_pos: Vec<LocType> = Vec::new(); // semicolon position -> [0, ..., len())

            semi_pos.push(0);
            for (id, iter) in code.chars().enumerate() {
                if iter == ';' {
                    semi_pos.push(id);
                }
            }
            semi_pos.push(code.len() - 1);

            if semi_pos.len() > 5 {
                self.location.word_id += semi_pos[4];
                return Err(CompilerError::new("too much attributes in one variable", &self.location));
            }
            
            if semi_pos.len() == 2 {
                attr_ranges.push((semi_pos[0] + 1, semi_pos[1]));
                break 'divide_attributes;
            }

            for (id, iter) in semi_pos.iter().enumerate() {
                if id + 1 < semi_pos.len() {
                    if *iter + 1 <= semi_pos[id + 1] {
                        attr_ranges.push((*iter + 1, semi_pos[id + 1]));
                    }
                }
            }
        }

        #[cfg(test)]
        {
            println!("In parse_variable(): attr_ranges.len() = [{}]", attr_ranges.len());
            println!("In parse_variable(): code.len() = [{}]", code.len());
            
            for (id, i) in attr_ranges.iter().enumerate() {
                println!("In parse_variable(): attr_ranges[{}] = [({}, {})]", id, i.0, i.1);
            }
        }

        let mut materials: Vec<Attribute> = Vec::new();
        let mut types: Vec<Attribute> = Vec::new();
        let mut quantity: u64 = 1;
        let mut end_char: String = "".to_string();
        let mut float_precision: i128 = -1;

        'process_materials: {
            let m_range: (LocType, LocType) = attr_ranges[0];

            if m_range.0 == m_range.1 {
                self.location.word_id += m_range.0;
                return Err(CompilerError::new("no materials in this variable", &self.location));
            }

            let unproc_materials: Vec<&str> = code[m_range.0 .. m_range.1].split(' ').collect();

            for upm in unproc_materials.iter() {
                let upm = String::from(*upm);

                if upm.len() == 0 {
                    continue;
                }
                if self.attributes.contains_key(&upm) {
                    materials.push(self.attributes.get(&upm).unwrap().clone());
                } else {
                    materials.push(Attribute::Uncertain(upm.clone()));
                }
            }

            break 'process_materials;
        }

        'process_types: {
            if attr_ranges.len() <= 1 {
                break 'process_types;
            }
            let m_range: (LocType, LocType) = attr_ranges[1];

            if m_range.0 == m_range.1 {
                break 'process_types;
            }

            let unproc_types: Vec<&str> = code[m_range.0 .. m_range.1].trim().split(' ').collect();

            #[cfg(test)]
            {
                for (id, i) in unproc_types.iter().enumerate() {
                    println!("In parse_variable(): unproc_types[{}] = [{}]", id, i)
                }
            }

            for upt in unproc_types.iter() {
                let upt = String::from(*upt);

                #[cfg(test)]
                {
                    println!("In parse_variable(): upt = [{}]", upt)
                }

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
            if attr_ranges.len() <= 2 {
                break 'process_quantity;
            }
            let m_range: (LocType, LocType) = attr_ranges[2];

            if m_range.0 == m_range.1 {
                break 'process_quantity;
            }

            #[cfg(test)]
            {
                println!("In parse_variable(): code = [{}]", code);
                println!("In parse_variable(): m_range = ({}, {})", m_range.0, m_range.1);
                println!("In parse_variable(): code[m_range] = [{}]", &code[m_range.0 .. m_range.1]);
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
            if attr_ranges.len() <= 3 {
                break 'process_end_char;
            }
            let m_range: (LocType, LocType) = attr_ranges[3];

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

        result.set_quantity(quantity);
        result.set_end_char(end_char);
        result.set_to_variable();

        if materials.len() != types.len() {
            #[cfg(test)]
            {
                println!("In parse_variable(): materials.len() = [{}]", materials.len());
                println!("In parse_variable(): types.len() = [{}]", types.len());
                if materials.len() >= 2 { println!("In parse_variable(): materials[1] = [{}]", match &materials[1] {
                    Attribute::Tag(t) => { t.clone() },
                    Attribute::Uncertain(v) => { v.clone() },
                    _ => { "X".to_string() }
                })}
                println!("In parse_variable(): materials[0] = {}", match &materials[0] {
                    Attribute::Tag(t) => { t.clone() },
                    Attribute::Uncertain(v) => { v.clone() },
                    _ => { "X".to_string() }
                });
                println!("In parse_variable(): materials[0].type = [{}]", materials[0].get_type());
            }
            if materials.len() == 1 && (materials[0] == Attribute::Tag("BEGIN".to_string()) || materials[0] == Attribute::Tag("END".to_string()))  {
                result.set_to_tag(if materials[0] == Attribute::Tag("BEGIN".to_string()) { 1 } else { 2 });
                return Ok(result);
            } else {
                return Err(CompilerError::new("number of material and type must be the same", &self.location)) 
            }
        }

        let parse_int = | s: &str | -> Result<(i64, i64), CompilerError> {
            let dot_pos = match s.find("..") {
                Some(p) => { p },
                None => { return Err(CompilerError::new("Internal CompilerError", &self.location)); }
            };
            let mut res: (i64, i64) = (0, 0);

            res.0 = match s[0 .. dot_pos].parse() {
                Ok(r) => { r },
                Err(_) => { return Err(CompilerError::new("the border of NumberRange must be a number", &self.location)); }
            };
            res.1 = match s[dot_pos+2 .. ].parse() {
                Ok(r) => { r },
                Err(_) => { return Err(CompilerError::new("the border of NumberRange must be a number", &self.location)); }
            };

            return Ok(res);
        };

        let parse_float = | s: &str | -> Result<((f64, f64), i128), CompilerError> {
            let dot_pos = match s.find("..") {
                Some(p) => { p },
                None => { return Err(CompilerError::new("Internal CompilerError", &self.location)); }
            };
            let col_pos = match s.find(":.") {
                Some(p) => { p },
                None => { 0 }
            };
            if s.chars().nth(col_pos).unwrap() != ':' { return Err(CompilerError::new("Invalid Sytanx", &self.location)); }
            let mut res: ((f64, f64), i128) = ((0_f64, 0_f64), 0);

            res.0.0 = match s[0 .. dot_pos].parse() {
                Ok(r) => { r },
                Err(_) => { return Err(CompilerError::new("the border of NumberRange must be a number", &self.location)); }
            };
            res.0.1 = match s[dot_pos+2 .. ].parse() {
                Ok(r) => { r },
                Err(_) => { return Err(CompilerError::new("the border of NumberRange must be a number", &self.location)); }
            };
            res.1 = match s[col_pos+2 .. s.len()-1].parse() {
                Ok(r) => { r },
                Err(_) => { return Err(CompilerError::new("the precision of NumberRange must be positive integer", &self.location)); }
            };

            return Ok(res);
        };

        for i in 0..materials.len() {
            let s = match &materials[i] {
                Attribute::Uncertain(v) => { v.trim() },
                Attribute::StrSet(v) => { v.trim() },
                _ => { return Err(CompilerError::new("Internal CompilerError", &self.location)); },
            };
            let dot_pos: i128 = match s.find("..") {
                Some(p) => { p.try_into().unwrap() },
                None => { -1 }
            };
            let col_pos: i128 = match s.find(":.") {
                Some(p) => { p.try_into().unwrap() },
                None => { -1 }
            };
            if types[i] == Attribute::Type("=".to_string()) {
                if dot_pos != -1 && col_pos != -1 {
                    let float_range = match parse_float(s) {
                        Ok(r) => { r },
                        Err(e) => { return Err(e); }
                    };
                    float_precision = if float_precision == -1 { float_range.1 } 
                    else { if float_range.1 == float_precision { float_precision } 
                    else { return Err(CompilerError::new("float precision in one variable must be the same", &self.location)); } };
                    result.join_float_range(float_range.0);
                } else if dot_pos != -1 {
                    result.join_int_range(match parse_int(s) {
                        Ok(r) => { r },
                        Err(e) => { return Err(e); }
                    });
                }
            } else if types[i] == Attribute::Type("int".to_string()) {
                if dot_pos == -1 { return Err(CompilerError::new("IntRange syntax mismatched (Tips: [L..R])", &self.location)); }
                result.join_int_range(match parse_int(s) {
                    Ok(r) => { r },
                    Err(e) => { return Err(e); }
                });
            } else if types[i] == Attribute::Type("float".to_string()) {
                if dot_pos == -1 || col_pos == -1 { return Err(CompilerError::new("FloatRange syntax mismatched (Tips: [L..R]:.[N]f)", &self.location)); }
                let float_range = match parse_float(s) {
                    Ok(r) => { r },
                    Err(e) => { return Err(e); }
                };
                float_precision = if float_precision == -1 { float_range.1 } 
                else { if float_range.1 == float_precision { float_precision } 
                else { return Err(CompilerError::new("float precision in one variable must be the same", &self.location)); } };
                result.join_float_range(float_range.0);
            } else if types[i] == Attribute::Type("string".to_string()) {
                let sv: Vec<char> = s.chars().collect();
                for c in sv.iter() { result.join_str_set(*c); }
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

        #[cfg(test)]
        {
            println!("In parse_line(): code = [{}]", code);
            for (id, i) in var_bracket_pairs.iter().enumerate() {
                println!("In parse_line(): var_bracket_pairs[{}] = ({}, {})", id, i.0, i.1);
            }
        }
        for (id, iter) in var_bracket_pairs.iter().enumerate() {
            if id == 0 && 0 < iter.0 {
                result.push(make_const_variable(&code[0..iter.0]));
            } else if id == var_bracket_pairs.len() - 1 && iter.1 < code.len()-1 {
                result.push(make_const_variable(&code[iter.1..code.len()-1].trim()))
            } else {
                if id + 1 < var_bracket_pairs.len() {
                    if iter.1 < var_bracket_pairs[id + 1].0 {
                        result.push(make_const_variable(
                            &code[iter.1..var_bracket_pairs[id + 1].0],
                        ));
                    }
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
