//! CDP JSON envelope types.
//!
//! Kept minimal — we send request bodies as `serde_json::Value` and dispatch
//! responses back to the requester by `id`. Events are surfaced as-is.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A CDP response or event, discriminated by the presence of `id` (response) or
/// `method` (event). `sessionId` is the only camelCase field in the wire format
/// at this level, so it's the only one that needs an explicit rename.
#[derive(Debug, Deserialize)]
pub struct Frame {
    pub id: Option<u64>,
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
    pub method: Option<String>,
    pub result: Option<Value>,
    pub params: Option<Value>,
    pub error: Option<CommandError>,
}

impl Frame {
    /// Deserialize a raw text frame.
    pub fn parse(text: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(text)
    }
}

/// A successful response to a request.
#[derive(Debug, Clone)]
pub struct Response {
    pub id: u64,
    pub session_id: Option<String>,
    pub result: Value,
}

/// An event dispatched by the browser.
#[derive(Debug, Clone)]
pub struct Event {
    pub method: String,
    pub session_id: Option<String>,
    pub params: Value,
}

/// The `error` object present when a request fails.
#[derive(Debug, Clone, Deserialize)]
pub struct CommandError {
    pub code: i64,
    pub message: String,
}

/// A request envelope. Consumers rarely build this directly — [`crate::Client::send`]
/// wraps it — but it's exposed for callers that want to serialize manually.
#[derive(Debug, Serialize)]
pub struct Request<'a> {
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sessionId")]
    pub session_id: Option<&'a str>,
    pub method: &'a str,
    pub params: Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_response_frame() {
        let text = r#"{"id":7,"sessionId":"abc","result":{"cookies":[]}}"#;
        let f = Frame::parse(text).unwrap();
        assert_eq!(f.id, Some(7));
        assert_eq!(f.session_id.as_deref(), Some("abc"));
        assert!(f.result.is_some());
        assert!(f.method.is_none());
    }

    #[test]
    fn parse_event_frame() {
        let text = r#"{"method":"Target.attachedToTarget","params":{"sessionId":"xyz","targetInfo":{}}}"#;
        let f = Frame::parse(text).unwrap();
        assert!(f.id.is_none());
        assert_eq!(f.method.as_deref(), Some("Target.attachedToTarget"));
        assert!(f.params.is_some());
    }

    #[test]
    fn parse_error_frame() {
        let text = r#"{"id":3,"error":{"code":-32601,"message":"'Frobnicate.wobble' wasn't found"}}"#;
        let f = Frame::parse(text).unwrap();
        assert_eq!(f.id, Some(3));
        let e = f.error.unwrap();
        assert_eq!(e.code, -32601);
        assert!(e.message.contains("wasn't found"));
    }
}
