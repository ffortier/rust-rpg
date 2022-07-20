use anyhow::{Context, Result};
use thiserror::Error;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Blob, HtmlImageElement, Response, Url};

pub struct AssetsLoader {
    base_path: String,
}

#[derive(Debug, Error)]
pub enum AssetsLoaderError {
    #[error("no window")]
    NoWindow,
    #[error("no document")]
    NoDocument,
    #[error("unexpected")]
    Unexpected(String),
    #[error("bad element")]
    BadElement,
}

impl From<JsValue> for AssetsLoaderError {
    fn from(val: JsValue) -> Self {
        AssetsLoaderError::Unexpected(format!("{:?}", val).to_string())
    }
}

#[cfg_attr(test, mockall::automock)]
impl AssetsLoader {
    pub async fn load_image(name: &str) -> Result<HtmlImageElement> {
        let window = window().ok_or(AssetsLoaderError::NoWindow)?;
        let document = window.document().ok_or(AssetsLoaderError::NoDocument)?;
        let response = JsFuture::from(window.fetch_with_str(""))
            .await
            .and_then(|response| response.dyn_into::<Response>())
            .map_err(AssetsLoaderError::from)
            .with_context(move || format!("loading image {}", name))?;

        let blob = response
            .blob()
            .map_err(AssetsLoaderError::from)
            .context("getting blob")?;

        let blob = JsFuture::from(blob)
            .await
            .and_then(|blob| blob.dyn_into::<Blob>())
            .map_err(AssetsLoaderError::from)
            .context("reading blob")?;

        let url = Url::create_object_url_with_blob(&blob);

        let image = document
            .create_element("img")
            .map_err(AssetsLoaderError::from)
            .and_then(|image| {
                image
                    .dyn_into::<HtmlImageElement>()
                    .or(Err(AssetsLoaderError::BadElement))
            })
            .context("creating image")?;

        Ok(image)
    }
}

impl Default for AssetsLoader {
    fn default() -> Self {
        Self {
            base_path: "/assets".to_string(),
        }
    }
}
