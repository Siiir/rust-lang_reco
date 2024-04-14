pub mod err {
    use derive_more::{From, Into};

    #[derive(thiserror::Error, From, Debug, Into)]
    #[error("Provided language set doesn't containt id={missing_id}.", missing_id = .0.requested_id)]
    pub struct UncoveredLangId(#[source] crate::lang_set::err::LangIdNotFound);
}

use std::io::BufRead;

use anyhow::Context;
use num_rational::Ratio;

/// Counts codepoints read from reader.
pub fn count_codepoints<R: BufRead>(buf_in_reader: R) -> anyhow::Result<crate::CpCounter> {
    let mut counter = [0; crate::CP_KINDS_COUNT];
    for (idx, codepoint) in buf_in_reader.bytes().enumerate() {
        let codepoint =
            codepoint.with_context(|| format!("Failed to read byte with index = {idx}."))?;
        if is_codepoint_vital(codepoint) {
            let normalized_codepoint = normalize_codepoint(codepoint);
            let counter_idx: usize = normalized_codepoint.into();
            counter[counter_idx] += 1;
        }
    }
    Ok(counter)
}

fn is_codepoint_vital(codepoint: crate::CodePoint) -> bool {
    codepoint.is_ascii_alphabetic()
}
fn normalize_codepoint(codepoint: crate::CodePoint) -> u8 {
    codepoint.to_ascii_lowercase() - b'a'
}

/// Trains neural network.
///
/// # Panics
/// If [`lang_set`] doesn't cover any id in [`text_descriptors`].
pub fn train_nn(
    nn: &mut perc_ic::OneLayerNN<{ crate::SUPPORTED_LANG_COUNT }, { crate::CP_KINDS_COUNT }>,
    lang_set: &crate::LangSet,
    mut text_descriptors: Vec<crate::TextDescrFBased>,
) {
    // Train NN
    use rand::prelude::*;
    text_descriptors.shuffle(&mut rand::thread_rng());

    nn.fit_to::<_, _, crate::BitSeq, crate::BitSeq>(
        text_descriptors.iter().map(|td| &td.cp_counter),
        text_descriptors.iter().map(|td| {
            lang_set
                .id_to_code(td.lang_id)
                .map_err(err::UncoveredLangId::from)
                .expect("Provided `lang_set` should cover ids in provided `text_descriptors`.")
        }),
        10,
    );
}
pub fn measure_accuracy(lang_classifier: impl crate::LClassifier) -> anyhow::Result<Ratio<usize>> {
    use crate::PATH_TO_TEST_DATA;

    // Reading data
    let (lang_set, text_descrs) = crate::data::read(PATH_TO_TEST_DATA)?;

    // Counting good, bad
    let count = text_descrs.len();
    if count == 0 {
        return Err(anyhow::anyhow!(
            "There are no testing language samples in \"{PATH_TO_TEST_DATA}\"."
        ));
    }
    let mut sum: Ratio<_> = num_traits::zero();
    for crate::TextDescrFBased {
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

pub fn predict_user_provided_lang(
    l_classifier: impl crate::LClassifier,
    app_cfg: crate::Cfg,
) -> anyhow::Result<()> {
    let l_reco = crate::from_classifier(l_classifier);
    let mut buf_reader = std::io::BufReader::new(app_cfg.input_reader()?);
    let pred_langs = l_reco(&mut buf_reader)?;

    let preamble = format!(
        "{opt_nl}Language of the provided text",
        opt_nl = if app_cfg.is_input_stdin() { "\n" } else { "" }
    );
    match pred_langs[..] {
        [] => println!("{preamble} couldn't be determined."),
        [ref lang] => println!("{preamble} is {lang}."),
        ref langs => print!("{preamble} is one of:\n* {}.\n", langs.join(",\n* ")),
    }
    Ok(())
}
