//! HTTP discovery endpoints exposed by a running Chromium browser with CDP
//! enabled: `/json`, `/json/version`, `/json/list`.
//!
//! These endpoints require CDP to be already open on the target port. They
//! do not enable CDP themselves.

use serde::Deserialize;
use std::time::Duration;

use crate::{CdpError, Result};

/// A single debugging target (page, worker, service worker, or the browser
/// itself). Matches Chromium's `/json` output shape.
#[derive(Debug, Clone, Deserialize)]
pub struct TargetInfo {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default, rename = "type")]
    pub target_type: Option<String>,
    #[serde(default, rename = "webSocketDebuggerUrl")]
    pub ws_url: Option<String>,
    #[serde(default, rename = "devtoolsFrontendUrl")]
    pub frontend_url: Option<String>,
}

/// GET `/json` — list all discoverable targets.
pub fn list_targets(port: u16) -> Result<Vec<TargetInfo>> {
    let body = http_get(&format!("http://127.0.0.1:{port}/json"))?;
    serde_json::from_str(&body).map_err(|e| CdpError::Parse(e.to_string()))
}

/// GET `/json/version` — extract the browser's own WebSocket URL. This is the
/// WS endpoint to connect to when you want to drive the browser session (as
/// opposed to a specific tab); it accepts `Target.*` methods and can attach
/// to per-target sessions via the flat-session model.
pub fn browser_ws_url(port: u16) -> Result<String> {
    let body = http_get(&format!("http://127.0.0.1:{port}/json/version"))?;
    let parsed: serde_json::Value =
        serde_json::from_str(&body).map_err(|e| CdpError::Parse(e.to_string()))?;
    parsed
        .get("webSocketDebuggerUrl")
        .and_then(|v| v.as_str())
        .map(String::from)
        .ok_or_else(|| CdpError::Parse("no webSocketDebuggerUrl in /json/version".into()))
}

/// Cheap CDP presence probe. Returns true iff `/json/version` responded with a
/// parseable body containing `webSocketDebuggerUrl`.
pub fn probe(port: u16) -> bool {
    browser_ws_url(port).is_ok()
}

fn http_get(url: &str) -> Result<String> {
    let agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_millis(500))
        .timeout_read(Duration::from_secs(2))
        .build();
    agent
        .get(url)
        .call()
        .map_err(|e| CdpError::Discovery(e.to_string()))?
        .into_string()
        .map_err(|e| CdpError::Discovery(e.to_string()))
}
