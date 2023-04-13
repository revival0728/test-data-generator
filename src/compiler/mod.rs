#![allow(dead_code)]

use crate::grammer::parser::Parser;
use crate::grammer::error;
use crate::grammer::error::CompilerError;
use crate::grammer::td_assembly;
use std::collections::HashMap;

struct Compiler {
    asm_sytanx_table: HashMap<String, td_assembly::AssemblySytanx>
}

impl Compiler {
    
    pub fn new() -> Self {
        let asm_sytanx_table = td_assembly::get_assembly_sytanx_hashmap();

        Compiler { asm_sytanx_table }
    }

    pub fn compile(&mut self, file_path: String) -> Result<String, CompilerError> {
        let mut parser = Parser::new(file_path)?;
        let mut res = String::new();
        let mut tokens = parser.parse()?;
        let fake_location = error::Location::new("PLEASE REPORT".to_string(), 0, 0);

        //  fn c_##() means "create ##"
        let c_out = | s: &String | -> String { format!("OUT {}", s) };
        let c_rep = | q: u64 | -> String { format!("REP {}", q) };
        let c_crep = || -> String { format!("CREP") };
        let c_rd = | qu: &String, s: &String | -> String { format!("RD {} {} CRD", qu, s) };
        let c_qu = | q: u64 | -> String { format!("QU {}", q) };
        let c_rdi = | rg: (i64, i64) | -> String { format!("RDI {} {}", rg.0, rg.1) };
        let c_rdf = | rg: (f64, f64), p: f64 | -> String { format!("RDF {} {} {}", rg.0, rg.1, p) };
        let c_rds = | s: &String | -> String { format!("RDS {} CRDS", s) };
        let mut add = | s: &String | { res.push_str(s); res.push('\n'); };

        for token in tokens.iter_mut() {
            if token.is_constant() {
                if token.is_tag() {
                    match token.tag() {
                        1 => { add(&c_rep(token.quantity())) }
                        2 => { add(&c_crep()) }
                       _ => { return Err(CompilerError::new("Internal CompilerError", &fake_location)); }
                    }
                } else {
                    let mut s = token.const_str().clone();
                    s = s.replace(" ", " SPACE ")
                    .replace("\n", " NEWLINE ")
                    .trim()
                    .replace("  ", " ");
                    if self.asm_sytanx_table.contains_key(&s) {
                        add(&c_out(&s[0..1].to_string()));
                        add(&c_out(&s[1..].to_string()));
                    } else {
                        add(&c_out(&s));
                    }
                }
            } else {
                let mut int_out = String::new();
                let mut float_out = String::new();
                let mut str_out = String::new();

                {
                    let int_range = token.int();

                    if !int_range.is_empty() { 
                        for i in int_range.iter() {
                            int_out.push_str(&c_rdi(*i));
                            int_out.push(' ');
                        }
                        int_out = int_out.trim().to_string();
                    }
                }

                {
                    let (float_range, precision) = token.float();

                    if !float_range.is_empty() {
                        for i in float_range.iter() {
                            float_out.push_str(&c_rdf(*i, precision));
                            float_out.push(' ');
                        }
                        float_out = float_out.trim().to_string();
                    }
                }
                
                {
                    let mut str_set = token.stri().clone();
                    str_set = str_set.replace(" ", " SPACE ").trim().to_string();

                    if !str_set.is_empty() {
                        str_out.push_str(&c_rds(&str_set));
                    }
                }

                let mut out = String::new();

                out.push_str(&int_out);
                out.push(' ');
                out.push_str(&float_out);
                out = out.trim().to_string();
                out.push(' ');
                out.push_str(&str_out);
                out = out.trim().to_string();

                add(&c_out(&c_rd(&c_qu(token.quantity()), &out)));
            }
        }
        res.pop();

        return Ok(res);
    }
}

#[cfg(test)]
mod test_compiler {
    use super::Compiler;

    #[test]
    fn compile_1() {
        let mut compiler = Compiler::new();

        match compiler.compile("./test_file/test1.tds".to_string()) {
            Ok(r) => { println!("{}", r); }
            Err(e) => { println!("{}", e.get_msg()); assert!(false); return; },
        }
    }

    #[test]
    fn compile_2() {
        let mut compiler = Compiler::new();

        match compiler.compile("./test_file/test2.tds".to_string()) {
            Ok(r) => { println!("{}", r); }
            Err(e) => { println!("{}", e.get_msg()); assert!(false); return; },
        }
    }

    #[test]
    fn compile_3() {
        let mut compiler = Compiler::new();

        match compiler.compile("./test_file/test3.tds".to_string()) {
            Ok(r) => { println!("{}", r); }
            Err(e) => { println!("{}", e.get_msg()); assert!(false); return; },
        }
    }
}
