//! Browser domain: browser-scoped operations (version, close, window info).
//!
//! Schema reference: `browser_protocol.json` → `Browser` domain (stable
//! except where flagged).

use serde::{Deserialize, Serialize};

use crate::command::Command;
use crate::domains::target::TargetId;

pub type WindowId = i64;

/// `Browser.Bounds` — window position + size + state.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bounds {
    #[serde(default)]
    pub left: Option<i32>,
    #[serde(default)]
    pub top: Option<i32>,
    #[serde(default)]
    pub width: Option<i32>,
    #[serde(default)]
    pub height: Option<i32>,
    /// `"normal"`, `"minimized"`, `"maximized"`, `"fullscreen"`.
    #[serde(default)]
    pub window_state: Option<String>,
}

// -- Browser.getVersion ------------------------------------------------------

#[derive(Debug, Serialize, Default)]
pub struct GetVersion;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVersionResponse {
    pub protocol_version: String,
    pub product: String,
    pub revision: String,
    pub user_agent: String,
    pub js_version: String,
}

impl Command for GetVersion {
    const METHOD: &'static str = "Browser.getVersion";
    type Return = GetVersionResponse;
}

// -- Browser.close -----------------------------------------------------------

#[derive(Debug, Serialize, Default)]
pub struct Close;

#[derive(Debug, Deserialize)]
pub struct CloseResponse {}

impl Command for Close {
    const METHOD: &'static str = "Browser.close";
    type Return = CloseResponse;
}

// -- Browser.getWindowForTarget (experimental) -------------------------------

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWindowForTarget {
    /// Defaults to the current target when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<TargetId>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWindowForTargetResponse {
    pub window_id: WindowId,
    pub bounds: Bounds,
}

impl Command for GetWindowForTarget {
    const METHOD: &'static str = "Browser.getWindowForTarget";
    type Return = GetWindowForTargetResponse;
}
