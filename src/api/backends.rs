use yew::{
    function_component, html, use_context, use_effect_with_deps, Children, ContextProvider,
    Properties,
};

use crate::api::auth::use_api_url;
use crate::api::build_api_url;
use crate::api::client::{use_fetch, Request, State};
pub use b3scale_api::{Backend, BackendPatch, BackendRequest};

/// List all backends
pub fn list() -> Request {
    Request::get("/api/v1/backends")
}

/// Get a single backend by ID
pub fn get(id: &str) -> Request {
    let url = format!("/api/v1/backends/{}", id);
    Request::get(&url)
}

/// Create a new backend
pub fn create(backend: &BackendRequest) -> Request {
    Request::post("/api/v1/backends")
        .json(backend)
        .expect("Failed to serialize backend request")
}

/// Update an existing backend
pub fn update(id: &str, patch: &BackendPatch) -> Request {
    Request::patch(&format!("/api/v1/backends/{}", id))
        .json(patch)
        .expect("Failed to serialize backend patch")
}

/// Delete a backend
pub fn delete(id: &str) -> Request {
    Request::delete(&format!("/api/v1/backends/{}", id))
}

/// Context
#[derive(PartialEq, Clone, Properties)]
pub struct BackendsContextProps {
    pub children: Children,
}

#[function_component(BackendsContext)]
pub fn backends_context(props: &BackendsContextProps) -> Html {
    let BackendsContextProps { children } = props;
    let api_url = use_api_url();
    let url = build_api_url("/api/v1/backends", api_url.as_deref());
    let state = use_fetch::<Vec<Backend>>(Request::get(&url));
    html! {
        <ContextProvider<State<Vec<Backend>>> context={state.clone()}>
          { for children.iter() }
        </ContextProvider<State<Vec<Backend>>>>
    }
}

pub fn use_backends() -> State<Vec<Backend>> {
    let ctx = use_context::<State<Vec<Backend>>>().expect("require backends context");
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

/// Hook to fetch a single backend
pub fn use_backend(id: &str) -> State<Backend> {
    let api_url = use_api_url();
    let url = build_api_url(&format!("/api/v1/backends/{}", id), api_url.as_deref());
    let state = use_fetch::<Backend>(Request::get(&url));
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