use anyhow::Result;

#[cfg(test)]
use mockall::automock;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::renderer::Renderer;

pub struct MainGame {
    running: bool,
    canvas: HtmlCanvasElement,
    renderer: Renderer<CanvasRenderingContext2d>,
}

#[cfg_attr(test, automock)]
pub trait Game {
    fn is_running(&self) -> bool;

    fn setup(&mut self) -> Result<()>;

    fn draw(&mut self) -> Result<()>;
}

impl Game for MainGame {
    fn is_running(&self) -> bool {
        self.running
    }

    fn setup(&mut self) -> Result<()> {
        self.running = true;
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        Ok(())
    }
}

impl MainGame {
    pub async fn new() -> Result<Self> {
        todo!()
    }
}

impl Drop for MainGame {
    fn drop(&mut self) {}
}
