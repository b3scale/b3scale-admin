use yew::{
    function_component, html, use_context, use_effect_with_deps, Children, ContextProvider,
    Properties,
};

use crate::api::auth::use_api_url;
use crate::api::build_api_url;
use crate::api::client::{use_fetch, Request, State};
pub use b3scale_api::{Frontend, FrontendPatch, FrontendRequest};

/// List all frontends
pub fn list() -> Request {
    Request::get("/api/v1/frontends")
}

/// Get a single frontend by ID
pub fn get(id: &str) -> Request {
    let url = format!("/api/v1/frontends/{}", id);
    Request::get(&url)
}

/// Create a new frontend
pub fn create(frontend: &FrontendRequest) -> Request {
    Request::post("/api/v1/frontends")
        .json(frontend)
        .expect("Failed to serialize frontend request")
}

/// Update an existing frontend
pub fn update(id: &str, patch: &FrontendPatch) -> Request {
    Request::patch(&format!("/api/v1/frontends/{}", id))
        .json(patch)
        .expect("Failed to serialize frontend patch")
}

/// Delete a frontend
pub fn delete(id: &str) -> Request {
    Request::delete(&format!("/api/v1/frontends/{}", id))
}

/// Context
#[derive(PartialEq, Clone, Properties)]
pub struct FrontendsContextProps {
    pub children: Children,
}

#[function_component(FrontendsContext)]
pub fn frontends_context(props: &FrontendsContextProps) -> Html {
    let FrontendsContextProps { children } = props;
    let api_url = use_api_url();
    let url = build_api_url("/api/v1/frontends", api_url.as_deref());
    let state = use_fetch::<Vec<Frontend>>(Request::get(&url));
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

/// Hook to fetch a single frontend
pub fn use_frontend(id: &str) -> State<Frontend> {
    let api_url = use_api_url();
    let url = build_api_url(&format!("/api/v1/frontends/{}", id), api_url.as_deref());
    let state = use_fetch::<Frontend>(Request::get(&url));
    // Trigger initial fetch immediately
    {
        let state = state.clone();
        use_effect_with_deps(
            move |_id| {
                state.fetch();
                || ()
            },
            id.to_string(),
        );
    }
    state
}
