//! Accessibility domain: extract the accessibility tree.
//!
//! `Accessibility.getFullAXTree` is a cheap element-discovery primitive —
//! returns a serialized ARIA tree without needing DOM/CSS selector logic in
//! the client. Preferred over `DOMSnapshot.captureSnapshot` when the goal is
//! "find the clickable / interactive elements on the page."
//!
//! Both commands are marked experimental in the CDP schema.
//!
//! Schema reference: `browser_protocol.json` → `Accessibility` domain
//! (EXPERIMENTAL).

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::command::Command;
use crate::domains::page::FrameId;

/// `Accessibility.AXNode` — kept as `serde_json::Value` because the shape is
/// deeply nested (properties, name, description, role, value, children) and
/// consumers typically stringify + regex or JSON-path into it rather than
/// walking typed fields.
pub type AxNode = Value;

// -- Accessibility.enable ----------------------------------------------------

#[derive(Debug, Serialize, Default)]
pub struct Enable;

#[derive(Debug, Deserialize)]
pub struct EnableResponse {}

impl Command for Enable {
    const METHOD: &'static str = "Accessibility.enable";
    type Return = EnableResponse;
}

// -- Accessibility.getFullAXTree ---------------------------------------------

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFullAxTree {
    /// Maximum tree depth (default: entire subtree).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Restrict to the frame with this id (default: main frame).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<FrameId>,
}

#[derive(Debug, Deserialize)]
pub struct GetFullAxTreeResponse {
    pub nodes: Vec<AxNode>,
}

impl Command for GetFullAxTree {
    const METHOD: &'static str = "Accessibility.getFullAXTree";
    type Return = GetFullAxTreeResponse;
}
