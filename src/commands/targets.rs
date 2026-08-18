//! `cdp targets list` — enumerate debuggable targets via `/json`.

use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct TargetsArgs {
    #[command(subcommand)]
    action: TargetsCommand,
}

#[derive(clap::Subcommand)]
enum TargetsCommand {
    /// List all debuggable targets (pages, workers, iframes, extensions).
    List,
}

pub fn run(port: u16, args: TargetsArgs) -> Result<()> {
    match args.action {
        TargetsCommand::List => list(port),
    }
}

fn list(port: u16) -> Result<()> {
    let targets = cdp_ninja::discovery::list_targets(port)?;
    let out: Vec<_> = targets
        .into_iter()
        .map(|t| {
            serde_json::json!({
                "id": t.id,
                "type": t.target_type,
                "title": t.title,
                "url": t.url,
                "wsUrl": t.ws_url,
            })
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}
