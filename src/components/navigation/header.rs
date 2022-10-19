use yew::{function_component, html};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
          <nav class="navbar navbar-expand-md navbar-dark fixed-top bg-primary">
            <div class="container-fluid">
              <a class="navbar-brand" href="#">{"🐱"}</a>
            </div>
            <div class="d-flex">
            </div>
          </nav>
        </header>
    }
}
