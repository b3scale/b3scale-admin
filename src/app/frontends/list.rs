use yew::{function_component, html, Html, Properties};

use crate::{
    api::frontends::{use_frontends, Frontend},
    app::{nav::Button, router::Route},
};

/// Frontend ListItem Properties
#[derive(Properties, Clone, PartialEq)]
pub struct ListItemProps {
    pub frontend: Frontend,
}

#[function_component(ListItem)]
pub fn list_item(ListItemProps { frontend }: &ListItemProps) -> Html {
    html! {
      <Button<Route> to={Route::Frontends{id: frontend.id.clone()}}>
        <div class="ms-2 me-auto">
          <div class="fw-bold">{&frontend.bbb.key}</div>
          <div class="subtitle">{&frontend.id}</div>
        </div>
      </Button<Route>>
    }
}

#[function_component(List)]
pub fn list() -> Html {
    let frontends = use_frontends();
    let frontends_list = match frontends.result() {
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
            <Button<Route> to={Route::FrontendsNew} class="btn btn-success mb-2">
                <i class="bi bi-plus-circle me-2"></i>
                {"Create New Frontend"}
            </Button<Route>>
            {frontends_list}
        </div>
    }
}
