pub mod parser;
pub mod variable;
pub mod error;
pub mod td_assembly;

#[cfg(test)]
mod test_parser {

    use crate::grammer::parser::Parser;

    #[test]
    fn parse_1() {
        let mut p = match Parser::new("./test_file/test1.tds".to_string()) {
            Ok(r) => { r },
            Err(e) => { println!("{}", e.get_msg()); assert!(false); return; },
        };
        match p.parse() {
            Ok(r) => { for i in r.iter() { println!("{}", i); } },
            Err(e) => { println!("{}", e.get_msg()); assert!(false); return; },
        };
    }

    #[test]
    fn parse_2() {
        let mut p = match Parser::new("./test_file/test2.tds".to_string()) {
            Ok(r) => { r },
            Err(e) => { println!("{}", e.get_msg()); assert!(false); return; },
        };
        match p.parse() {
            Ok(r) => { for i in r.iter() { println!("{}", i); } },
            Err(e) => { println!("{}", e.get_msg()); assert!(false); return; },
        };
    }

    #[test]
    fn parse_3() {
        let mut p = match Parser::new("./test_file/test3.tds".to_string()) {
            Ok(r) => { r },
            Err(e) => { println!("{}", e.get_msg()); assert!(false); return; },
        };
        match p.parse() {
            Ok(r) => { for i in r.iter() { println!("{}", i); } },
            Err(e) => { println!("{}", e.get_msg()); assert!(false); return; },
        };
    }
}

#[cfg(test)]
mod test_variable_attribute {
    
    use crate::grammer::variable::Attribute;

    #[test]
    fn value_equal() {
        let a = Attribute::Any(());
        let b = Attribute::StrSet("abc".to_string());
        let c = Attribute::StrSet("cdef".to_string());

        assert!(a != b);
        assert!(b != c);
    }

    #[test]
    fn type_equal() {
        let mut a = Attribute::Any(());
        let mut b = Attribute::StrSet("abc".to_string());
        let mut c = Attribute::StrSet("cdef".to_string());
        let mut d = Attribute::Except(Box::from(Attribute::Except(Box::from(Attribute::Any(())))));
        let mut e = Attribute::Except(Box::from(Attribute::Except(Box::from(Attribute::Any(())))));
        let mut f = Attribute::Except(Box::from(Box::from(Attribute::Any(()))));
        let mut g = Attribute::IntRange((0, 1));
        let mut h = Attribute::FloatRange((0.0, 1.1));
        let mut i = Attribute::Tag("BEGIN".to_string());
        let mut j = Attribute::Type("int".to_string());
        let mut k = Attribute::Uncertain("1..10".to_string());

        assert_ne!(a.get_type(), b.get_type());
        assert_eq!(b.get_type(), c.get_type());
        assert_eq!(d.get_type(), e.get_type());
        assert_ne!(e.get_type(), f.get_type());
        assert_ne!(f.get_type(), a.get_type());

        assert_eq!(a.get_type(), "Any");
        assert_eq!(b.get_type(), "StrSet");
        assert_eq!(d.get_type(), "Except[Except[Any]]");
        assert_eq!(g.get_type(), "IntRange");
        assert_eq!(h.get_type(), "FloatRange");
        assert_eq!(i.get_type(), "Tag");
        assert_eq!(j.get_type(), "Type");
        assert_eq!(k.get_type(), "Uncertain");
    }
}

#[cfg(test)]
mod test_variable {

    use crate::grammer::variable::Variable;

    #[test]
    fn join_int_range() {
        let mut a = Variable::new();
        let ans: &Vec<(i64, i64)> = &vec![(-2, 11), (15, 30), (100, 100)];
        let data: &Vec<(i64, i64)>;

        a.join_int_range((0, 10));
        a.join_int_range((-1, 11));
        a.join_int_range((-2, 5));
        a.join_int_range((15, 20));
        a.join_int_range((20, 30));
        a.join_int_range((100, 100));

        data = a.get_int_ranges();

        assert_eq!(data, ans);
    }

