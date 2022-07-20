#[cfg(test)]
use mockall::automock;
use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;

#[cfg_attr(test, automock)]
pub trait CanvasRenderingContext2d {
    fn draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        &self,
        image: &HtmlImageElement,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
}

impl CanvasRenderingContext2d for web_sys::CanvasRenderingContext2d {
    fn draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        &self,
        image: &HtmlImageElement,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue> {
        web_sys::CanvasRenderingContext2d::draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            self,
            image,
            sx,
            sy,
            sw,
            sh,
            dx,
            dy,
            dw,
            dh,
        )
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = requestAnimationFrame)]
    fn request_animation_frame(f: &Closure<dyn FnMut()>) -> usize;
}

#[cfg_attr(test, automock)]
pub mod animation_frame {
    use wasm_bindgen::prelude::Closure;

    #[allow(dead_code)]
    pub fn request_animation_frame(f: &Closure<dyn FnMut()>) -> usize {
        super::request_animation_frame(f)
    }
}
