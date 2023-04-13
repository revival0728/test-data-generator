#![allow(dead_code)]

use rand::distributions::uniform::{ UniformInt, UniformFloat, UniformSampler };
use rand;

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

    fn new(quantity: u64) -> Self {
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

    pub fn gen_s(&mut self, ss: &String) -> Result<(), ()>{
        if self.s_res.len() != 0 { return Err(()); }

        let gen: UniformInt<usize> = UniformInt::new(0, ss.len());
        let mut rng = rand::thread_rng();
        let mut chars = ss.chars();
        self.s_weight = ss.len() as u128;
        for _ in 0..self.quantity {
            let id = gen.sample(&mut rng);

            self.s_res.push(chars.nth(id).unwrap());
        }

        Ok(())
    }

    pub fn res(&mut self) -> Result<&Vec<GenerResult>, ()> {

        Ok(&self.res)
    }
}
