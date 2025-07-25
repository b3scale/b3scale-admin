use b3scale_admin::{
    api::{auth::AuthenticationContext, backends::BackendsContext, frontends::FrontendsContext},
    app::Router,
};

use yew::{function_component, html, Html, Renderer};
use yew_router::BrowserRouter;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
        <AuthenticationContext>
        <FrontendsContext>
        <BackendsContext>
          <Router />
        </BackendsContext>
        </FrontendsContext>
        </AuthenticationContext>
        </BrowserRouter>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
