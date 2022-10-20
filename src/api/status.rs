use super::client::Request;

// Read the status api
pub fn read() -> Request {
    Request::get("/api/v1/status")
}
