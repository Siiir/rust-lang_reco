use crate::{LangSet, TextDescr, TextDescrFBased, PATH_TO_DATA};

/// Reads and analyzer the data in [`crate::PATH_TO_DATA`].
pub fn read() -> (LangSet, Vec<TextDescrFBased>) {
    let lang_set = crate::LangSet::default();
    let mut text_describtors = Vec::<TextDescr>::new();

    // Iterating over languages.
    for lang_dir in std::fs::read_dir(PATH_TO_DATA).unwrap() {
        let lang_dir = lang_dir.unwrap();
        if !lang_dir.file_type().unwrap().is_dir() {
            continue;
        }

        let lang_name = lang_dir.file_name();
        let lang_name: &str = lang_name.to_str().unwrap();
        let lang_id = lang_set.push_lang_name(lang_name.into()).unwrap();

        for lang_text_file in std::fs::read_dir(PATH_TO_DATA).unwrap() {
            let lang_text_file = lang_text_file.unwrap();
            if !lang_text_file.file_type().unwrap().is_file() {
                continue;
            }

            let cp_counter = crate::anal::count_codepoints(std::io::BufReader::new(
                std::fs::File::open(lang_text_file.path()).unwrap(),
            ));

            let text_descr = TextDescr {
                lang_id,
                cp_counter,
            };

            text_describtors.push(text_descr);
        }
    }

    // Adapt text descriptors to current NN impl.
    let text_describtors: Vec<TextDescrFBased> =
        crate::text_descr::map_to_f_based(text_describtors);
    (lang_set, text_describtors)
}
