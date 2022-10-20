use yew::{function_component, html};

use super::Page;

#[function_component(Frontends)]
pub fn frontends() -> Html {
    html! {
        <Page>
         <p>{" stuff...frontends...."}</p>
        </Page>
    }
}
