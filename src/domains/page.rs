//! Page domain: navigation, screenshots, PDF, persistent script injection.
//!
//! Schema reference: `browser_protocol.json` → `Page` domain (stable except
//! where flagged).

use serde::{Deserialize, Serialize};

use crate::command::Command;

/// `Page.FrameId` (opaque string).
pub type FrameId = String;
/// `Page.ScriptIdentifier` (opaque string).
pub type ScriptIdentifier = String;
/// `Network.LoaderId` (opaque string) — referenced by `Page.navigate`.
pub type LoaderId = String;
/// `IO.StreamHandle` (opaque string) — returned by `Page.printToPDF` when
/// `transferMode: "ReturnAsStream"`.
pub type StreamHandle = String;

/// `Page.Viewport` — clip region for [`CaptureScreenshot`].
#[derive(Debug, Clone, Serialize)]
pub struct Viewport {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub scale: f64,
}

// -- Page.enable / disable ---------------------------------------------------

#[derive(Debug, Serialize, Default)]
pub struct Enable;
#[derive(Debug, Deserialize)]
pub struct EnableResponse {}
impl Command for Enable {
    const METHOD: &'static str = "Page.enable";
    type Return = EnableResponse;
}

#[derive(Debug, Serialize, Default)]
pub struct Disable;
#[derive(Debug, Deserialize)]
pub struct DisableResponse {}
impl Command for Disable {
    const METHOD: &'static str = "Page.disable";
    type Return = DisableResponse;
}

// -- Page.captureScreenshot --------------------------------------------------

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CaptureScreenshot {
    /// `"png"` (default) or `"jpeg"` or `"webp"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// 0–100, only meaningful for `"jpeg"`/`"webp"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<Viewport>,
    /// If true, capture from the surface rather than the view (default true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_surface: Option<bool>,
    /// Capture the full page beyond the viewport.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_beyond_viewport: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimize_for_speed: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CaptureScreenshotResponse {
    /// Base64-encoded image bytes.
    pub data: String,
}

impl Command for CaptureScreenshot {
    const METHOD: &'static str = "Page.captureScreenshot";
    type Return = CaptureScreenshotResponse;
}

// -- Page.navigate -----------------------------------------------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Navigate {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
    /// `"typed"`, `"link"`, `"reload"`, `"auto_toplevel"`, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<FrameId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer_policy: Option<String>,
}

impl Navigate {
    /// `Page.navigate` with only the URL set. Uses the page's default frame
    /// and transition type.
    pub fn to(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            referrer: None,
            transition_type: None,
            frame_id: None,
            referrer_policy: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigateResponse {
    pub frame_id: FrameId,
    #[serde(default)]
    pub loader_id: Option<LoaderId>,
    /// Populated when the navigation failed at the browser level (invalid URL,
    /// blocked scheme, etc.). HTTP error responses do NOT surface here.
    #[serde(default)]
    pub error_text: Option<String>,
}

impl Command for Navigate {
    const METHOD: &'static str = "Page.navigate";
    type Return = NavigateResponse;
}

// -- Page.printToPDF ---------------------------------------------------------

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PrintToPdf {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landscape: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_header_footer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_top: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_bottom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_left: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_right: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_ranges: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_css_page_size: Option<bool>,
    /// `"ReturnAsBase64"` (default; data field populated) or
    /// `"ReturnAsStream"` (stream field populated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_mode: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PrintToPdfResponse {
    /// Base64-encoded PDF bytes (present when transferMode was default).
    pub data: String,
    /// Stream handle to read the PDF from (present when transferMode was
    /// `"ReturnAsStream"`).
    #[serde(default)]
    pub stream: Option<StreamHandle>,
}

impl Command for PrintToPdf {
    const METHOD: &'static str = "Page.printToPDF";
    type Return = PrintToPdfResponse;
}

// -- Page.addScriptToEvaluateOnNewDocument -----------------------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddScriptToEvaluateOnNewDocument {
    pub source: String,
    /// Optional isolated-world name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_command_line_api: Option<bool>,
    /// If true, run against the currently loaded document as well as future
    /// ones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_immediately: Option<bool>,
}

impl AddScriptToEvaluateOnNewDocument {
    pub fn with_source(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            world_name: None,
            include_command_line_api: None,
            run_immediately: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AddScriptToEvaluateOnNewDocumentResponse {
    pub identifier: ScriptIdentifier,
}

impl Command for AddScriptToEvaluateOnNewDocument {
    const METHOD: &'static str = "Page.addScriptToEvaluateOnNewDocument";
    type Return = AddScriptToEvaluateOnNewDocumentResponse;
}

// -- Page.removeScriptToEvaluateOnNewDocument --------------------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveScriptToEvaluateOnNewDocument {
    pub identifier: ScriptIdentifier,
}

#[derive(Debug, Deserialize)]
pub struct RemoveScriptToEvaluateOnNewDocumentResponse {}

impl Command for RemoveScriptToEvaluateOnNewDocument {
    const METHOD: &'static str = "Page.removeScriptToEvaluateOnNewDocument";
    type Return = RemoveScriptToEvaluateOnNewDocumentResponse;
}

// -- Page.reload -------------------------------------------------------------

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Reload {
    /// If true, ignore the browser's cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_cache: Option<bool>,
    /// JS source to inject on load.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_to_evaluate_on_load: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReloadResponse {}

impl Command for Reload {
    const METHOD: &'static str = "Page.reload";
    type Return = ReloadResponse;
}
