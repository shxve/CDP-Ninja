//! Network domain: currently only `Network.getAllCookies`.
//!
//! For post-exploitation cookie extraction consider
//! [`crate::domains::storage::GetCookies`] instead — it queries the live
//! browser cookie store rather than the encrypted on-disk SQLite database
//! and preserves fewer forensic artifacts.
//!
//! Schema reference: `browser_protocol.json` → `Network` domain (stable).

use serde::{Deserialize, Serialize};

use crate::command::Command;

/// `Network.CookieSameSite` — SameSite attribute value.
///
/// The schema defines this as an enum of Strict / Lax / None. Kept as `String`
/// to be forward-compatible; consumers can match on the raw value.
pub type CookieSameSite = String;

/// `Network.Cookie` — canonical CDP cookie shape.
///
/// This mirrors the schema exactly. Consumers translate to their own domain
/// types via `From`/`TryFrom` as needed (see `lester-cdp::types` for
/// the Lester-side adapter). `Serialize` is implemented for CLI/JSON
/// re-emission — the same struct round-trips through the wire format.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    /// Expiration date as Unix epoch seconds; `-1` (or absent) means session cookie.
    #[serde(default)]
    pub expires: f64,
    pub size: u64,
    pub http_only: bool,
    pub secure: bool,
    pub session: bool,
    #[serde(default)]
    pub same_site: Option<CookieSameSite>,
    #[serde(default)]
    pub priority: Option<String>,
    #[serde(default)]
    pub same_party: bool,
    #[serde(default)]
    pub source_scheme: Option<String>,
    #[serde(default)]
    pub source_port: Option<i32>,
    #[serde(default)]
    pub partition_key: Option<serde_json::Value>,
    #[serde(default)]
    pub partition_key_opaque: bool,
}

// -- Network.getAllCookies ---------------------------------------------------

#[derive(Debug, Serialize, Default)]
pub struct GetAllCookies;

#[derive(Debug, Deserialize)]
pub struct GetAllCookiesResponse {
    pub cookies: Vec<Cookie>,
}

impl Command for GetAllCookies {
    const METHOD: &'static str = "Network.getAllCookies";
    type Return = GetAllCookiesResponse;
}
