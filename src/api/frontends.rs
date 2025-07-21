use yew::{
    function_component, html, use_context, use_effect_with_deps, Children, ContextProvider,
    Properties,
};

use crate::api::client::{use_fetch, Request, State};
pub use b3scale_api::Frontend;

/// List frontends Request
fn list() -> Request {
    Request::get("/api/v1/frontends")
}

/// Context
#[derive(PartialEq, Clone, Properties)]
pub struct FrontendsContextProps {
    pub children: Children,
}

#[function_component(FrontendsContext)]
pub fn frontends_context(props: &FrontendsContextProps) -> Html {
    let FrontendsContextProps { children } = props;
    let state = use_fetch::<Vec<Frontend>>(list());
    html! {
        <ContextProvider<State<Vec<Frontend>>> context={state.clone()}>
          { for children.iter() }
        </ContextProvider<State<Vec<Frontend>>>>
    }
}

pub fn use_frontends() -> State<Vec<Frontend>> {
    let ctx = use_context::<State<Vec<Frontend>>>().expect("require frontends context");
    // Trigger refresh
    {
        let ctx = ctx.clone();
        use_effect_with_deps(
            move |_| {
                ctx.fetch();
                || ()
            },
            (),
        );
    };
    ctx
}
