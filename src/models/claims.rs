use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub(crate) struct Claims {
    pub(crate) id: usize,
    pub(crate) sub: String,
    pub(crate) group: String,
    pub(crate) email: String,
    pub(crate) exp: usize,
}