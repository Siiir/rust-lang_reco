use std::io::Read;

use anyhow::Context;
use derive_more::{Constructor, Deref, DerefMut};

#[derive(Constructor, Debug, Deref, DerefMut)]
pub struct Cfg {
    args: crate::Args,
}
impl Cfg {
    pub fn input_reader(&self) -> anyhow::Result<Box<dyn Read>> {
        match self.args.input_f_path {
            None => Ok(Box::new(std::io::stdin().lock())),
            Some(ref path) => {
                let app_input_file = std::fs::File::open(path).with_context(|| {
                    format!("Failed to open the `app input file` at \"{path}\".")
                })?;
                Ok(Box::new(app_input_file))
            }
        }
    }
}
