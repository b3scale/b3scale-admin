use yew::{function_component, html, Properties};

use crate::app::{frontends::list::List, nav::Link, router::Route, Page};

#[derive(Properties, Clone, PartialEq)]
pub struct FrontendsPageProps {
    pub id: Option<String>,
}

#[function_component(FrontendsPage)]
pub fn frontends_page(props: &FrontendsPageProps) -> Html {
    html! {
        <Page>
          <nav>
            <div class="nav-header">
              <ul class="nav nav-pills nav-fill">
                <li class="nav-item">
                  <Link<Route> depth={1} to={Route::Frontends { id: "new".to_string()}}>
                    {"Frontends"}
                  </Link<Route>>
                </li>
                <li class="nav-item">
                  <Link<Route> depth={2} to={Route::Backends}>
                    {"Backends"}
                  </Link<Route>>
                </li>
              </ul>
              <ul class="nav nav-pills nav-fill">
                <li class="nav-item">
                <Link<Route> to={Route::Frontends { id: "new".to_string() }}>
                  { "(+)" }
                  </Link<Route>>
                </li>
              </ul>
            </div>

            <List />

          </nav>
          <main>
          </main>
        </Page>
    }
}
