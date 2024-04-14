use std::io::BufRead;

pub mod raw {
    pub trait LClassifier: Fn(&crate::CpCounterFVec) -> Vec<crate::LangName> {}
    impl<T> LClassifier for T where T: Fn(&crate::CpCounterFVec) -> Vec<crate::LangName> {}
}

pub trait LReco: Fn(&mut dyn BufRead) -> anyhow::Result<Vec<crate::LangName>> {}
impl<T> LReco for T where T: Fn(&mut dyn BufRead) -> anyhow::Result<Vec<crate::LangName>> {}
