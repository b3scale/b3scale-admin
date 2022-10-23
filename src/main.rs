use b3scale_admin::{
    api::{auth::AuthenticationContext, frontends::FrontendsContext},
    app::Router,
};

use yew::{function_component, html};
use yew_router::BrowserRouter;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
        <AuthenticationContext>
        <FrontendsContext>
          <Router />
        </FrontendsContext>
        </AuthenticationContext>
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
