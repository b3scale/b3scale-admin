use yew::hook;
use super::auth::use_api_url;
use super::client::{use_fetch, HttpRequest, Request, State};
use super::build_api_url;
use b3scale_api::Status as StatusModel;

/// Read the status api
pub fn read() -> HttpRequest {
    HttpRequest::Builder(Request::get("/api/v1/status"))
}

/// Read the status api with a custom base URL
pub fn read_with_base_url(base_url: Option<&str>) -> HttpRequest {
    HttpRequest::Builder(Request::get(&build_api_url("/api/v1/status", base_url)))
}

#[hook]
pub fn use_status() -> State<StatusModel> {
    let api_url = use_api_url();
    let url = build_api_url("/api/v1/status", api_url.as_deref());
    let state = use_fetch::<StatusModel>(HttpRequest::Builder(Request::get(&url)));
    state
}
