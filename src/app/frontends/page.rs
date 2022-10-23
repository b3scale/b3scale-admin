use yew::{function_component, html, Properties};

use crate::app::{
    frontends::list::List,
    nav::Link,
    router::{FrontendsRoute, Route},
    Page,
};

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
                  <Link<Route> to={Route::Frontends}>
                    {"Frontends"}
                  </Link<Route>>
                </li>
                <li class="nav-item">
                  <Link<Route> to={Route::Backends}>
                    {"Backends"}
                  </Link<Route>>
                </li>
              </ul>
              <ul class="nav nav-pills nav-fill">
                <li class="nav-item">
                <Link<FrontendsRoute> to={FrontendsRoute::Show { id: "new".to_string() }}>
                  { "(+)" }
                  </Link<FrontendsRoute>>
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