    #[test]
    fn del_int_range() {
        let mut a = Variable::new();
        let ans: &Vec<(i64, i64)> = &vec![(21, 24), (27, 30)];
        let data: &Vec<(i64, i64)>;

        a.join_int_range((0, 10));
        a.join_int_range((-1, 11));
        a.join_int_range((-2, 5));
        a.join_int_range((15, 20));
        a.join_int_range((20, 30));
        a.join_int_range((100, 100));
        // a = [(-2, 11), (15, 30)]
        
        a.del_int_range((-3, 12));
        a.del_int_range((15, 15));
        a.del_int_range((100, 100));
        a.del_int_range((5, 20));
        a.del_int_range((25, 26));

        data = a.get_int_ranges();

        assert_eq!(data, ans);
    }

    fn approx_eq(a: f64, b: f64) -> bool { (a - b).abs() < 1e-10 }
    fn range_approx_eq(a: (f64, f64), b: (f64, f64)) -> bool { approx_eq(a.0, b.0) && approx_eq(a.1, b.1) }

    #[test]
    fn join_float_range() {
        let mut a = Variable::new();
        let ans: &Vec<(f64, f64)> = &vec![(-0.5_f64, 10.5_f64), (11.123_f64, 20_f64)];
        let data: &Vec<(f64, f64)>;

        a.join_float_range((1_f64, 10.5_f64));
        a.join_float_range((-1_f64, 0_f64));
        a.join_float_range((-0.5_f64, 5.34_f64));
        a.join_float_range((11.123_f64, 20_f64));

        data = a.get_float_ranges();

        assert_eq!(ans.len(), data.len());
        for i in 0..ans.len() { assert!( range_approx_eq(ans[i], data[i]) ) }
    }

    #[test]
    fn del_float_range() {
        let mut a = Variable::new();
        let ans: &Vec<(f64, f64)> = &vec![(15.001_f64, 17.549_f64), (18.011_f64, 19.499_f64)];
        let data: &Vec<(f64, f64)>;

        a.join_float_range((1_f64, 10.5_f64));
        a.join_float_range((-1_f64, 0_f64));
        a.join_float_range((-0.5_f64, 5.34_f64));
        a.join_float_range((11.123_f64, 20_f64));
        // a = [(-1_f64, 10.5_f64), (11.123_f64, 20_f64)]
        
        a.del_float_range((-2_f64, 11.67_f64));
        a.del_float_range((8.45_f64, 15_f64));
        a.del_float_range((19.5_f64, 50_f64));
        a.del_float_range((17.55_f64, 18.01_f64));

        data = a.get_float_ranges();

        assert_eq!(ans.len(), data.len());
        for i in 0..ans.len() { assert!( range_approx_eq(ans[i], data[i]) ) }
    }

    #[test]
    fn join_str_set() {
        let mut a = Variable::new();
        let s: String = String::from("abcdefghijk");

        for i in s.chars() {
            a.join_str_set(i);
        }

        for i in s.chars() {
            assert_eq!(false, a.join_str_set(i));
        }
        assert_eq!(a.join_str_set('z'), true);

        assert_eq!(a.get_str_set(), &String::from("abcdefghijkz"));
    }

    #[test]
    fn del_str_set() {
        let mut a = Variable::new();
        let s: String = String::from("abcdefghijk");

        for i in s.chars() {
            a.join_str_set(i);
        }

        a.del_str_set('a');
        a.del_str_set('g');
        a.del_str_set('k');
        assert_eq!(a.del_str_set('j'), true);
        assert_eq!(a.del_str_set('a'), false);

        assert_eq!(a.get_str_set(), &String::from("bcdefhi"));
    }

    #[test]
    fn set_quantity() {
        let mut a = Variable::new();
        
        a.set_quantity(15);

        assert_eq!(15, a.get_quanity());
    }

    #[test]
    fn set_end_char() {
        let mut a = Variable::new();

        a.set_end_char(String::from("abc"));

        assert_eq!(a.get_end_char(), &String::from("abc"));
    }

    #[test]
    fn set_to_constant() {
        let mut a = Variable::new();

        a.set_to_constant(String::from("CONSTANT_VALUE"));

        assert_eq!(a.get_const_str(), &String::from("CONSTANT_VALUE"));
        assert_eq!(a.get_constant(), true);
    }

    #[test]
    fn set_to_variable() {
        let mut a = Variable::new();

        a.set_to_variable();

        assert_eq!(a.get_constant(), false);
    }

    #[test]
    fn set_float_precision() {
        let mut a = Variable::new();

        a.set_float_precision(5);

        assert!( approx_eq(a.get_flaot_precision(), 0.00001_f64) );
    }
}
