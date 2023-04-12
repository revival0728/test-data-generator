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
        let c_rd = | s: &String | -> String { format!("RD {}", s) };
        let c_crd = || -> String { format!("CRD") };
        let c_rdi = | rg: (i64, i64) | -> String { format!("RDI {} {}", rg.0, rg.1) };
        let c_rdf = | rg: (f64, f64), p: f64 | -> String { format!("RDI {} {} {}", rg.0, rg.1, p) };
        let c_rds = | s: &String | -> String { format!("RDS {}", s) };
        let mut add = | s: &String | { res.push_str(s); };
        let newline = "\n".to_string();

        for token in tokens.iter_mut() {
            if token.is_constant() {
                if token.is_tag() {
                    match token.tag() {
                        1 => { add(&c_out(&c_rep(token.quantity()))) }
                        2 => { add(&c_out(&c_crep())) }
                       _ => { return Err(CompilerError::new("Internal CompilerError", &fake_location)); }
                    }
                } else {
                    let s = token.stri();
                    if self.asm_sytanx_table.contains_key(s) {
                        add(&c_out(&s[0..1].to_string()));
                        add(&c_out(&s[1..].to_string()));
                    } else {
                        add(&c_out(&s));
                    }
                }
            } else {
            }
        }

        return Ok(res);
    }
}
