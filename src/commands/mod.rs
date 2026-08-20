//! Subcommand implementations. Each module exposes an `Args` struct (or
//! nothing, for arg-less commands) and a `run(port, args) -> Result<()>`.

pub mod cookies;
pub mod eval;
pub mod navigate;
pub mod probe;
pub mod screenshot;
pub mod targets;

use anyhow::{anyhow, Result};
use cdp_ninja::discovery::TargetInfo;

/// Resolve a target selector to a concrete target id.
///
/// Selection order:
/// 1. Explicit `--target <ID>` — exact match on target id, any type. The
///    operator asked for it; we assume they know what type it is.
/// 2. No selector — the first `page`-type target.
///
/// Fails (instead of falling back) when no `--target` was passed and no
/// `page` target exists. The prior "first target of any type" fallback
/// silently attached to service workers or extensions and produced opaque
/// CDP errors when the caller then ran a page-only command like navigate
/// or screenshot.
pub fn pick_target(port: u16, requested: Option<&str>) -> Result<TargetInfo> {
    let targets = cdp_ninja::discovery::list_targets(port)?;
    select_target(targets, requested, port)
}

/// Pure selection logic factored out for unit testing.
fn select_target(
    targets: Vec<TargetInfo>,
    requested: Option<&str>,
    port: u16,
) -> Result<TargetInfo> {
    if let Some(id) = requested {
        return targets
            .into_iter()
            .find(|t| t.id.as_deref() == Some(id))
            .ok_or_else(|| anyhow!("no target with id {id}"));
    }

    if targets.is_empty() {
        return Err(anyhow!("no debuggable targets on port {port}"));
    }

    targets
        .into_iter()
        .find(|t| t.target_type.as_deref() == Some("page"))
        .ok_or_else(|| {
            anyhow!(
                "no `page` target on port {port}; pass --target <ID> to select a \
                 non-page target explicitly (try `cdp targets list`)"
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(id: &str, ty: &str) -> TargetInfo {
        TargetInfo {
            id: Some(id.to_string()),
            title: None,
            url: None,
            target_type: Some(ty.to_string()),
            ws_url: None,
            frontend_url: None,
        }
    }

    #[test]
    fn explicit_id_matches_regardless_of_type() {
        let ts = vec![t("a", "service_worker"), t("b", "page")];
        let picked = select_target(ts, Some("a"), 8181).unwrap();
        assert_eq!(picked.id.as_deref(), Some("a"));
    }

    #[test]
    fn explicit_id_errors_when_not_found() {
        let ts = vec![t("a", "page")];
        let err = select_target(ts, Some("nope"), 8181).unwrap_err().to_string();
        assert!(err.contains("no target with id nope"), "got: {err}");
    }

    #[test]
    fn no_selector_picks_first_page_skipping_non_pages() {
        let ts = vec![
            t("sw", "service_worker"),
            t("p1", "page"),
            t("p2", "page"),
        ];
        let picked = select_target(ts, None, 8181).unwrap();
        assert_eq!(picked.id.as_deref(), Some("p1"));
    }

    #[test]
    fn no_selector_errors_when_no_page_target() {
        // Regression: previously this would silently return the service
        // worker and then blow up in a downstream navigate/screenshot call.
        let ts = vec![t("sw", "service_worker"), t("ext", "background_page")];
        let err = select_target(ts, None, 8181).unwrap_err().to_string();
        assert!(err.contains("no `page` target"), "got: {err}");
    }

    #[test]
    fn no_selector_errors_when_no_targets_at_all() {
        let err = select_target(vec![], None, 8181).unwrap_err().to_string();
        assert!(err.contains("no debuggable targets"), "got: {err}");
    }
}
