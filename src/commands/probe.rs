//! `cdp probe` — check whether CDP is open on the given port and print
//! Browser.getVersion output.

use anyhow::{Context, Result};
use cdp_ninja::domains::browser::GetVersion;

pub fn run(port: u16) -> Result<()> {
    if !cdp_ninja::discovery::probe(port) {
        anyhow::bail!("no CDP endpoint on 127.0.0.1:{port}");
    }

    let mut client = cdp_ninja::Client::connect_browser(port)
        .with_context(|| format!("connect to CDP browser session on port {port}"))?;

    let version = client
        .call(None, &GetVersion)
        .context("Browser.getVersion")?;

    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "port": port,
            "product": version.product,
            "revision": version.revision,
            "userAgent": version.user_agent,
            "protocolVersion": version.protocol_version,
            "jsVersion": version.js_version,
        }))?
    );
    Ok(())
}
