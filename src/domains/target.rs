//! Target domain: attach to and enumerate debugging targets.
//!
//! Schema reference: `browser_protocol.json` → `Target` domain (stable).

use serde::{Deserialize, Serialize};

use crate::command::Command;

/// A protocol-level `TargetID` (opaque string).
pub type TargetId = String;

/// A protocol-level `SessionID` (opaque string). Distinct from
/// [`crate::SessionId`] — this is the raw wire form; the client-facing wrapper
/// is used at API boundaries.
pub type SessionIdString = String;

/// A protocol-level `BrowserContextID` (opaque string).
pub type BrowserContextId = String;

/// `Target.TargetInfo` — the schema type returned by [`GetTargets`] and
/// emitted by target-lifecycle events. Distinct from
/// [`crate::discovery::TargetInfo`] which mirrors the `/json` HTTP endpoint
/// (similar shape, different provenance).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetInfo {
    pub target_id: TargetId,
    #[serde(rename = "type")]
    pub target_type: String,
    pub title: String,
    pub url: String,
    pub attached: bool,
    #[serde(default)]
    pub opener_id: Option<TargetId>,
    #[serde(default)]
    pub browser_context_id: Option<BrowserContextId>,
}

// -- Target.attachToTarget ---------------------------------------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachToTarget {
    pub target_id: TargetId,
    /// Enable "flat" session mode: subsequent messages carry the returned
    /// `sessionId` at the top level instead of being wrapped in
    /// `Target.sendMessageToTarget`. Cdp-core's `Client` requires flat mode;
    /// always send `true`.
    pub flatten: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachToTargetResponse {
    pub session_id: SessionIdString,
}

impl Command for AttachToTarget {
    const METHOD: &'static str = "Target.attachToTarget";
    type Return = AttachToTargetResponse;
}

// -- Target.detachFromTarget -------------------------------------------------

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DetachFromTarget {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<SessionIdString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<TargetId>,
}

#[derive(Debug, Deserialize)]
pub struct DetachFromTargetResponse {}

impl Command for DetachFromTarget {
    const METHOD: &'static str = "Target.detachFromTarget";
    type Return = DetachFromTargetResponse;
}

// -- Target.getTargets -------------------------------------------------------

#[derive(Debug, Serialize, Default)]
pub struct GetTargets;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTargetsResponse {
    pub target_infos: Vec<TargetInfo>,
}

impl Command for GetTargets {
    const METHOD: &'static str = "Target.getTargets";
    type Return = GetTargetsResponse;
}
