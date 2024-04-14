use thiserror::Error;

#[derive(Error, Clone, Copy, Debug)]
#[error("Language ID {requested_id} not found.")]
pub struct LangIdNotFound {
    pub requested_id: usize,
}

#[derive(Error, Debug)]
#[error("Language name \"{requested_name}\" not found.")]
pub struct LangNameNotFound<S> {
    pub requested_name: S,
}

#[derive(Error, Debug)]
#[error(
    "Language set is full. `Self::MAX_LEN` == {} .",
    crate::LangSet::MAX_LEN
)]
pub struct LangSetIsFull;
