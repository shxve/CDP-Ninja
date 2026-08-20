//! Storage domain: browser-context-scoped cookie access.
//!
//! `Storage.getCookies` reads from the live in-memory cookie store and is
//! usually the right choice over `Network.getAllCookies`:
//! * It does not require a target attach and does not need Network events
//!   enabled on the current session.
//! * It respects the browser-context boundary when `browserContextId` is
//!   supplied (default context otherwise).
//!
//! Schema reference: `browser_protocol.json` → `Storage` domain (stable).

use serde::{Deserialize, Serialize};

use crate::command::Command;
use crate::domains::network::Cookie;
use crate::domains::target::BrowserContextId;

// -- Storage.getCookies ------------------------------------------------------

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCookies {
    /// Restrict to a specific browser context; `None` = default context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_context_id: Option<BrowserContextId>,
}

#[derive(Debug, Deserialize)]
pub struct GetCookiesResponse {
    pub cookies: Vec<Cookie>,
}

impl Command for GetCookies {
    const METHOD: &'static str = "Storage.getCookies";
    type Return = GetCookiesResponse;
}

// -- Storage.clearCookies ----------------------------------------------------

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearCookies {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_context_id: Option<BrowserContextId>,
}

#[derive(Debug, Deserialize)]
pub struct ClearCookiesResponse {}

impl Command for ClearCookies {
    const METHOD: &'static str = "Storage.clearCookies";
    type Return = ClearCookiesResponse;
}
