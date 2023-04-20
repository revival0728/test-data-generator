pub mod virtual_machine;
pub mod error;
mod buffer_reader;

#[cfg(test)]
mod test_buffer_reader {

    use super::buffer_reader::BufferReader;

    #[test]
    fn read_all() {
        let mut bfr = match BufferReader::new("./test_file/test3.tdc".to_string()) {
            Ok(v) => { v }
            Err(_) => { panic!("BufferReader::new()") }
        };

        let res = match bfr.read_to_end() {
            Ok(v) => { v }
            Err(_) => { panic!("BufferReader::read_to_end()") }
        };
        let mut ans: Vec<String> = Vec::new();
        let mut _ans: Vec<&str> = vec!["REP", "1", "OUT", "RD", "QU", "8", "EC", "*", "RDS", "ABCDEFGHIJKLMNOPQRSTUVWXYZhijklmnopqrstuvwxyz~`!@#$%^&*()_-+=|[]{}:;'\"/?.>,<", "CRDS", "CRD", "OUT", "NEWLINE", "OUT", "RD", "QU", "11", "EC", "SPACE", "RDI", "1", "24", "RDI", "76", "100", "CRD", "OUT", "NEWLINE", "OUT", "RD", "QU", "15", "EC", "SPACE", "RDF", "1", "24.999", "0.00010000000000000005", "RDF", "75.001", "100", "0.00010000000000000005", "CRD", "CREP", "OUT", "NEWLINE"];

        ans.extend(_ans.iter().map(| a | -> String { a.to_string() }));

        assert_eq!(res, ans);
    }
}

#[cfg(test)]
mod test_virtual_machine {

    use super::virtual_machine::VirtualMachine;

    // #[test]
    // fn run_1() {
    //     let mut vm = match VirtualMachine::new("./test_file/test1.tdc".to_string()) {
    //         Ok(v) => { v }
    //         Err(_) => { panic!("VirtualMachine::new()") }
    //     };
    //
    //     match vm.exec() {
    //         Ok(_) => {}
    //         Err(_) => { panic!("VirtualMachine::exec()") }
    //     };
    //
    //     println!("vm.stdout() = [{}]", vm.stdout());
    // }
    //
    // #[test]
    // fn run_2() {
    //     let mut vm = match VirtualMachine::new("./test_file/test2.tdc".to_string()) {
    //         Ok(v) => { v }
    //         Err(_) => { panic!("VirtualMachine::new()") }
    //     };
    //
    //     match vm.exec() {
    //         Ok(_) => {}
    //         Err(_) => { panic!("VirtualMachine::exec()") }
    //     };
    //
    //     println!("vm.stdout() = [{}]", vm.stdout());
    // }

    #[test]
    fn run_3() {
        let mut vm = match VirtualMachine::new("./test_file/test3.tdc".to_string()) {
            Ok(v) => { v }
            Err(_) => { panic!("VirtualMachine::new()") }
        };

        match vm.exec() {
            Ok(_) => {}
            Err(_) => { panic!("VirtualMachine::exec()") }
        };

        println!("vm.stdout() = [{}]", vm.stdout());
    }
}
