use gloo_console::log;
use yew::{function_component, html, Html, Properties};

use super::Page;
use crate::api::frontends::{use_frontends, Frontend};

/// Frontend ListItem Properties
#[derive(Properties, Clone, PartialEq)]
pub struct ListItemProps {
    pub frontend: Frontend,
}

#[function_component(ListItem)]
pub fn list_item(ListItemProps { frontend }: &ListItemProps) -> Html {
    html! {
      <button
        class="list-group-item list-group-item-action">
        <div class="ms-2 me-auto">
          <div class="fw-bold">{&frontend.bbb.key}</div>
          <div class="subtitle">{&frontend.id}</div>
        </div>
      </button>
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
        {frontends.clone()}
        {frontends.clone()}
        {frontends.clone()}
        {frontends.clone()}
        {frontends.clone()}
        {frontends.clone()}
        {frontends.clone()}
        {frontends.clone()}
        {frontends.clone()}
    </div>
    }
}

#[function_component(Frontends)]
pub fn frontends() -> Html {
    html! {
        <Page>
          <nav>
            <div class="nav-header">
              <ul class="nav nav-pills nav-fill">
                <li class="nav-item">
                  <a class="nav-link active"
                     href="/frontends">{"Frontends"}</a></li>
                <li class="nav-item">
                  <a class="nav-link"
                     href="/backends">{"Backends"}</a></li>
              </ul>
              <ul class="nav nav-pills nav-fill">
                <li class="nav-item">
                  <a class="nav-link" href="/frontends/new">
                  { "(+)" }
                  </a>
                </li>
              </ul>
            </div>

            <List />

          </nav>
          <main>
          </main>
        </Page>
    }
}
