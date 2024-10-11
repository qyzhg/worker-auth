use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub(crate) struct Claims {
    pub(crate) sub: String,
    pub(crate) group: String,
    pub(crate) exp: usize,
}