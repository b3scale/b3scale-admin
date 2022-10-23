use yew::{function_component, html, Properties};

/// FrontendView Properties
#[derive(Clone, PartialEq, Properties)]
pub struct FrontendViewProps {
    pub id: String,
}

#[function_component(FrontendView)]
pub fn frontend_view(props: &FrontendViewProps) -> Html {
    let FrontendViewProps { id } = props;
    html! {
        <div class="frontend">
            {id}
        </div>
    }
}
