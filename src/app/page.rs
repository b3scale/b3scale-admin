use yew::{function_component, html, Callback, Children, Properties, Html};
use yew_router::{hooks::use_navigator, prelude::*};

use crate::{
    api::auth::use_authentication,
    app::{nav::Link, router::Route},
};

#[derive(Properties, Clone, PartialEq)]
pub struct PageSelectProps {
    pub active: String,
}

#[function_component(PageSelect)]
pub fn page_select(PageSelectProps { active, .. }: &PageSelectProps) -> Html {
    let history = use_navigator().unwrap();
    
    let (page_title, create_route) = match active.as_str() {
        "frontends" => ("Frontends", Route::FrontendsNew),
        "backends" => ("Backends", Route::BackendsNew),
        _ => ("", Route::FrontendsNew), // fallback
    };
    
    let on_create = {
        let history = history.clone();
        let create_route = create_route.clone();
        Callback::from(move |_| {
            history.push(&create_route);
        })
    };
    
    html! {
        <div class="nav-header">
            <ul class="nav nav-pills nav-fill mb-3">
                <li class="nav-item">
                  <Link<Route> active={active == "frontends"} to={Route::FrontendsNew}>
                    {"Frontends"}
                  </Link<Route>>
                </li>
                <li class="nav-item">
                  <Link<Route> active={active == "backends"} to={Route::BackendsNew}>
                    {"Backends"}
                  </Link<Route>>
                </li>
            </ul>

            <div class="d-flex justify-content-between align-items-center mb-3">
                <button 
                    type="button" 
                    class="btn btn-primary btn-sm"
                    onclick={on_create}
                >
                    {format!("+ Add {}", page_title.trim_end_matches('s'))}
                </button>
            </div>
            
        </div>
    }
}

#[function_component(Header)]
fn header() -> Html {
    let auth = use_authentication();
    let history = use_navigator().unwrap();
    
    let on_logout = {
        let auth = auth.clone();
        let history = history.clone();
        Callback::from(move |_| {
            let mut auth = auth.clone();
            auth.logout();
            history.push(&Route::Authenticate);
        })
    };
    
    html! {
        <header>
          <nav class="navbar navbar-expand-md navbar-dark fixed-top bg-primary">
            <div class="container-fluid">
              <a class="navbar-brand" href="/">{"B3Scale Admin"}</a>
              <div class="d-flex">
                <button 
                    type="button" 
                    class="btn btn-outline-light btn-sm"
                    onclick={on_logout}
                >
                    {"Logout"}
                </button>
              </div>
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
