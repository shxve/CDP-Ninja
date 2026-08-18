//! Chrome DevTools Protocol client — transport, session multiplexing, discovery.
//!
//! Sync-first. Backed by `tungstenite` (WebSocket) and `ureq` (HTTP). Keeps
//! `tokio` out of consumers that don't want it.
//!
//! # Layers
//!
//! * [`discovery`] — HTTP `/json`, `/json/version`, `/json/list` for a running
//!   browser at a known loopback port.
//! * [`message`] — CDP request/response/event JSON envelope types.
//! * [`transport`] — blocking WebSocket wrapper that owns the socket.
//! * [`client`] — single-socket multiplexing across the browser session and
//!   per-target sessions via CDP's flat-session `sessionId` mode.
//!
//! # Session model
//!
//! CDP uses one WebSocket per browser. Each response and event carries an
//! optional `sessionId` — messages with no `sessionId` are directed at the
//! browser session; messages with a `sessionId` are directed at the target
//! session identified by that id. [`Client::attach_target`] uses
//! `Target.attachToTarget {flatten: true}` and returns the assigned session
//! id; subsequent calls take the session id explicitly.
//!
//! This model is why per-tab WebSocket connections miss events on other
//! targets and serialise operations that should run in parallel.

pub mod client;
pub mod command;
pub mod discovery;
pub mod domains;
pub mod message;
pub mod transport;

pub use client::{Client, SessionId};
pub use command::Command;
pub use discovery::{browser_ws_url, list_targets, TargetInfo};
pub use message::{Event, Response};
pub use transport::Transport;

/// Umbrella error type surfaced to consumers.
#[derive(Debug, thiserror::Error)]
pub enum CdpError {
    #[error("discovery failed: {0}")]
    Discovery(String),
    #[error("transport failed: {0}")]
    Transport(String),
    #[error("CDP command failed: {code} {message}")]
    Command { code: i64, message: String },
    #[error("CDP response missing required field: {0}")]
    Parse(String),
    #[error("no target found matching {0}")]
    NoTarget(String),
    #[error("closed")]
    Closed,
}

pub type Result<T> = std::result::Result<T, CdpError>;
