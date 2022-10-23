use yew::{function_component, html, Html, Properties};

use crate::{
    api::frontends::{use_frontends, Frontend},
    app::{
        nav::Button,
        router::{FrontendsRoute, Route},
    },
};

/// Frontend ListItem Properties
#[derive(Properties, Clone, PartialEq)]
pub struct ListItemProps {
    pub frontend: Frontend,
}

#[function_component(ListItem)]
pub fn list_item(ListItemProps { frontend }: &ListItemProps) -> Html {
    html! {
      <Button<FrontendsRoute> to={FrontendsRoute::Show{id: frontend.id.clone()}}>
        <div class="ms-2 me-auto">
          <div class="fw-bold">{&frontend.bbb.key}</div>
          <div class="subtitle">{&frontend.id}</div>
        </div>
      </Button<FrontendsRoute>>
    }
}

#[function_component(List)]
pub fn list() -> Html {
    let frontends = use_frontends();
    let frontends = match frontends.result() {
        None => html! { <p>{"No Frontends"}</p> },
        Some(frontends) => frontends
            .iter()
            .map(|f| {
                html! { <ListItem frontend={f.clone()} /> }
            })
            .collect::<Html>(),
    };
    html! {
    <div class="nav-select-list list-group">
      {frontends}
    </div>
    }
}
