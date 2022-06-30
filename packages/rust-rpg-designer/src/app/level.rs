use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, MouseEvent};
use yew::{html, Component, Html, Properties};

pub struct Tile {}

impl Tile {
  pub fn new() -> Self {
    Self {}
  }
}

pub struct LevelComponent {
  tiles: Vec<Tile>,
}

#[derive(Properties, PartialEq)]
pub struct LevelComponentProps {
  pub cols: u16,
  pub rows: u16,
}

impl Component for LevelComponent {
  type Message = ();
  type Properties = LevelComponentProps;

  fn create(ctx: &yew::Context<Self>) -> Self {
    let LevelComponentProps { cols, rows } = ctx.props();
    log::info!("create cols={cols} rows={rows}");

    Self {
      tiles: (0..cols * rows).map(|_| Tile::new()).collect(),
    }
  }

  fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
    let LevelComponentProps { cols, rows } = ctx.props();
    log::info!("changed cols={cols} rows={rows}");
    true
  }

  fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
    let cols = ctx.props().cols;

    html! {
      <div class="level" style={format!("--level-cols: {cols};")}>
      {
        self.tiles.iter().map(|tile| html! {
          <div class="tile"></div>
        }).collect::<Vec<Html>>()
      }
      </div>
    }
  }
}
