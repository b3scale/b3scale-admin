use yew::{function_component, html, Children, Properties};

#[function_component(Header)]
fn header() -> Html {
    html! {
        <header>
          <nav class="navbar navbar-expand-md navbar-dark fixed-top bg-primary">
            <div class="container-fluid">
              <a class="navbar-brand" href="/">{"B3Scale Admin"}</a>
            </div>
            <div class="d-flex">
            </div>
          </nav>
        </header>
    }
}

#[derive(PartialEq, Clone, Properties)]
pub struct PageProps {
    pub children: Children,
}

#[function_component(Page)]
pub fn page(props: &PageProps) -> Html {
    let PageProps { children } = props;
    html! {
        <main>
            <Header />
            { for children.iter() }
        </main>
    }
}
