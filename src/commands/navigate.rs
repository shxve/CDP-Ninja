//! `cdp navigate` — navigate a page target to a URL.

use anyhow::Result;
use clap::Args;

use cdp_ninja::domains::page;

#[derive(Args)]
pub struct NavigateArgs {
    /// Target id to navigate. Defaults to the first page target.
    #[arg(long)]
    target: Option<String>,
    /// Destination URL.
    url: String,
}

pub fn run(port: u16, args: NavigateArgs) -> Result<()> {
    let target = super::pick_target(port, args.target.as_deref())?;
    let target_id = target
        .id
        .ok_or_else(|| anyhow::anyhow!("selected target has no id"))?;

    let mut client = cdp_ninja::Client::connect_browser(port)?;
    let session = client.attach_target(&target_id)?;

    let resp = client.call(Some(&session), &page::Navigate::to(&args.url))?;
    let _ = client.detach(&session);

    if let Some(err) = resp.error_text {
        anyhow::bail!("navigation error: {err}");
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "frameId": resp.frame_id,
            "loaderId": resp.loader_id,
        }))?
    );
    Ok(())
}
