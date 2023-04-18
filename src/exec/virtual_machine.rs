// TODO
// 1. process repeat_stack in CREP or other cmd
// 2. cmd RD is not finished

#![allow(dead_code)]

use std::collections::HashMap;

use super::buffer_reader::BufferReader;
use super::error::RuntimeError;
use crate::random::Gener;
use crate::random::GenerResult;
use crate::grammer::td_assembly;
use crate::grammer::td_assembly::AssemblySytanx;

struct VirtualMachine {
    buffer_reader: BufferReader,
    repeat_stack: Vec<(usize, u64)>,  // [(starting command buf_reader index, repeat count)]
    stdout: String,
    asm_table: HashMap<String, AssemblySytanx>
}

impl VirtualMachine {

    pub fn new(file_name: String) -> Result<Self, RuntimeError> {
        let buffer_reader: BufferReader = BufferReader::new(file_name)?;
        let repeat_stack: Vec<(usize, u64)> = Vec::new();
        let stdout = String::new();
        let asm_table = td_assembly::get_assembly_sytanx_hashmap();

        Ok(VirtualMachine { buffer_reader, repeat_stack, stdout, asm_table })
    }

    fn cmd_rep(&mut self) -> Result<(), RuntimeError> {
        let times: u64 = self.buffer_reader.read_arg()?;
        self.repeat_stack.push((self.buffer_reader.get_reader_index() - 1, times));

        let next_cmd = self.buffer_reader.read_cmd()?;
        if !next_cmd.eq("OUT") {
            return Err(RuntimeError::new("all the generated data must be exported to designated file"));
        }

        self.cmd_out()?;
        
        Ok(())
    }

    fn cmd_out(&mut self) -> Result<(), RuntimeError> {
        let next_token = self.buffer_reader.read_token()?;
        
        if self.asm_table.contains_key(next_token) {
            if !next_token.eq("RD") {
                return Err(RuntimeError::new("Internal RuntimeError: In cmd_out()"));
            }

            let data = &self.cmd_rd()?;
            self.stdout.push_str(data);
        } else {
            self.stdout.push_str(&next_token.replace("SPACE", " ").replace("NEWLINE", "\n"));
        }

        Ok(())
    }

    fn cmd_rd(&mut self) -> Result<String, RuntimeError> {
        let mut res = String::new();
        let quantity: u64;
        let end_char: String;
        
        {
            let sub_cmd = self.buffer_reader.read_cmd()?;
            if !sub_cmd.eq("QU") {
                return Err(RuntimeError::new("Internal RuntimeError: didn't find QU cmd after RD cmd"));
            }
            quantity = self.cmd_qu()?;
        }

        {
            let sub_cmd = self.buffer_reader.read_cmd()?;
            if !sub_cmd.eq("EC") {
                return Err(RuntimeError::new("Internal RuntimeError: didn't find EC cmd after RD cmd"));
            }
            end_char = self.cmd_ec()?;
        }

        let mut gener = Gener::new(quantity);
        let mut vi: Vec<(i64, i64)> = Vec::new();
        let mut vf: Vec<(f64, f64)> = Vec::new();
        let mut precision: f64 = 0.0;
        let mut ss: String = String::new();

        loop {
            let sub_cmd = self.buffer_reader.read_cmd()?;

            if sub_cmd.eq("CRD") {
                break;
            }

            match sub_cmd {
                "RDI" => { vi.push(self.cmd_rdi()?) }
                "RDF" => { let r = self.cmd_rdf()?; vf.push(r.0); precision = r.1; }
                "RDS" => { ss.push_str(&self.cmd_rds()?) }
                _ => { return Err(RuntimeError::new(&format!("Internal RuntimeError: unexpected cmd {}", sub_cmd))); }
            }
        }

        match || -> Result<(), ()> {
            gener.gen_i(&vi)?;
            gener.gen_f(&vf, precision)?;
            gener.gen_s(&ss)?;

            Ok(())
        }() {
            Ok(_) => {}
            Err(_) => { return Err(RuntimeError::new("Internal RuntimeError: occured when generating datas")); }
        }

        for i in 0..quantity {

            if i != quantity - 1 {
                res.push_str(&end_char);
            }
        }

        Ok(res)
    }

    fn cmd_rdi(&mut self) -> Result<(i64, i64), RuntimeError> {
        let l: i64 = self.buffer_reader.read_arg()?;
        let r: i64 = self.buffer_reader.read_arg()?;

        Ok((l, r))
    }

    fn cmd_rdf(&mut self) -> Result<((f64, f64), f64), RuntimeError> {
        let l: f64 = self.buffer_reader.read_arg()?;
        let r: f64 = self.buffer_reader.read_arg()?;
        let p: f64 = self.buffer_reader.read_arg()?;

        Ok(((l, r), p))
    }

    fn cmd_rds(&mut self) -> Result<String, RuntimeError> {
        let mut res = String::new();

        loop {
            let token = self.buffer_reader.read_token()?;


            if token.eq("CRDS") {
                break;
            }

            res.push_str(&token.replace("SPACE", " "));
        }

        Ok(res)
    }

    fn cmd_qu(&mut self) -> Result<u64, RuntimeError> {
        let quantity: u64 = self.buffer_reader.read_arg()?;

        Ok(quantity)
    }

    fn cmd_ec(&mut self) -> Result<String, RuntimeError> {
        let end_char = self.buffer_reader.read_token()?.to_string();

        Ok(end_char)
    }

    pub fn main(&mut self) -> Result<(), RuntimeError> {
        let cmd = self.buffer_reader.read_cmd()?;

        if !cmd.eq("REP") {
            return Err(RuntimeError::new("the beginning of the program must be (BEGIN)"));
        }
        
        self.cmd_rep()?;

        Ok(())
    }
}
