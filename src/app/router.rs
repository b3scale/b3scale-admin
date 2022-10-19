use gloo::console::log;
use yew::{function_component, html, use_effect, Callback, Html};
use yew_router::{
    history::History,
    hooks::{use_history, use_route},
    Routable, Switch,
};

use crate::app::authenticate::Authenticate;
use crate::context::{use_access_token, use_authentication};

#[derive(Clone, Routable, PartialEq, Debug)]
enum Route {
    #[at("/")]
    Start,
    #[at("/authenticate")]
    Authenticate,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Start => html! { <Start /> },
        Route::Authenticate => html! { <Authenticate /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component(Router)]
pub fn router() -> Html {
    let history = use_history().unwrap();
    let auth = use_authentication();
    let route = use_route().unwrap_or(Route::Start);
    log!(format!("route: {:?}", route));
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
    let token = use_access_token();
    let authenticated = match token {
        Some(t) => html! { <b>{"Auth: "}{t}</b> },
        None => html! { <b>{"Unauthenticated"}</b> },
    };
    log!("render start");

    let on_login = {
        let api = use_authentication();
        Callback::from(move |_| {
            let api = &mut api.clone();
            api.authenticate("foooblubb").expect("yay");
        })
    };

    let on_logout = {
        let api = use_authentication();
        Callback::from(move |_| {
            let api = &mut api.clone();
            api.logout();
            log!("looogout");
        })
    };

    html! {
        <p>{"Start"}<a href="/authenticate">{"Authenticate"}</a>
            <br />
            {authenticated}
            <button onclick={on_login}>{"Login"}</button>
            <button onclick={on_logout}>{"Logout"}</button>
        </p>
    }
}
