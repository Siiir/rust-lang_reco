use std::io::BufRead;

use anyhow::Context;

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
pub use ty::{raw::LClassifier, LReco};
pub mod ty;
pub use paths::*;
pub mod paths {
    pub const PATH_TO_TRAIN_DATA: &str = "./data/train";
    pub const PATH_TO_TEST_DATA: &str = "./data/test";
}

pub const SUPPORTED_LANG_COUNT_U8: u8 = 4;
pub const SUPPORTED_LANG_COUNT: usize = SUPPORTED_LANG_COUNT_U8 as usize;
pub const CP_KINDS_COUNT: usize = (b'z' - b'a' + 1) as usize;

pub type BitSeq = usize;
pub type CodePoint = u8;
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
         it thinks your text is written in.\n\
        \n\
        Example usage from bash (for executable named \"lang_reco\"):\n\
        * `echo hahaha | lang_reco -A`\n\
        * `lang_reco <<< ''`\n\
        * `lang_reco`\n\
        * `lang_reco -A ./data/test/Polish/1.txt`\n\
        * `cat ./data/test/English/1.txt | lang_reco -A`\n\
        "
    };
}

/// Creates the language recognizer.
pub fn create() -> anyhow::Result<impl crate::LReco> {
    let classifier = create_classifier().context("Failed to create language recognizer.")?;
    Ok(from_classifier(classifier))
}
pub fn from_classifier(classify: impl crate::LClassifier) -> impl crate::LReco {
    move |buf_reader: &mut dyn BufRead| {
        let counter: CpCounterFVec = anal::count_codepoints(buf_reader)
            .context("Language recognizer couldn't analyze the input.")?
            .map(|i| i as crate::CpCountFloat)
            .into();
        Ok(classify(&counter))
    }
}
/// Creates the raw language recognizer that operates .
pub fn create_classifier() -> anyhow::Result<impl crate::LClassifier> {
    let mut res = (|| {
        // Building neural network and lang. set.
        let mut nn = perc_ic::OneLayerNN::default();
        let (lang_set, text_describtors): (LangSet, Vec<TextDescrFBased>) =
            data::read(PATH_TO_TRAIN_DATA)?;
        anal::train_nn(&mut nn, &lang_set, text_describtors);
        Ok(
            move |cp_counter: &crate::CpCounterFVec| -> Vec<crate::LangName> {
                let lang_codes: BitSeq = nn.decide_for(cp_counter);
                lang_set.decode(lang_codes)
            },
        )
    })();
    res = res.context("Failed to create a language classifier.");
    res
}

pub fn opt_run_accuracy_measure(
    app_cfg: &crate::Cfg,
    lang_reco: impl crate::LClassifier,
) -> anyhow::Result<()> {
    if app_cfg.accuracy_measure() {
        let measure = anal::measure_accuracy(lang_reco)
            .context("Optional accuracy measure failed. Run app with '-A' to disable it.")?;
        eprintln!("Language recognizer accuracy is {measure}.\nMeasured for sample texts in \"{PATH_TO_TEST_DATA}\".\n")
    }
    Ok(())
}
