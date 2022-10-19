use b3scale_admin::{app::Router, context::AuthenticationContext};

use yew::{function_component, html};
use yew_router::BrowserRouter;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
        <AuthenticationContext>
            <Router />
        </AuthenticationContext>
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
