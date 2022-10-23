use yew::{function_component, html, Properties};

use crate::app::{Page, PageSelect};

#[derive(Properties, PartialEq, Clone)]
pub struct BackendsPageProps {
    pub id: String,
}

#[function_component(BackendsPage)]
pub fn backends_page(props: &BackendsPageProps) -> Html {
    html! {
        <Page>
            <aside>
                <PageSelect active="backends" />
            </aside>
            <main>
            </main>
        </Page>
    }
}
