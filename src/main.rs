mod exec;
mod grammer;
mod random;
mod compiler;

//TODO finished CLI feature and finished the project

use std::env;
use subprocess;

use crate::compiler::Compiler;
use crate::exec::virtual_machine::VirtualMachine;

// [(config name, argument count)]
const CONFIG_ARGS: [(&'static str, u8); 5] = [
    ("-c", 0),  // compile
    ("-r", 0),  // run
    ("-n", 1),  // the number of generated file
    ("--filename-format", 1),  // file name format e.g. FN_** 
                               // (** means the id of the test data, 0 base, N starts means N decimals)
    ("--create-answer", 1),  // run a program with all the generated datas and output result as .out file
];

fn main() {
    let args: Vec<String> = env::args().collect();
    :q
    :qa
}
