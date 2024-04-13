use std::io::BufRead;

use anyhow::Context;

/// Counts codepoints read from reader.
pub fn count_codepoints<R: BufRead>(buf_in_reader: R) -> anyhow::Result<crate::CpCounter> {
    let mut counter = [0; crate::CP_KINDS_COUNT];
    for (idx, codepoint) in buf_in_reader.bytes().enumerate() {
        let counter_idx: usize = codepoint
            .with_context(|| format!("Failed to read byte with index = {idx}."))?
            .into();
        counter[counter_idx] += 1;
    }
    Ok(counter)
}

/// Trains neural network.
pub fn train_nn(
    nn: &mut perc_ic::OneLayerNN<{ crate::SUPPORTED_LANG_COUNT }, { crate::CP_KINDS_COUNT }>,
    lang_set: &crate::LangSet,
    mut text_describtors: Vec<crate::TextDescrFBased>,
) {
    // Train NN
    use rand::prelude::*;
    text_describtors.shuffle(&mut rand::thread_rng());

    nn.fit_to::<_, _, crate::BitSeq, crate::BitSeq>(
        text_describtors.iter().map(|td| &td.cp_counter),
        text_describtors
            .iter()
            .map(|td| lang_set.id_to_code(td.lang_id).unwrap()),
        10,
    );
}
