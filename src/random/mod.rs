#![allow(dead_code)]

use rand::distributions::uniform::{ UniformInt, UniformFloat, UniformSampler };
use rand;
use rand::seq::SliceRandom;

pub enum GenerResult {
    I(i64),
    F(f64),
    S(char)
}

pub struct Gener {
    i_res: Vec<i64>,
    f_res: Vec<f64>,
    s_res: Vec<char>,
    i_weight: u128,
    f_weight: u128,
    s_weight: u128,
    res: Vec<GenerResult>,
    quantity: u64
}

impl Gener {

    pub fn new(quantity: u64) -> Self {
        let i_res: Vec<i64> = Vec::new();
        let f_res: Vec<f64> = Vec::new();
        let s_res: Vec<char> = Vec::new();
        let i_weight: u128 = 0;
        let f_weight: u128 = 0;
        let s_weight: u128 = 0;
        let res: Vec<GenerResult> = Vec::new();

        Gener { i_res, f_res, s_res, i_weight, f_weight, s_weight, res, quantity }
    }

    pub fn gen_i(&mut self, vi: &Vec<(i64, i64)>) -> Result<(), ()> {
        if self.i_res.len() != 0 { return Err(()); }

        let mut weight: Vec<u128> = Vec::new();

        for i in vi.iter() { 
            let w: u128 = (i.1 - i.0).try_into().unwrap();
            weight.push(w); 
            self.i_weight += w;
        }

        for (id, i) in vi.iter().enumerate() {
            let gen: UniformInt<i64> = UniformInt::new_inclusive(i.0, i.1);
            let mut rng = rand::thread_rng();
            let q: u64 = ((self.quantity as f64 * weight[id] as f64) / (self.i_weight as f64)).round() as u64;

            for _ in 0..q {
                self.i_res.push(gen.sample(&mut rng));
            }
        }

        Ok(())
    }

    pub fn gen_f(&mut self, vf: &Vec<(f64, f64)>, pn: f64) -> Result<(), ()> {
        if self.f_res.len() != 0 { return Err(()); }

        let mut weight: Vec<u128> = Vec::new();

        for i in vf.iter() { 
            let w: u128 = (i.1 - i.0).round() as u128;
            weight.push(w); 
            self.f_weight += w;
        }

        for (id, i) in vf.iter().enumerate() {
            let gen: UniformFloat<f64> = UniformFloat::new_inclusive(i.0, i.1);
            let mut rng = rand::thread_rng();
            let q: u64 = ((self.quantity as f64 * weight[id] as f64) / (self.i_weight as f64)).round() as u64;

            for _ in 0..q {
                self.f_res.push((gen.sample(&mut rng) / pn).round() * pn);
            }
        }

        Ok(())
    }

    pub fn gen_s(&mut self, ss: &String) -> Result<(), ()> {
        if self.s_res.len() != 0 { return Err(()); }

        let gen: UniformInt<usize> = UniformInt::new(0, ss.len());
        let mut rng = rand::thread_rng();
        self.s_weight = ss.len() as u128;
        for _ in 0..self.quantity {
            let id = gen.sample(&mut rng);

            self.s_res.push(ss.chars().nth(id).unwrap());
        }

        Ok(())
    }

    pub fn res(&mut self) -> &Vec<GenerResult> {
        if self.res.len() != 0 { return &self.res }

        let total_weight = self.i_weight + self.f_weight + self.s_weight;

        {
            let q = (self.quantity as f64 * self.i_weight as f64 / total_weight as f64).ceil() as u128;

            let gen: UniformInt<usize> = UniformInt::new(0, self.i_res.len());
            let mut rng = rand::thread_rng();
            for _ in 0..q {
                self.res.push(GenerResult::I(self.i_res[gen.sample(&mut rng)]));
            }
        }

        {
            let q = (self.quantity as f64 * self.f_weight as f64 / total_weight as f64).ceil() as u128;

            let gen: UniformInt<usize> = UniformInt::new(0, self.f_res.len());
            let mut rng = rand::thread_rng();
            for _ in 0..q {
                self.res.push(GenerResult::F(self.f_res[gen.sample(&mut rng)]));
            }
        }

        {
            let q = (self.quantity as f64 * self.s_weight as f64 / total_weight as f64).ceil() as u128;

            let gen: UniformInt<usize> = UniformInt::new(0, self.s_res.len());
            let mut rng = rand::thread_rng();
            for _ in 0..q {
                self.res.push(GenerResult::S(self.s_res[gen.sample(&mut rng)]));
            }
        }

        let mut rng = rand::thread_rng();
        self.res.shuffle(&mut rng);
        
        while self.res.len() > self.quantity as usize { self.res.pop(); }
        
        &self.res
    }
}

#[cfg(test)]
mod test_gener{

    use super::Gener;
    use super::GenerResult;

    #[test]
    fn rand() {
        let quantity: u64 = 30;
        let mut gen = Gener::new(quantity);
        let vi: Vec<(i64, i64)> = vec![(1, 100)];
        let vf: Vec<(f64, f64)> = vec![(1.1, 5.8), (10.0, 45.8)];
        let precision: f64 = 0.0001;
        let ss: String = String::from("@#$%^_{}ajeibpxn");

        match gen.gen_i(&vi) {
            Ok(_) => {},
            Err(_) => { println!("test_gener::rand(): gen_i() error"); }
        }
        match gen.gen_f(&vf, precision) {
            Ok(_) => {},
            Err(_) => { println!("test_gener::rand(): gen_f() error"); }
        }
        match gen.gen_s(&ss) {
            Ok(_) => {},
            Err(_) => { println!("test_gener::rand(): gen_s() error"); }
        }
        
        for i in gen.res().iter() {
            print!("{} ", match i {
                GenerResult::I(v) => { v.to_string() }
                GenerResult::F(v) => { v.to_string() }
                GenerResult::S(v) => { v.to_string() }
            });
        }
        println!();

        assert_eq!(gen.res().len(), 30);
    }
}
