use yew::{function_component, html, use_effect, Html};
use yew_router::{
    history::History,
    hooks::{use_history, use_route},
    Routable, Switch,
};

use crate::api::auth::use_authentication;
use crate::app::{AuthenticatePage, BackendsPage, FrontendsPage};

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Start,
    #[at("/authenticate")]
    Authenticate,
    #[at("/frontends/:id")]
    Frontends { id: String },
    #[at("/backends/:id")]
    Backends { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// Render page
fn switch(route: &Route) -> Html {
    match route {
        Route::Start => html! { <Start /> },
        Route::Frontends { id } => html! {
            <FrontendsPage id={id.clone()} />
        },
        Route::Backends { id } => html! {
            <BackendsPage id={id.clone()} />
        },
        Route::Authenticate => html! {
            <AuthenticatePage />
        },
        Route::NotFound => html! {
            <NotFound />
        },
    }
}

#[function_component(Router)]
pub fn router() -> Html {
    let history = use_history().unwrap();
    let auth = use_authentication();
    let route = use_route().unwrap_or(Route::Start);
    {
        // Navigate to auth page
        use_effect(move || {
            if route != Route::Authenticate && !auth.is_authenticated() {
                history.replace(Route::Authenticate);
            };
            || ()
        });
    }
    html! {
        <Switch<Route> render={Switch::render(switch)} />
    }
}

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <p>{"404 not found"}</p>
    }
}

#[function_component(Start)]
pub fn start() -> Html {
    let auth = use_authentication();
    let history = use_history().unwrap();

    use_effect(move || {
        if auth.is_authenticated() {
            history.replace(Route::Frontends {
                id: "new".to_owned(),
            });
        };
        || ()
    });

    html! {
        <></>
    }
}
