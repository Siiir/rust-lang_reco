use std::io::BufRead;

pub use args::Args;
pub mod args;
pub use cfg::Cfg;
pub mod cfg;
pub use lang_set::{LangName, LangSet};
pub mod lang_set;
pub use text_descr::{float::TextDescrFBased, TextDescr};
pub mod anal;
pub mod data;
pub mod text_descr;
pub use paths::*;
pub mod paths {
    pub const PATH_TO_DATA: &str = "./data";
}

pub const SUPPORTED_LANG_COUNT_U8: u8 = 4;
pub const SUPPORTED_LANG_COUNT: usize = SUPPORTED_LANG_COUNT_U8 as usize;
pub const CP_KINDS_COUNT: usize = 255;

pub type BitSeq = usize;
pub type CpCount = usize;
pub type CpCountFloat = perc_ic::perceptron::PerFloat;
pub type CpCounter = [CpCount; CP_KINDS_COUNT];
pub type CpCounterFVec = perc_ic::perceptron::PerVec<CP_KINDS_COUNT>;

const _BIT_SEQ_LONG_ENOUGH: () = assert!(SUPPORTED_LANG_COUNT_U8 as u32 <= BitSeq::BITS);

#[macro_export]
macro_rules! exe_doc {
    () => {
        "App for guessing language of the provided text.\n\
        \n\
        You can provide some UTF-8 through stdin \
         or by giving path to a text file as the first argument.\n\
        The language recognizer should tell you what language \
         it thinks your text is written in."
    };
}

/// Creates the language recognizer.
pub fn create() -> anyhow::Result<impl Fn(&mut dyn BufRead) -> Vec<LangName>> {
    // Building neural network and lang. set.
    let mut nn = perc_ic::OneLayerNN::default();
    let (lang_set, text_describtors): (LangSet, Vec<TextDescrFBased>) = data::read();
    anal::train_nn(&mut nn, &lang_set, text_describtors);
    // The classifier
    Ok(move |buf_reader: &mut dyn BufRead| {
        let counter: CpCounterFVec = anal::count_codepoints(buf_reader)
            .map(|i| i as crate::CpCountFloat)
            .into();
        let lang_codes: BitSeq = nn.decide_for(&counter);
        lang_set.decode(lang_codes)
    })
}
