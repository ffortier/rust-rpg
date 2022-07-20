use std::ops::Deref;

use anyhow::{Context, Result};
use mockall_double::double;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageData};

#[double]
use crate::canvas_factory::CanvasFactory;
use crate::error::WebError;
struct ConvexHull {
    canvas: HtmlCanvasElement,
    rendering_context: CanvasRenderingContext2d,
}

#[cfg_attr(test, mockall::automock)]
impl ConvexHull {
    fn new(canvas_width: u32, canvas_height: u32) -> Result<Self> {
        let (canvas, rendering_context) = CanvasFactory::default()
            .create_canvas_with_2d_context(canvas_width, canvas_height)
            .context("creating canvas and 2d context")?;

        return Ok(Self {
            canvas,
            rendering_context,
        });
    }

    pub fn wrap_image(&mut self, image: &HtmlImageElement) -> Result<Vec<(u32, u32)>> {
        let image_data = self.get_image_data(image)?;
        let mut all_points = self.get_all_points(&image_data);

        todo!()
    }

    fn get_image_data(&self, image: &HtmlImageElement) -> Result<ImageData> {
        let w = image.width() as f64;
        let h = image.height() as f64;

        self.rendering_context.clear_rect(0.0, 0.0, w, h);

        self.rendering_context
            .draw_image_with_html_image_element(image, 0.0, 0.0)
            .map_err(WebError::from)
            .context("drawing image")?;

        let image_data = self
            .rendering_context
            .get_image_data(0.0, 0.0, w, h)
            .map_err(WebError::from)
            .context("getting image data")?;

        Ok(image_data)
    }

    fn get_all_points(&self, image_data: &ImageData) -> Vec<(u32, u32)> {
        let mut all_points = vec![];

        for (i, rgba) in image_data.data().deref().chunks_exact(4).enumerate() {
            if rgba[3] > 0 {
                let x = i as u32 % image_data.width();
                let y = i as u32 / image_data.height();

                all_points.push((x, y));
            }
        }

        all_points
    }
}

impl Drop for ConvexHull {
    fn drop(&mut self) {
        self.canvas.remove();
    }
}
