mod exec;
mod grammer;
mod random;
mod compiler;

use std::env;
use std::process;
use subprocess;

use crate::compiler::Compiler;
use crate::exec::virtual_machine::VirtualMachine;

mod helper_func {
    use std::{ path::Path, fs::File, io::Write, fs::OpenOptions };

    pub fn print_msg(msg: &str) { println!("tdg: {}", msg) }

    // return true if success else return false
    pub fn write_file(file_name: &str, text: &str) -> bool {
        let path = Path::new(file_name);
        let path_display = path.display();
        let mut file = if path.exists() { 
            match OpenOptions::new().truncate(true).write(true).open(&path) {
                Ok(r) => { r }
                Err(why) => { 
                    print_msg(&format!("cannot open or create file {} due to {}", path_display, why)); 
                    return false;
                }
            }
        } else {
            match File::create(&path) {
                Ok(r) => { r }
                Err(why) => { 
                    print_msg(&format!("cannot open or create file {} due to {}", path_display, why)); 
                    return false;
                }
            }
        };

        match file.write_all(text.as_bytes()) {
            Ok(_) => {}
            Err(why) => {
                print_msg(&format!("cannot write file {} due to {}", path_display, why)); 
                return false;
            }
        };

        return true;
    }
}

const CONFIG_ARGS_COUNT: usize = 5;

// [(config name, argument count)]
const CONFIG_ARGS: [(&'static str, u8); CONFIG_ARGS_COUNT] = [
    ("-c", 1),  // compile
    ("-e", 1),  // execute
    ("-n", 1),  // the number of generated file
    ("--filename-format", 1),  // file name format e.g. FN_** 
                               // (** means the id of the test data, 0 base, N starts means N decimals)
    ("--create-answer", 1),  // run a program with all the generated datas and output result as .out file
];

fn proc_args(args: &Vec<String>) -> [Vec<&str>; CONFIG_ARGS_COUNT] {
    const INIT_VEC: Vec<&str> = Vec::new();
    let mut config: [Vec<&str>; CONFIG_ARGS_COUNT] = [INIT_VEC; CONFIG_ARGS_COUNT];

    for i in 0..CONFIG_ARGS_COUNT {
        let mut to_push = false;
        for j in args.iter() {
            if to_push {
                config[i].push(&j);
            }
            to_push = config[i].len() == 0 && j.eq(CONFIG_ARGS[i].0);
        }
    }

    for i in 0..CONFIG_ARGS_COUNT {
        if config[i].len() == 0 {
            config[i].push("");
        }
    }

    config
}


fn compile(file_name: &str) -> i32 {
    let mut compiler = Compiler::new();

    let res = match compiler.compile(file_name.to_string()) {
        Ok(r) => { r }
        Err(e) => {
            helper_func::print_msg(&e.get_msg());
            return 1;
        }
    };

    helper_func::write_file(&file_name.replace(".tds", ".tdc"), &res);

    return 0;
}

fn execute(file_name: &str, gen_file_count: &str, output_format: &str, ans_exec_cmd: &str) -> i32 {
    let gen_file_count: u8 = if gen_file_count.len() != 0 { match gen_file_count.parse() {
        Ok(r) => { r }
        Err(_) => {
            helper_func::print_msg("the arguemnt of -n must be an positive integer");
            return 1;
        }
    }} else { 1 };
    let star_count = | s: &str | -> (u8, String) {
        let mut res: (u8, String) = (0, "".to_string());
        for c in s.chars() {
            if c == '*' {
                res.0 += 1;
                res.1.push('*');
            }
        }

        res
    };
    let make_id = | i: u8, l: u8 | -> Result<String, i32> {
        let i = i.to_string();
        let mut res = String::new();

        if l < i.len() as u8 { return Err(1) }

        for _ in 0..(l-i.len() as u8) { res.push('0'); }
        res.push_str(&i);

        Ok(res)
    };

    for count in 0..gen_file_count {
        let mut vm = match VirtualMachine::new(file_name.to_string()) {
            Ok(r) => { r }
            Err(e) => { 
                helper_func::print_msg(&e.get_msg());
                return 1;
            }
        };

        match vm.exec() {
            Ok(_) => {}
            Err(e) => {
                helper_func::print_msg(&e.get_msg());
                return 1;
            }
        }

        let id_info = star_count(output_format);
        let fn_template = if output_format.len() != 0 {
                output_format.replace(
                    &id_info.1, 
                    &match make_id(count, id_info.0) {
                        Ok(r) => { r }
                        Err(c) => {
                            helper_func::print_msg("stars in --filename-format is less than the length of the file ID");
                            return c;
                        }
                    }
                )
            } else {
                file_name.replace(".tdc", "")
            };

        // generate datas
        {
            let output_fn = format!("{}.in", fn_template);
            helper_func::write_file(&output_fn, vm.stdout());
        }

        if ans_exec_cmd.len() != 0 {
            let ans_text = match subprocess::Exec::cmd(ans_exec_cmd)
            .stdin(vm.stdout().as_str())
            .stdout(subprocess::Redirection::Pipe)
            .capture() {
                Ok(r) => { r }
                Err(_) => {
                    helper_func::print_msg("cannot catch answer command standard output");
                    return 1;
                }
            }.stdout_str();

            let output_fn = format!("{}.out", fn_template);

            helper_func::write_file(&output_fn, &ans_text);
        }
    }

    return 0;
}

fn cli() -> i32 {
    let args: Vec<String> = env::args().collect();
    let config: [Vec<&str>; CONFIG_ARGS_COUNT] = proc_args(&args);

    // cannot execute and compile at the same time
    if config[0][0].len() != 0 && config[1][0].len() != 0 {
        helper_func::print_msg("cannot execute and compile at the same time");
        return 1;
    }

    if config[0][0].len() == 0 && config[1][0].len() == 0 {
        helper_func::print_msg("need to assign a task to execute");
        return 1;
    }

    return if config[0][0].len() != 0 { compile(config[0][0]) } else { execute(config[1][0], config[2][0], config[3][0], config[4][0]) }
}

fn main() {
    let exit_code = cli();

    process::exit(exit_code);
}
