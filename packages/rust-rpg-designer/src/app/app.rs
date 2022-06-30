use super::{LevelComponent, LevelCreationComponent};
use yew::{html, Component};

pub struct AppComponent {
  level: Option<(u16, u16)>,
}

pub enum Msg {
  CreateLevel(u16, u16),
}

impl AppComponent {
  fn create_level(&mut self, cols: u16, rows: u16) {
    self.level = Some((cols, rows));
  }
}

impl Component for AppComponent {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &yew::Context<Self>) -> Self {
    Self { level: None }
  }

  fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::CreateLevel(cols, rows) => {
        self.create_level(cols, rows);
        true
      }
    }
  }

  fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
    html! {
      <div class="designer">
      {
        match self.level {
          Some((cols, rows)) => html! {<LevelComponent {cols} {rows}/>},
          None => {
            let on_create = ctx
              .link()
              .callback(|(cols, rows)| Msg::CreateLevel(cols, rows));

            html! { <LevelCreationComponent {on_create}/>}
          }
        }
      }
      </div>
    }
  }
}
