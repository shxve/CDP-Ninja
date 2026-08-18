//! Fetch domain: intercept + rewrite requests and responses.
//!
//! Pause a request, rewrite headers/body, continue or fulfill locally.
//! Also handles HTTP auth challenges via [`ContinueWithAuth`].
//!
//! Schema reference: `browser_protocol.json` → `Fetch` domain (stable).

use serde::{Deserialize, Serialize};

use crate::command::Command;

pub type RequestId = String;

/// `Fetch.RequestStage` — `"Request"` (before send) or `"Response"` (after
/// server response headers, before body forwarding).
pub type RequestStage = String;

/// `Fetch.RequestPattern` — filter for `Fetch.enable` intercept scope.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestPattern {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_pattern: Option<String>,
    /// One of `Document`, `Stylesheet`, `Image`, `Media`, `Font`, `Script`,
    /// `TextTrack`, `XHR`, `Fetch`, `Prefetch`, `EventSource`, `WebSocket`,
    /// `Manifest`, `SignedExchange`, `Ping`, `CSPViolationReport`,
    /// `Preflight`, `FedCM`, `Other`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_stage: Option<RequestStage>,
}

/// A single HTTP header for [`ContinueRequest`] / [`FulfillRequest`].
#[derive(Debug, Clone, Serialize)]
pub struct HeaderEntry {
    pub name: String,
    pub value: String,
}

/// Answer to an HTTP auth challenge, supplied via [`ContinueWithAuth`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthChallengeResponse {
    /// `"Default"`, `"CancelAuth"`, or `"ProvideCredentials"`.
    pub response: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

// -- Fetch.enable / disable --------------------------------------------------

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Enable {
    /// If empty (or omitted) the domain intercepts everything.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patterns: Option<Vec<RequestPattern>>,
    /// If true, `Fetch.authRequired` fires for each auth challenge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle_auth_requests: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct EnableResponse {}

impl Command for Enable {
    const METHOD: &'static str = "Fetch.enable";
    type Return = EnableResponse;
}

#[derive(Debug, Serialize, Default)]
pub struct Disable;
#[derive(Debug, Deserialize)]
pub struct DisableResponse {}
impl Command for Disable {
    const METHOD: &'static str = "Fetch.disable";
    type Return = DisableResponse;
}

// -- Fetch.continueRequest ---------------------------------------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinueRequest {
    pub request_id: RequestId,
    /// Override the URL (retains scheme + method).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Base64-encoded body override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HeaderEntry>>,
    /// If true, `Fetch.requestPaused` fires again at the Response stage so
    /// the caller can modify the response before it reaches the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intercept_response: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ContinueRequestResponse {}

impl Command for ContinueRequest {
    const METHOD: &'static str = "Fetch.continueRequest";
    type Return = ContinueRequestResponse;
}

// -- Fetch.continueWithAuth --------------------------------------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinueWithAuth {
    pub request_id: RequestId,
    pub auth_challenge_response: AuthChallengeResponse,
}

#[derive(Debug, Deserialize)]
pub struct ContinueWithAuthResponse {}

impl Command for ContinueWithAuth {
    const METHOD: &'static str = "Fetch.continueWithAuth";
    type Return = ContinueWithAuthResponse;
}

// -- Fetch.failRequest -------------------------------------------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FailRequest {
    pub request_id: RequestId,
    /// `Network.ErrorReason` — `"Failed"`, `"Aborted"`, `"TimedOut"`,
    /// `"AccessDenied"`, `"ConnectionClosed"`, `"ConnectionReset"`,
    /// `"ConnectionRefused"`, `"ConnectionAborted"`, `"ConnectionFailed"`,
    /// `"NameNotResolved"`, `"InternetDisconnected"`, `"AddressUnreachable"`,
    /// `"BlockedByClient"`, `"BlockedByResponse"`.
    pub error_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct FailRequestResponse {}

impl Command for FailRequest {
    const METHOD: &'static str = "Fetch.failRequest";
    type Return = FailRequestResponse;
}

// -- Fetch.fulfillRequest ----------------------------------------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FulfillRequest {
    pub request_id: RequestId,
    /// HTTP status code (e.g. 200, 404).
    pub response_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<Vec<HeaderEntry>>,
    /// Alternate to `response_headers`: raw header block terminated by `\0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_response_headers: Option<String>,
    /// Base64-encoded response body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Status-phrase override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_phrase: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FulfillRequestResponse {}

impl Command for FulfillRequest {
    const METHOD: &'static str = "Fetch.fulfillRequest";
    type Return = FulfillRequestResponse;
}
