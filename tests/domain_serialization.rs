//! Wire-format fidelity tests. Every domain command must serialize to the
//! exact JSON shape the CDP schema demands: camelCase field names, correct
//! optional-field elision, and correct field renames (`type` for enum-typed
//! event kind fields, `sessionId` at the frame level, etc.).
//!
//! These tests are cheap insurance against schema drift and typos — the
//! authoritative reference is `~/dev/reference/cdp/browser_protocol.json`.

use cdp_core::domains::{fetch, input, page, storage, target};
use cdp_core::Command;

/// Every Command's METHOD constant should be `Domain.method` — schema
/// convention.
#[test]
fn method_names_match_schema() {
    assert_eq!(target::AttachToTarget::METHOD, "Target.attachToTarget");
    assert_eq!(target::DetachFromTarget::METHOD, "Target.detachFromTarget");
    assert_eq!(target::GetTargets::METHOD, "Target.getTargets");
    assert_eq!(storage::GetCookies::METHOD, "Storage.getCookies");
    assert_eq!(page::CaptureScreenshot::METHOD, "Page.captureScreenshot");
    assert_eq!(page::Navigate::METHOD, "Page.navigate");
    assert_eq!(fetch::Enable::METHOD, "Fetch.enable");
    assert_eq!(fetch::ContinueRequest::METHOD, "Fetch.continueRequest");
    assert_eq!(input::DispatchKeyEvent::METHOD, "Input.dispatchKeyEvent");
    assert_eq!(input::DispatchMouseEvent::METHOD, "Input.dispatchMouseEvent");
}

#[test]
fn page_capture_screenshot_defaults_are_empty() {
    let cmd = page::CaptureScreenshot::default();
    let params = serde_json::to_value(&cmd).unwrap();
    // No None fields should be serialized.
    assert_eq!(params, serde_json::json!({}));
}

#[test]
fn page_capture_screenshot_field_names_are_camel_case() {
    let cmd = page::CaptureScreenshot {
        format: Some("jpeg".to_string()),
        quality: Some(80),
        from_surface: Some(true),
        capture_beyond_viewport: Some(true),
        ..Default::default()
    };
    let params = serde_json::to_value(&cmd).unwrap();
    assert_eq!(params["format"], "jpeg");
    assert_eq!(params["quality"], 80);
    assert_eq!(params["fromSurface"], true);
    assert_eq!(params["captureBeyondViewport"], true);
    // No snake_case leaks:
    assert!(params.get("from_surface").is_none());
    assert!(params.get("capture_beyond_viewport").is_none());
}

#[test]
fn page_navigate_url_is_required_field() {
    let cmd = page::Navigate::to("https://example.com/");
    let params = serde_json::to_value(&cmd).unwrap();
    assert_eq!(params["url"], "https://example.com/");
    // Optional fields elided:
    assert!(params.get("referrer").is_none());
    assert!(params.get("transitionType").is_none());
}

#[test]
fn fetch_enable_patterns_serialize_correctly() {
    let cmd = fetch::Enable {
        patterns: Some(vec![
            fetch::RequestPattern {
                url_pattern: Some("*://example.com/*".to_string()),
                resource_type: Some("XHR".to_string()),
                request_stage: Some("Response".to_string()),
            },
        ]),
        handle_auth_requests: Some(true),
    };
    let params = serde_json::to_value(&cmd).unwrap();
    assert_eq!(params["handleAuthRequests"], true);
    assert_eq!(params["patterns"][0]["urlPattern"], "*://example.com/*");
    assert_eq!(params["patterns"][0]["resourceType"], "XHR");
    assert_eq!(params["patterns"][0]["requestStage"], "Response");
}

#[test]
fn fetch_continue_with_auth_shape() {
    let cmd = fetch::ContinueWithAuth {
        request_id: "req-1".to_string(),
        auth_challenge_response: fetch::AuthChallengeResponse {
            response: "ProvideCredentials".to_string(),
            username: Some("alice".to_string()),
            password: Some("hunter2".to_string()),
        },
    };
    let params = serde_json::to_value(&cmd).unwrap();
    assert_eq!(params["requestId"], "req-1");
    assert_eq!(params["authChallengeResponse"]["response"], "ProvideCredentials");
    assert_eq!(params["authChallengeResponse"]["username"], "alice");
    assert_eq!(params["authChallengeResponse"]["password"], "hunter2");
}

#[test]
fn input_dispatch_key_event_uses_type_field_not_event_type() {
    // The schema field is named "type" — a Rust keyword — and must be
    // renamed via #[serde(rename = "type")] rather than leaking as
    // "event_type".
    let cmd = input::DispatchKeyEvent::char("A");
    let params = serde_json::to_value(&cmd).unwrap();
    assert_eq!(params["type"], "char");
    assert_eq!(params["text"], "A");
    assert!(params.get("event_type").is_none());
    assert!(params.get("eventType").is_none());
}

#[test]
fn input_dispatch_mouse_event_click_left_preset() {
    let cmd = input::DispatchMouseEvent::click_left(100.0, 200.0);
    let params = serde_json::to_value(&cmd).unwrap();
    assert_eq!(params["type"], "mousePressed");
    assert_eq!(params["x"], 100.0);
    assert_eq!(params["y"], 200.0);
    assert_eq!(params["button"], "left");
    assert_eq!(params["clickCount"], 1);
    assert_eq!(params["buttons"], 1);
}

#[test]
fn storage_get_cookies_elides_absent_context() {
    let params = serde_json::to_value(&storage::GetCookies::default()).unwrap();
    assert_eq!(params, serde_json::json!({}));
}

#[test]
fn target_attach_to_target_serializes_target_id_and_flatten() {
    let cmd = target::AttachToTarget {
        target_id: "t-1".to_string(),
        flatten: true,
    };
    let params = serde_json::to_value(&cmd).unwrap();
    assert_eq!(params["targetId"], "t-1");
    assert_eq!(params["flatten"], true);
    assert!(params.get("target_id").is_none());
}

#[test]
fn target_info_deserializes_from_wire_format() {
    let wire = r#"{
        "targetId": "abc",
        "type": "page",
        "title": "Home",
        "url": "https://example.com/",
        "attached": true,
        "browserContextId": "ctx-1"
    }"#;
    let info: target::TargetInfo = serde_json::from_str(wire).unwrap();
    assert_eq!(info.target_id, "abc");
    assert_eq!(info.target_type, "page");
    assert_eq!(info.browser_context_id.as_deref(), Some("ctx-1"));
    assert!(info.attached);
}
