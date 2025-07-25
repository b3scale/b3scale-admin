use yew::{
    function_component, html, use_context, use_effect_with, hook, Children, ContextProvider, Html,
    Properties,
};

use crate::api::auth::use_api_url;
use crate::api::build_api_url;
use crate::api::client::{use_fetch, HttpRequest, Request, State};
pub use b3scale_api::{Backend, BackendPatch, BackendRequest};

/// List all backends
pub fn list() -> HttpRequest {
    HttpRequest::Builder(Request::get("/api/v1/backends"))
}

/// Get a single backend by ID
pub fn get(id: &str) -> HttpRequest {
    let url = format!("/api/v1/backends/{}", id);
    HttpRequest::Builder(Request::get(&url))
}

/// Create a new backend
pub fn create(backend: &BackendRequest) -> HttpRequest {
    let req = Request::post("/api/v1/backends")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(backend).expect("Failed to serialize to JSON"))
        .expect("Failed to build request");
    HttpRequest::Request(req)
}

/// Update an existing backend
pub fn update(id: &str, patch: &BackendPatch) -> HttpRequest {
    let req = Request::patch(&format!("/api/v1/backends/{}", id))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(patch).expect("Failed to serialize to JSON"))
        .expect("Failed to build request");
    HttpRequest::Request(req)
}

/// Delete a backend
pub fn delete(id: &str) -> HttpRequest {
    HttpRequest::Builder(Request::delete(&format!("/api/v1/backends/{}", id)))
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
    let state = use_fetch::<Vec<Backend>>(HttpRequest::Builder(Request::get(&url)));
    html! {
        <ContextProvider<State<Vec<Backend>>> context={state.clone()}>
          { for children.iter() }
        </ContextProvider<State<Vec<Backend>>>>
    }
}

#[hook]
pub fn use_backends() -> State<Vec<Backend>> {
    let ctx = use_context::<State<Vec<Backend>>>().expect("require backends context");
    // Trigger refresh
    {
        let ctx = ctx.clone();
        use_effect_with(
            (),
            move |_| {
                ctx.fetch();
                || ()
            },
        );
    };
    ctx
}

/// Hook to fetch a single backend
#[hook]
pub fn use_backend(id: &str) -> State<Backend> {
    let api_url = use_api_url();
    let url = build_api_url(&format!("/api/v1/backends/{}", id), api_url.as_deref());
    let state = use_fetch::<Backend>(HttpRequest::Builder(Request::get(&url)));
    // Trigger initial fetch immediately
    {
        let state = state.clone();
        use_effect_with(
            id.to_string(),
            move |_id| {
                state.fetch();
                || ()
            },
        );
    }
    state
}