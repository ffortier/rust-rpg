use anyhow::{Context, Result};
use web_sys::HtmlImageElement;

use crate::{error::WebError, js_adapter::CanvasRenderingContext2d};

pub const TILE_SIZE: u32 = 32;
pub const TILE_SIZE_F64: f64 = TILE_SIZE as f64;

pub struct Renderer<T>
where
    T: CanvasRenderingContext2d,
{
    context: T,
    tileset: HtmlImageElement,
}

impl<T> Renderer<T>
where
    T: CanvasRenderingContext2d,
{
    pub fn new(context: T, tileset: HtmlImageElement) -> Self {
        Self { context, tileset }
    }

    pub fn draw_tile(&self, idx: u32, (dx, dy): (f64, f64)) -> Result<()> {
        let cols = self.tileset.width() / TILE_SIZE;
        let row = idx / cols;
        let col = idx % cols;

        self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &self.tileset,
                col as f64 * TILE_SIZE_F64,
                row as f64 * TILE_SIZE_F64,
                TILE_SIZE_F64,
                TILE_SIZE_F64,
                dx,
                dy,
                TILE_SIZE_F64,
                TILE_SIZE_F64,
            )
            .map_err(WebError::from)
            .context("drawing image")
    }
}

#[cfg(test)]
mod test {
    use js_sys::{eval, Reflect};
    use wasm_bindgen::{JsCast, JsValue};

    use super::*;
    use crate::js_adapter::MockCanvasRenderingContext2d;
    use mockall::predicate::*;
    use wasm_bindgen_test::*;

    fn setup() -> Renderer<MockCanvasRenderingContext2d> {
        let tileset = eval("({width: 320, height: 96, id: 'tileset'})").expect("eval to object");

        Renderer {
            context: MockCanvasRenderingContext2d::new(),
            tileset: tileset.unchecked_into(),
        }
    }

    fn get(val: &JsValue, prop: &str) -> JsValue {
        Reflect::get(val, &prop.into()).unwrap()
    }

    fn get_string(val: &JsValue, prop: &str) -> String {
        get(val, prop).as_string().unwrap()
    }

    #[wasm_bindgen_test]
    fn it_should_draw_a_sprite_from_the_tileset() {
        let mut renderer = setup();

        renderer
            .context
            .expect_draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh()
            .times(1)
            .with(
                function(|img: &HtmlImageElement| get_string(img, "id") == "tileset"),
                eq(64.0),
                eq(0.0),
                eq(32.0),
                eq(32.0),
                eq(32.0),
                eq(64.0),
                eq(32.0),
                eq(32.0),
            )
            .return_const_st(Ok(()));

        renderer.draw_tile(2, (32.0, 64.0)).expect("draw tile");
    }
}
