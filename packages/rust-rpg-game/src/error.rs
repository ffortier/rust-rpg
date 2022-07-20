use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum WebError {
    #[error("no window")]
    NoWindow,
    #[error("no document")]
    NoDocument,
    #[error("unknown {0}")]
    Unknown(String),
    #[error("invalid cast into {0}")]
    Cast(&'static str),
    #[error("expected value {0}")]
    ExpectedValue(&'static str),
}

impl From<JsValue> for WebError {
    fn from(val: JsValue) -> Self {
        WebError::Unknown(format!("{val:#?}"))
    }
}
