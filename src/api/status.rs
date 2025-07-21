use super::client::{use_fetch, Request, State};
use b3scale_api::Status as StatusModel;

/// Read the status api
pub fn read() -> Request {
    Request::get("/api/v1/status")
}

pub fn use_status() -> State<StatusModel> {
    let state = use_fetch::<StatusModel>(read());
    state
}
