#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Clone)]
pub struct AssemblySytanx {
    pub command: &'static str,
    pub arg_count: i8  // -1 means "until close"
}

pub const ASSEMBLY_SYTNAX: [AssemblySytanx; 11] = [
    AssemblySytanx { command: "REP",  arg_count: 1  },
    AssemblySytanx { command: "CREP", arg_count: 0  },
    AssemblySytanx { command: "RD",   arg_count:-1  },
    AssemblySytanx { command: "QU",   arg_count: 1  },
    AssemblySytanx { command: "EC",   arg_count: 1  },
    AssemblySytanx { command: "CRD",  arg_count: 0  },
    AssemblySytanx { command: "RDI",  arg_count: 2  },
    AssemblySytanx { command: "RDF",  arg_count: 3  },
    AssemblySytanx { command: "RDS",  arg_count:-1  },
    AssemblySytanx { command: "CRDS", arg_count: 0  },
    AssemblySytanx { command: "OUT",  arg_count: 1  },
];

pub fn get_assembly_sytanx_hashmap() -> HashMap<String, AssemblySytanx> {
    let mut res: HashMap<String, AssemblySytanx> = HashMap::new();

    for asyx in ASSEMBLY_SYTNAX.iter() {
        res.insert(asyx.command.to_string(), asyx.clone());
    }

    return res;
}
