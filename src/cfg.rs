use std::io::Read;

use derive_more::{Constructor, Deref, DerefMut};

#[derive(Constructor, Debug, Deref, DerefMut)]
pub struct Cfg {
    args: crate::Args,
}
impl Cfg {
    pub fn input_reader(&self) -> Result<Box<dyn Read>, ()> {
        match self.args.input_f_path {
            None => Ok(Box::new(std::io::stdin().lock())),
            Some(ref path) => Ok(Box::new(std::fs::File::open(path).unwrap())),
        }
    }
}
