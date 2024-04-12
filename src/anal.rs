use std::io::BufRead;

/// Counts codepoints read from reader.
pub fn count_codepoints<R: BufRead>(buf_in_reader: R) -> crate::CpCounter {
    let mut counter = [0; crate::CP_KINDS_COUNT];
    for byte in buf_in_reader.bytes() {
        let counter_idx: usize = byte.unwrap().into();
        counter[counter_idx] += 1;
    }
    counter
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
