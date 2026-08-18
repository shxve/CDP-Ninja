//! Input domain: synthetic keyboard, mouse, and text input.
//!
//! Used for foreground-tab flows that require user-gesture context (autofill
//! password extraction, click-through consent dialogs). `dispatchKeyEvent`
//! and `dispatchMouseEvent` are stable; `insertText` is experimental.
//!
//! Schema reference: `browser_protocol.json` → `Input` domain (stable except
//! where flagged).

use serde::{Deserialize, Serialize};

use crate::command::Command;

/// `Input.MouseButton` — `"none"`, `"left"`, `"middle"`, `"right"`,
/// `"back"`, `"forward"`.
pub type MouseButton = String;

// -- Input.dispatchKeyEvent --------------------------------------------------

/// `type` values per the schema: `"keyDown"`, `"keyUp"`, `"rawKeyDown"`,
/// `"char"`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DispatchKeyEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    /// Bit mask: `Alt=1`, `Ctrl=2`, `Meta/Command=4`, `Shift=8`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmodified_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_virtual_key_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_virtual_key_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repeat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_keypad: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_system_key: Option<bool>,
    /// `0`=standard, `1`=left, `2`=right, `3`=numpad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<i32>,
}

impl DispatchKeyEvent {
    /// Preset: press a printable character. Sends a `char`-type event with
    /// the given text.
    pub fn char(text: impl Into<String>) -> Self {
        Self {
            event_type: "char".to_string(),
            text: Some(text.into()),
            modifiers: None,
            timestamp: None,
            unmodified_text: None,
            key_identifier: None,
            code: None,
            key: None,
            windows_virtual_key_code: None,
            native_virtual_key_code: None,
            auto_repeat: None,
            is_keypad: None,
            is_system_key: None,
            location: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DispatchKeyEventResponse {}

impl Command for DispatchKeyEvent {
    const METHOD: &'static str = "Input.dispatchKeyEvent";
    type Return = DispatchKeyEventResponse;
}

// -- Input.dispatchMouseEvent ------------------------------------------------

/// `type` values: `"mousePressed"`, `"mouseReleased"`, `"mouseMoved"`,
/// `"mouseWheel"`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DispatchMouseEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub x: f64,
    pub y: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<MouseButton>,
    /// Bit mask of currently-pressed buttons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_y: Option<f64>,
    /// `"mouse"` or `"pen"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer_type: Option<String>,
}

impl DispatchMouseEvent {
    /// Preset: single left click at `(x, y)`. Emits a mousePressed + does not
    /// emit the release — call again with `type = "mouseReleased"` if a full
    /// click is needed.
    pub fn click_left(x: f64, y: f64) -> Self {
        Self {
            event_type: "mousePressed".to_string(),
            x,
            y,
            modifiers: None,
            timestamp: None,
            button: Some("left".to_string()),
            buttons: Some(1),
            click_count: Some(1),
            delta_x: None,
            delta_y: None,
            pointer_type: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DispatchMouseEventResponse {}

impl Command for DispatchMouseEvent {
    const METHOD: &'static str = "Input.dispatchMouseEvent";
    type Return = DispatchMouseEventResponse;
}

// -- Input.insertText (experimental) -----------------------------------------

/// Experimental in the CDP schema. Inserts text as if it were dispatched via
/// the IME, respecting the currently-focused element.
#[derive(Debug, Serialize)]
pub struct InsertText {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct InsertTextResponse {}

impl Command for InsertText {
    const METHOD: &'static str = "Input.insertText";
    type Return = InsertTextResponse;
}
