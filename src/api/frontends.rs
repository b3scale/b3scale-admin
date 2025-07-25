use yew::{
    function_component, html, use_context, use_effect_with, hook, Children, ContextProvider, Html,
    Properties,
};

use crate::api::auth::use_api_url;
use crate::api::build_api_url;
use crate::api::client::{use_fetch, HttpRequest, Request, State};
pub use b3scale_api::{Frontend, FrontendPatch, FrontendRequest};

/// List all frontends
pub fn list() -> HttpRequest {
    HttpRequest::Builder(Request::get("/api/v1/frontends"))
}

/// Get a single frontend by ID
pub fn get(id: &str) -> HttpRequest {
    let url = format!("/api/v1/frontends/{}", id);
    HttpRequest::Builder(Request::get(&url))
}

/// Create a new frontend
pub fn create(frontend: &FrontendRequest) -> HttpRequest {
    let req = Request::post("/api/v1/frontends")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(frontend).expect("Failed to serialize to JSON"))
        .expect("Failed to build request");
    HttpRequest::Request(req)
}

/// Update an existing frontend
pub fn update(id: &str, patch: &FrontendPatch) -> HttpRequest {
    let req = Request::patch(&format!("/api/v1/frontends/{}", id))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(patch).expect("Failed to serialize to JSON"))
        .expect("Failed to build request");
    HttpRequest::Request(req)
}

/// Delete a frontend
pub fn delete(id: &str) -> HttpRequest {
    HttpRequest::Builder(Request::delete(&format!("/api/v1/frontends/{}", id)))
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
    let state = use_fetch::<Vec<Frontend>>(HttpRequest::Builder(Request::get(&url)));
    html! {
        <ContextProvider<State<Vec<Frontend>>> context={state.clone()}>
          { for children.iter() }
        </ContextProvider<State<Vec<Frontend>>>>
    }
}

#[hook]
pub fn use_frontends() -> State<Vec<Frontend>> {
    let ctx = use_context::<State<Vec<Frontend>>>().expect("require frontends context");
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

/// Hook to fetch a single frontend
#[hook]
pub fn use_frontend(id: &str) -> State<Frontend> {
    let api_url = use_api_url();
    let url = build_api_url(&format!("/api/v1/frontends/{}", id), api_url.as_deref());
    let state = use_fetch::<Frontend>(HttpRequest::Builder(Request::get(&url)));
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
