use anyhow::Result;
pub use assets_loader::AssetsLoader;
pub use convex_hull::ConvexHull;
use game_state::{Game, MainGame};
#[mockall_double::double]
use js_adapter::animation_frame;
use log::Level;
use std::{cell::RefCell, panic, rc::Rc};
use wasm_bindgen::prelude::*;

mod assets_loader;
mod canvas_factory;
mod convex_hull;
mod error;
mod game_state;
mod js_adapter;
mod math_vector;
mod renderer;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsError> {
    console_log::init_with_level(Level::Info).expect("error initializing logger");
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let result = MainGame::new().await.map(|game| run(Box::new(game)));

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(JsError::new(&format!("{e}"))),
    }
}

pub fn run(game: Box<dyn Game>) -> Result<()> {
    let mut game = game;

    game.setup()?;

    let closure_ref = Rc::new(RefCell::new(None));
    let closure_ref_clone = closure_ref.clone();

    closure_ref_clone.replace({
        let closure: Closure<dyn FnMut()> = Closure::new(move || {
            game.draw().expect("draw");

            if game.is_running() {
                animation_frame::request_animation_frame(closure_ref.borrow().as_ref().unwrap());
            }
        });

        Some(closure)
    });

    animation_frame::request_animation_frame(closure_ref_clone.borrow().as_ref().unwrap());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        game_state::MockGame, js_adapter::mock_animation_frame::request_animation_frame_context,
    };

    use super::*;
    use js_sys::{Array, Function};
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn it_should_loop_until_the_game_ends() {
        let mut game = MockGame::new();
        let frames_ref = Rc::new(RefCell::new(vec![]));
        let frames_ref_copy = frames_ref.clone();
        let ctx = request_animation_frame_context();

        ctx.expect().times(5).returning_st(move |closure| {
            let func = closure.as_ref().unchecked_ref::<Function>();
            let func = func.bind(&JsValue::NULL);
            let mut frames = frames_ref_copy.borrow_mut();
            frames.push(func);
            frames.len()
        });

        let mut count = 0;

        game.expect_is_running().times(5).returning_st(move || {
            count += 1;
            count < 5
        });

        game.expect_setup().once().returning_st(|| Ok(()));
        game.expect_draw().times(5).returning_st(|| Ok(()));

        run(Box::new(game)).unwrap();

        let mut frames = frames_ref.borrow().clone();
        let mut frame_count = 0;

        while !frames.is_empty() {
            frames
                .remove(0)
                .apply(&JsValue::NULL, &Array::default())
                .expect("no errors");

            frame_count += 1;
            frames = (&frames_ref.borrow()[frame_count..]).to_vec();
        }

        assert_eq!(frame_count, 5);
    }
}
