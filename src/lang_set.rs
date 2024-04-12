pub mod err;
use std::sync::Arc;

pub type LangName = Arc<str>;

#[derive(Default)]
pub struct LangSet {
    /// # Invariants
    /// Has same `.len()` as [`Self::names`].
    ids: elsa::FrozenMap<LangName, Box<usize>>,
    /// # Invariants
    /// Has `.len()` <= [`Self::MAX_LEN`].
    names: elsa::FrozenVec<LangName>,
}

impl LangSet {
    // Associates

    pub const MAX_LEN: usize = crate::SUPPORTED_LANG_COUNT;
    // CRUD-U: Update

    /// Pushes the language into `self`. Returns id that was assigned to it.
    pub fn push<L>(&self, new_lang_name: L) -> Result<usize, err::LangSetIsFull>
    where
        L: Into<LangName>,
    {
        let new_lang_name: LangName = new_lang_name.into();
        self.push_lang_name(new_lang_name)
    }

    /// Pushes the language into `self`. Returns id that was assigned to it.
    pub fn push_lang_name(&self, new_lang_name: LangName) -> Result<usize, err::LangSetIsFull> {
        self.ensure_not_full()?;

        let assigned_id = self.names.len();
        self.ids.insert(new_lang_name.clone(), assigned_id.into());
        self.names.push(new_lang_name);

        Ok(assigned_id)
    }

    // CRUD-R: Getters

    pub fn len(&self) -> usize {
        self.names.len()
    }

    // CRUD-R: Properties of `self`

    pub fn ensure_id_exists(&self, id: usize) -> Result<(), err::LangIdNotFound> {
        if id < self.len() {
            Ok(())
        } else {
            Err(err::LangIdNotFound { requested_id: id })
        }
    }
    pub fn ensure_not_full(&self) -> Result<(), err::LangSetIsFull> {
        if self.is_len_maximal() {
            Err(err::LangSetIsFull)
        } else {
            Ok(())
        }
    }
    pub fn is_len_maximal(&self) -> bool {
        self.len() == Self::MAX_LEN
    }

    // CRUD-R: Counceling

    pub fn id_to_code(&self, id: usize) -> Result<crate::BitSeq, err::LangIdNotFound> {
        self.ensure_id_exists(id)?;
        let mut bs: crate::BitSeq = 1;
        bs = bs
            .checked_shl(id.try_into().unwrap())
            .expect("`id` exists ==> `id` < `Self::MAX_LEN` <= `BitSeq::BITS`");
        Ok(bs)
    }

    // CRUD-R: Indexers

    /// Returns the name of the language for a given ID.
    pub fn id_to_name(&self, id: usize) -> Result<&str, err::LangIdNotFound> {
        self.names
            .get(id)
            .ok_or(err::LangIdNotFound { requested_id: id })
    }
    /// Returns the ID of the language for a given name.
    pub fn name_to_id<'n>(&self, name: &'n str) -> Result<usize, err::LangNameNotFound<&'n str>> {
        self.ids.get(name).copied().ok_or(err::LangNameNotFound {
            requested_name: name,
        })
    }

    // CRUD-R: Compound foreign conversions

    /// Converts a language name to its corresponding `BitSeq` representation.
    pub fn name_to_code<'n>(
        &self,
        name: &'n str,
    ) -> Result<crate::BitSeq, err::LangNameNotFound<&'n str>> {
        let id = self.name_to_id(name)?;
        Ok(self
            .id_to_code(id)
            .expect("Id should be valid since it's returned by name_to_id mapper."))
    }

    pub fn decode(&self, lang_codes: crate::BitSeq) -> Vec<LangName> {
        let mut languages = Vec::new();
        for id in 0..self.len() {
            let lang_code: crate::BitSeq = 1 << id;
            if lang_codes & lang_code != 0 {
                let lang_name = self
                    .id_to_name(id)
                    .expect("Should be valid since it is < `self.len().`");
                languages.push(lang_name.into());
            }
        }
        languages
    }
}
