use yew::{function_component, html, Properties};

use crate::app::{frontends::list::List, Link, Page, PageSelect, Route};

#[derive(Properties, Clone, PartialEq)]
pub struct FrontendsPageProps {
    pub id: Option<String>,
}

#[function_component(FrontendsPage)]
pub fn frontends_page(props: &FrontendsPageProps) -> Html {
    html! {
        <Page>
          <aside>
            <PageSelect active="frontends" />
            <List />

          </aside>
          <main>
          </main>
        </Page>
    }
}
