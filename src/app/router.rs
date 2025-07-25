use yew::{function_component, html, use_effect, Html};
use yew_router::{
    history::History,
    hooks::{use_navigator, use_route},
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
    #[at("/frontends/new")]
    FrontendsNew,
    #[at("/frontends/:id")]
    Frontends { id: String },
    #[at("/backends/new")]
    BackendsNew,
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
        Route::FrontendsNew => html! {
            <FrontendsPage id={None::<String>} />
        },
        Route::Frontends { id } => html! {
            <FrontendsPage id={Some(id.clone())} />
        },
        Route::BackendsNew => html! {
            <BackendsPage id={None::<String>} />
        },
        Route::Backends { id } => html! {
            <BackendsPage id={Some(id.clone())} />
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
    let history = use_navigator().unwrap();
    let auth = use_authentication();
    let route = use_route().unwrap_or(Route::Start);
    {
        // Navigate to auth page
        use_effect(move || {
            if route != Route::Authenticate && !auth.is_authenticated() {
                history.replace(&Route::Authenticate);
            };
            || ()
        });
    }
    html! {
        <Switch<Route> render={|route: Route| switch(&route)} />
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
    let history = use_navigator().unwrap();

    use_effect(move || {
        if auth.is_authenticated() {
            history.replace(&Route::FrontendsNew);
        };
        || ()
    });

    html! {
        <></>
    }
}
