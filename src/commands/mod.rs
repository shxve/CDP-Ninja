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
/// 1. Explicit `--target <ID>` — exact match on target id.
/// 2. If no selector, first `page`-type target.
/// 3. Fallback to the first target of any type.
pub fn pick_target(port: u16, requested: Option<&str>) -> Result<TargetInfo> {
    let targets = cdp_ninja::discovery::list_targets(port)?;

    if let Some(id) = requested {
        return targets
            .into_iter()
            .find(|t| t.id.as_deref() == Some(id))
            .ok_or_else(|| anyhow!("no target with id {id}"));
    }

    targets
        .iter()
        .find(|t| t.target_type.as_deref() == Some("page"))
        .cloned()
        .or_else(|| targets.into_iter().next())
        .ok_or_else(|| anyhow!("no debuggable targets on port {port}"))
}
