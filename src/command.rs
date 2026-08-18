//! The `Command` trait — a typed CDP method binding.
//!
//! A `Command` carries three things:
//! * its wire-format method name (e.g. `"Target.attachToTarget"`)
//! * its parameter shape (via `Serialize`; the struct itself IS the params object)
//! * its response shape (via `Return: DeserializeOwned`)
//!
//! Consumers call [`crate::Client::call`] instead of the low-level
//! [`crate::Client::send`] to get typed round-trips grounded in the canonical
//! CDP protocol schema (`browser_protocol.json` + `js_protocol.json`).
//!
//! # Example
//!
//! ```ignore
//! use cdp_core::domains::network::GetAllCookies;
//! let response = client.call(Some(&session), &GetAllCookies)?;
//! for cookie in response.cookies {
//!     // cookie is a typed cdp_core::domains::network::Cookie
//! }
//! ```

use serde::{de::DeserializeOwned, Serialize};

/// A typed CDP method binding. Implementers are the parameter struct; the
/// return type is associated.
pub trait Command: Serialize {
    /// Wire-format method name, e.g. `"Target.attachToTarget"`.
    const METHOD: &'static str;

    /// Response shape.
    type Return: DeserializeOwned;
}
