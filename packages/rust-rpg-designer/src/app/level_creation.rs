use wasm_bindgen::JsCast;
use web_sys::{FocusEvent, FormData, HtmlFormElement};
use yew::{html, Callback, Component, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct LevelCreationProps {
  pub on_create: Callback<(u16, u16)>,
}

pub struct LevelCreationComponent {}

pub enum Msg {
  Apply(FocusEvent),
}

impl LevelCreationComponent {
  fn get_u16(data: &FormData, field: &str) -> u16 {
    data
      .get(field)
      .as_string()
      .expect("field")
      .parse()
      .expect("number")
  }

  fn apply(&self, ctx: &yew::Context<Self>, form: &HtmlFormElement) {
    let data = FormData::new_with_form(form).expect("form data expected");
    let cols = LevelCreationComponent::get_u16(&data, "cols");
    let rows = LevelCreationComponent::get_u16(&data, "rows");

    ctx.props().on_create.emit((cols, rows));
  }
}

impl Component for LevelCreationComponent {
  type Message = Msg;
  type Properties = LevelCreationProps;

  fn create(ctx: &yew::Context<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::Apply(e) => {
        e.prevent_default();
        self.apply(
          ctx,
          &e.target()
            .expect("target not null")
            .dyn_into::<HtmlFormElement>()
            .expect("form expected"),
        );
        false
      }
    }
  }

  fn view(&self, ctx: &yew::Context<Self>) -> Html {
    let onsubmit = ctx.link().callback(|e| Msg::Apply(e));

    html! {
      <div class="level-creation">
        <form {onsubmit}>
          <fluent-number-field name="cols" value="10">{"Columns:"}</fluent-number-field>
          <fluent-number-field name="rows" value="10">{"Rows:"}</fluent-number-field>
          <div>
            <fluent-button type="submit" appearance="accent">{"Create"}</fluent-button>
          </div>
        </form>
      </div>
    }
  }
}
