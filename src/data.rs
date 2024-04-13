pub mod err {
    pub struct NotADir;
}

use std::{ops::ControlFlow, path::Path};

use anyhow::{Context, Result};

use crate::{LangSet, TextDescr, TextDescrFBased, PATH_TO_TRAIN_DATA};

/// Reads and analyzer the data in [`crate::PATH_TO_DATA`].
pub fn read<P: AsRef<Path>>(path_to_data: P) -> anyhow::Result<(LangSet, Vec<TextDescrFBased>)> {
    fn read(path_to_data: &Path) -> anyhow::Result<(LangSet, Vec<TextDescrFBased>)> {
        let mut res = (|| {
            let lang_set = crate::LangSet::default();
            let mut text_describtors = Vec::<TextDescr>::new();

            // Iterating over languages.
            for should_be_lang_dir in std::fs::read_dir(path_to_data)? {
                let should_be_lang_dir =
                    should_be_lang_dir.context("Failed to read next directory entry.")?;
                // Read only valid dirs while ignoring every other dir entry for simplicity.
                if let Ok(file_type) = should_be_lang_dir.file_type() {
                    if file_type.is_dir() {
                        read_choosen_lang_samples(
                            should_be_lang_dir,
                            &lang_set,
                            &mut text_describtors,
                        )?;
                    }
                }
            }

            // Adapt text descriptors to current NN impl.
            let text_describtors: Vec<TextDescrFBased> =
                crate::text_descr::map_to_f_based(text_describtors);
            Ok((lang_set, text_describtors))
        })();
        res = res.with_context(|| {
            format!(
                "Failed to read/interpret data (language samples) from \"{:}\".",
                path_to_data.display()
            )
        });
        res
    }
    read(path_to_data.as_ref())
}

fn read_choosen_lang_samples(
    lang_dir: std::fs::DirEntry,
    lang_set: &LangSet,
    text_describtors: &mut Vec<TextDescr>,
) -> anyhow::Result<()> {
    let lang_dir_path = lang_dir.path();
    let lang_name = &lang_dir.file_name();
    let lang_name: &str = lang_name
        .to_str()
        .context("Not valid in UTF-8 encoding.")
        .with_context(|| {
            format!(
                "Failed to translate the name of directory entry to a name of natural language."
            )
        })?;
    use crate::SUPPORTED_LANG_COUNT;
    let lang_id = lang_set
            .push_lang_name(lang_name.into())
            .map_err(|_set_is_full| {
                anyhow::anyhow!(
                    "App was compiled with support enable for up to {SUPPORTED_LANG_COUNT} but more `language entries` have been found. Help: remove `directory entry` \"{lang_dir_path}\" or another. Note: Different app distributions might have different capacity.",
                    lang_dir_path = lang_dir_path.display(),
                )
            })?;

    let mut res = (|| {
        for lang_text_file in std::fs::read_dir(&lang_dir_path)? {
            let lang_text_file = lang_text_file.context("Failed to read next directory entry.")?;
            // Read only valid files while ignoring every other dir entry for simplicity.
            if let Ok(file_type) = lang_text_file.file_type() {
                if file_type.is_file() {
                    read_lang_sample(lang_text_file, lang_id, text_describtors)?;
                }
            }
        }
        Ok(())
    })();
    res = res.with_context(|| {
        format!(
            "Failed to read/interpret samples of \"{lang_name}\" language from {path}\".",
            path = lang_dir_path.display()
        )
    });
    res
}

fn read_lang_sample(
    lang_text_file: std::fs::DirEntry,
    lang_id: usize,
    text_describtors: &mut Vec<TextDescr>,
) -> anyhow::Result<()> {
    let lang_text_path = lang_text_file.path();
    let cp_counter = (|| {
        crate::anal::count_codepoints(std::io::BufReader::new(std::fs::File::open(
            lang_text_path,
        )?))
    })()
    .with_context(|| {
        format!(
            "Failed to read (lang-text sample) file \"{path}\".",
            path = lang_text_path.display()
        )
    })?;
    let text_descr = TextDescr {
        lang_id,
        cp_counter,
    };

    text_describtors.push(text_descr);

    Ok(())
}
