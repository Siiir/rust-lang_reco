use std::io::BufRead;

pub use args::Args;
pub mod args;
pub use cfg::Cfg;
pub mod cfg;
pub use lang_set::{LangName, LangSet};
pub mod lang_set;
use num_rational::Ratio;
pub use text_descr::{float::TextDescrFBased, TextDescr};
pub mod anal;
pub mod data;
pub mod text_descr;
pub use paths::*;
pub mod paths {
    pub const PATH_TO_TRAIN_DATA: &str = "./data/train";
    pub const PATH_TO_TEST_DATA: &str = "./data/test";
}
pub use ty::{raw::LClassifier, LReco};
pub mod ty {
    use std::io::BufRead;

    pub mod raw {
        pub trait LClassifier: Fn(&crate::CpCounterFVec) -> Vec<crate::LangName> {}
        impl<T> LClassifier for T where T: Fn(&crate::CpCounterFVec) -> Vec<crate::LangName> {}
    }

    pub trait LReco: Fn(&mut dyn BufRead) -> Vec<crate::LangName> {}
    impl<T> LReco for T where T: Fn(&mut dyn BufRead) -> Vec<crate::LangName> {}
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
pub fn create() -> anyhow::Result<impl crate::LReco> {
    let classifier = create_classifier()?;
    from_classifier(classifier)
}
pub fn from_classifier(classifier: impl crate::LClassifier) -> anyhow::Result<impl crate::LReco> {
    Ok(move |buf_reader: &mut dyn BufRead| {
        let counter: CpCounterFVec = anal::count_codepoints(buf_reader)
            .map(|i| i as crate::CpCountFloat)
            .into();
        classifier(&counter)
    })
}
/// Creates the raw language recognizer that operates .
pub fn create_classifier() -> anyhow::Result<impl crate::LClassifier> {
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
}

pub fn run_accuracy_measure(lang_reco: impl crate::LClassifier) -> anyhow::Result<()> {
    let measure = measure_accuracy(lang_reco)?;
    dbg!(measure);
    Ok(())
}
fn measure_accuracy(lang_classifier: impl crate::LClassifier) -> anyhow::Result<Ratio<usize>> {
    // Reading data
    let (lang_set, text_descrs) = data::read(PATH_TO_TEST_DATA)?;

    // Counting good, bad
    let count = text_descrs.len();
    assert_ne!(count, 0);
    let mut sum: Ratio<_> = num_traits::zero();
    for TextDescrFBased {
        lang_id,
        cp_counter,
    } in text_descrs
    {
        let expected_lang_name = lang_set.id_to_name(lang_id).expect(
            "Lang. id was created with corresponding lang. set. Thus it should be compatile.",
        );
        let prediction = lang_classifier(&cp_counter);
        if prediction
            .iter()
            .any(|pred_lang_name| **pred_lang_name == *expected_lang_name)
        {
            sum += Ratio::new(1, prediction.len());
        }
    }

    Ok(sum / count)
}
