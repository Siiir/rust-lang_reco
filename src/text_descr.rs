use crate::TextDescrFBased;

/// Text descriptor.
pub struct TextDescr {
    pub lang_id: usize,
    pub cp_counter: crate::CpCounter,
}
pub fn map_to_f_based(text_descriptors: Vec<TextDescr>) -> Vec<TextDescrFBased> {
    let text_descriptors: Vec<TextDescrFBased> = text_descriptors
        .into_iter()
        .map(TextDescrFBased::from)
        .collect();
    text_descriptors
}

pub mod float {
    use crate::CpCountFloat;

    use super::TextDescr;

    pub struct TextDescrFBased {
        pub lang_id: usize,
        pub cp_counter: crate::CpCounterFVec,
    }

    impl From<TextDescr> for TextDescrFBased {
        fn from(value: TextDescr) -> Self {
            let TextDescr {
                lang_id,
                cp_counter,
            } = value;
            TextDescrFBased {
                lang_id,
                cp_counter: cp_counter.map(|cp_count| cp_count as CpCountFloat).into(),
            }
        }
    }
}
