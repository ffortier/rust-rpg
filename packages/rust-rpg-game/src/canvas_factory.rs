use anyhow::{Context, Result};
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

use crate::error::WebError;

#[derive(Default)]
pub struct CanvasFactory {}

#[cfg_attr(test, mockall::automock)]
impl CanvasFactory {
    pub fn create_canvas(&self, width: u32, height: u32) -> Result<HtmlCanvasElement> {
        let canvas = window()
            .ok_or(WebError::NoWindow)?
            .document()
            .ok_or(WebError::NoDocument)?
            .create_element("canvas")
            .map_err(WebError::from)
            .context("creating canvas element")?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| WebError::Cast("HtmlCanvasElement"))?;

        Ok(canvas)
    }

    pub fn create_canvas_with_2d_context(
        &self,
        width: u32,
        height: u32,
    ) -> Result<(HtmlCanvasElement, CanvasRenderingContext2d)> {
        let canvas = self.create_canvas(width, height)?;

        let rendering_context = canvas
            .get_context("2d")
            .map_err(WebError::from)
            .context("getting 2d context")?
            .ok_or_else(|| WebError::ExpectedValue("2d context"))?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .map_err(|_| WebError::Cast("CanvasRenderingContext2d"))?;

        Ok((canvas, rendering_context))
    }
}
