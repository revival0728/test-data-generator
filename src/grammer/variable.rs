#![allow(dead_code)]

#[derive(Clone)]
pub enum Attribute {
    IntRange((i64, i64)),
    FloatRange((f64, f64)),
    StrSet(String),
    Tag(String),
    Except(Box<Attribute>),
    Any(()),
}

pub struct Variable {
    int_ranges: Vec<(i64, i64)>,
    float_ranges: Vec<(f64, f64)>,
    str_set: String,
    quantity: u64,
    end_char: String,
    constant: bool,
    const_str: String,
    float_precision: f64  // this is the "step" between generated float number, default value = 0.3f
}

pub fn make_const_variable(const_str: &str) -> Variable {
    let mut res: Variable = Variable::new();
    res.set_to_constant(const_str.to_string());
    return res;
}

impl Variable {

    pub fn new() -> Self {
        Variable {
            int_ranges: Vec::new(),
            float_ranges: Vec::new(),
            str_set: String::from(""),
            quantity: 0,
            end_char: String::from(""),
            constant: true,
            const_str: String::from(""),
            float_precision: 0.001_f64
        }
    }

    pub fn set_quantity(&mut self, quantity: u64) { self.quantity = quantity; }
    pub fn set_end_char(&mut self, end_char: String) { self.end_char = end_char; }
    pub fn set_to_constant(&mut self, const_str: String) { self.constant = true; self.const_str = const_str; }
    pub fn set_to_variable(&mut self) { self.constant = false; }
    pub fn set_float_precision(&mut self, precision: u8) { self.float_precision = 0.1_f64.powi(i32::from(precision)); }

    // return true if successfully jointed new element
    pub fn join_str_set(&mut self, element: char) -> bool {
        let ret: bool = match self.str_set.find(element) {
            Some(_) => { false },
            None => { true }
        };
        if ret {
            self.str_set.push(element);
        }

        return ret;
    }

    // return true if successfully deleted element in str_set
    pub fn del_str_set(&mut self, element: char) -> bool {
        let idx: usize = match self.str_set.find(element) {
            Some(idx) => { idx },
            None => { return false; }
        };
        self.str_set.remove(idx);

        return true;
    }

    pub fn join_int_range(&mut self, range: (i64, i64)) {
        let mut lnr: Vec<(i64, bool)> = Vec::new();

        for i in self.int_ranges.iter() {
            lnr.push((i.0, false));
            lnr.push((i.1, true));
        }
        lnr.push((range.0, false));
        lnr.push((range.1, true));

        lnr.sort_by(|a, b| a.partial_cmp(b).unwrap());

        self.int_ranges.clear();

        let mut last_zero_id: i64 = -1;
        let mut count: i32 = 0;
        for i in lnr.iter() {
            if last_zero_id == -1 { last_zero_id = i.0 }
            if i.1 { count -= 1 }
            else { count += 1 }

            if count == 0 {
                self.int_ranges.push((last_zero_id, i.0));
                last_zero_id = -1;
            }
        }
    }

    pub fn del_int_range(&mut self, range: (i64, i64)) {
        let mut buf: Vec<(i64, i64)> = Vec::new();

        for i in self.int_ranges.iter() {
            if i.1 < range.0 || i.0 > range.1 {
                buf.push(*i);
            } else if range.0 <= i.0 && i.1 <= range.1 {
                continue;
            } else if i.0 < range.0 && range.1 < i.1 {
                buf.push((i.0, range.0 - 1));
                buf.push((range.1 + 1, i.1));
            } else {
                if i.0 < range.0 {
                    buf.push((i.0, range.0 - 1));
                } else if range.1 < i.1 {
                    buf.push((range.1 + 1, i.1));
                } else { assert!(false); }
            }
        }

        self.int_ranges.clear();
        self.int_ranges.append(&mut buf);
    }

    pub fn join_float_range(&mut self, range: (f64, f64)) {
        let mut lnr: Vec<(f64, bool)> = Vec::new();

        for i in self.float_ranges.iter() {
            lnr.push((i.0, false));
            lnr.push((i.1, true));
        }
        lnr.push((range.0, false));
        lnr.push((range.1, true));

        lnr.sort_by(|a, b| a.partial_cmp(b).unwrap());

        self.float_ranges.clear();

        let mut last_zero_id: f64 = -1.0_f64;
        let mut count: i32 = 0;
        for i in lnr.iter() {
            if last_zero_id == -1.0_f64 { last_zero_id = i.0 }
            if i.1 { count -= 1 }
            else { count += 1 }

            if count == 0 {
                self.float_ranges.push((last_zero_id, i.0));
                last_zero_id = -1.0_f64;
            }
        }
    }

    pub fn del_float_range(&mut self, range: (f64, f64)) {
        let mut buf: Vec<(f64, f64)> = Vec::new();

        for i in self.float_ranges.iter() {
            if i.1 < range.0 || i.0 > range.1 {
                buf.push(*i);
            } else if range.0 <= i.0 && i.1 <= range.1 {
                continue;
            } else if i.0 < range.0 && range.1 < i.1 {
                buf.push((i.0, range.0 - self.float_precision));
                buf.push((range.1 + self.float_precision, i.1));
            } else {
                if i.0 < range.0 {
                    buf.push((i.0, range.0 - self.float_precision));
                } else if range.1 < i.1 {
                    buf.push((range.1 + self.float_precision, i.1));
                } else { assert!(false); }
            }
        }

        self.float_ranges.clear();
        self.float_ranges.append(&mut buf);
    }

    #[cfg(test)]
    pub fn get_int_ranges(&self) -> &Vec<(i64, i64)> { &self.int_ranges }

    #[cfg(test)]
    pub fn get_float_ranges(&self) -> &Vec<(f64, f64)> { &self.float_ranges }

    #[cfg(test)]
    pub fn get_str_set(&self) -> &String { &self.str_set }

    #[cfg(test)]
    pub fn get_quanity(&self) -> u64 { self.quantity }

    #[cfg(test)]
    pub fn get_end_char(&self) -> &String { &self.end_char }

    #[cfg(test)]
    pub fn get_constant(&self) -> bool { self.constant }

    #[cfg(test)]
    pub fn get_const_str(&self) -> &String { &self.const_str }

    #[cfg(test)]
    pub fn get_flaot_precision(&self) -> f64 { self.float_precision }
}
