//! Runtime domain: `Runtime.evaluate` for script execution.
//!
//! The `Runtime.evaluate` command has many optional parameters; only the
//! commonly-used ones are surfaced here. Callers needing a niche field can
//! fall back to [`crate::Client::send`] with a raw JSON value.
//!
//! Schema reference: `js_protocol.json` → `Runtime` domain (stable).

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::command::Command;

pub type ExecutionContextId = i64;

/// `Runtime.RemoteObject` — the return-value envelope for `evaluate`. Kept
/// as `serde_json::Value` for now; consumers usually only care about
/// `.value` (when `returnByValue: true` was set) or `.description`.
pub type RemoteObject = Value;

/// `Runtime.ExceptionDetails` — populated when `evaluate` threw. Kept as
/// `serde_json::Value` for the same reason.
pub type ExceptionDetails = Value;

// -- Runtime.evaluate --------------------------------------------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Evaluate {
    pub expression: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_command_line_api: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<ExecutionContextId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_by_value: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_gesture: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub await_promise: Option<bool>,
    /// Timeout in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
}

impl Evaluate {
    /// Common preset: evaluate an expression and receive the result inlined
    /// as `Value` rather than as a remote object handle.
    pub fn returning_value(expression: impl Into<String>) -> Self {
        Self {
            expression: expression.into(),
            object_group: None,
            include_command_line_api: None,
            silent: None,
            context_id: None,
            return_by_value: Some(true),
            user_gesture: None,
            await_promise: None,
            timeout: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateResponse {
    pub result: RemoteObject,
    #[serde(default)]
    pub exception_details: Option<ExceptionDetails>,
}

impl Command for Evaluate {
    const METHOD: &'static str = "Runtime.evaluate";
    type Return = EvaluateResponse;
}
