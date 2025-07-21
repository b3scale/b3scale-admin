use yew::{function_component, html, Children, Properties};

use crate::{
    app::{nav::Link, router::Route},
};

#[derive(Properties, Clone, PartialEq)]
pub struct PageSelectProps {
    pub active: String,
}

#[function_component(PageSelect)]
pub fn page_select(PageSelectProps { active, .. }: &PageSelectProps) -> Html {
    html! {
        <div class="nav-header">
          <ul class="nav nav-pills nav-fill">
            <li class="nav-item">
              <Link<Route> active={active == "frontends"} to={Route::Frontends { id: "new".to_string()}}>
                {"Frontends"}
              </Link<Route>>
            </li>
            <li class="nav-item">
              <Link<Route> active={active == "backends"} to={Route::Backends { id: "new".into() }}>
                {"Backends"}
              </Link<Route>>
            </li>
          </ul>
          <ul class="nav nav-pills nav-fill">
            <li class="nav-item">
            <Link<Route> active={false} to={Route::Frontends { id: "new".to_string() }}>
              { "(+)" }
              </Link<Route>>
            </li>
          </ul>
        </div>
    }
}

#[function_component(Header)]
fn header() -> Html {
    html! {
        <header>
          <nav class="navbar navbar-expand-md navbar-dark fixed-top bg-primary">
            <div class="container-fluid">
              <a class="navbar-brand" href="/">{"B3Scale Admin"}</a>
            </div>
            <div class="d-flex">
            </div>
          </nav>
        </header>
    }
}

#[derive(PartialEq, Clone, Properties)]
pub struct PageProps {
    pub children: Children,
}

#[function_component(Page)]
pub fn page(props: &PageProps) -> Html {
    let PageProps { children } = props;

    html! {
      <div class="container-page">
        <Header />
        <div class="page">
            { for children.iter() }
        </div>
      </div>
    }
}
