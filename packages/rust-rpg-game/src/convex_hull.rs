use std::ops::Deref;

use anyhow::{Context, Result};
use mockall_double::double;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageData};

#[double]
use crate::canvas_factory::CanvasFactory;
use crate::{error::WebError, math_vector::Vec3};
pub struct ConvexHull {
    canvas: HtmlCanvasElement,
    rendering_context: CanvasRenderingContext2d,
}

//#[cfg_attr(test, mockall::automock)]
impl ConvexHull {
    pub fn new(canvas_width: u32, canvas_height: u32) -> Result<Self> {
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
        let all_points = self.get_all_points(&image_data);
        let hull = self.jarvis(&all_points);

        Ok(hull.iter().map(|v| (v.x as u32, v.y as u32)).collect())
    }

    fn jarvis<'a>(&self, all_points: &'a Vec<Vec3>) -> Vec<&'a Vec3> {
        let mut hull = vec![];
        let current = &all_points[0];

        loop {
            hull.push(current);

            let mut endpoint = &all_points[0];

            if current == endpoint {
                break;
            }

            for checking in all_points.iter() {
                if endpoint == current || is_left(checking, endpoint, current) {
                    endpoint = checking;
                }
            }
        }

        hull
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

    fn get_all_points(&self, image_data: &ImageData) -> Vec<Vec3> {
        let mut all_points = vec![];

        for (i, rgba) in image_data.data().deref().chunks_exact(4).enumerate() {
            if rgba[3] > 0 {
                let x = (i as u32 % image_data.width()) as f64;
                let y = (i as u32 / image_data.height()) as f64;

                all_points.push(Vec3::from((x, y)));
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

fn is_left(checking: &Vec3, endpoint: &Vec3, ref_point: &Vec3) -> bool {
    let a = checking - ref_point;
    let b = endpoint - ref_point;
    let cross = a.cross(&b);

    cross.z < 0.0
}

mod test {}
