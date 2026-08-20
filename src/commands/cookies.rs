//! `cdp cookies dump` — read cookies from a running browser and print them
//! as JSON.
//!
//! Two backends:
//! * `--storage` (default) — `Storage.getCookies` on the default browser
//!   context. Browser-wide (all origins), no target attach required, no
//!   Network domain activation. Usually what you want.
//! * `--network` — `Network.getAllCookies` on a page target. Requires
//!   attaching to a target first; scope is the same in practice on a
//!   default context but the code path is different.

use anyhow::{Context, Result};
use clap::{Args, ValueEnum};
use std::fs;
use std::path::PathBuf;

use cdp_ninja::domains::{network, storage};

#[derive(Args)]
pub struct CookiesArgs {
    #[command(subcommand)]
    action: CookiesCommand,
}

#[derive(clap::Subcommand)]
enum CookiesCommand {
    /// Dump all cookies to stdout (or `--out FILE`) as JSON.
    Dump {
        #[arg(long, value_enum, default_value_t = Backend::Storage)]
        via: Backend,
        /// Target id when `--via network`. Defaults to the first page.
        #[arg(long)]
        target: Option<String>,
        /// Write to file instead of stdout.
        #[arg(long, short)]
        out: Option<PathBuf>,
    },
}

#[derive(Copy, Clone, ValueEnum)]
enum Backend {
    /// `Storage.getCookies` on the default browser context (recommended).
    Storage,
    /// `Network.getAllCookies` on a target session.
    Network,
}

pub fn run(port: u16, args: CookiesArgs) -> Result<()> {
    let CookiesCommand::Dump { via, target, out } = args.action;

    let cookies_json = match via {
        Backend::Storage => dump_via_storage(port)?,
        Backend::Network => dump_via_network(port, target.as_deref())?,
    };

    let text = serde_json::to_string_pretty(&cookies_json)?;
    match out {
        Some(path) => {
            fs::write(&path, text).with_context(|| format!("write {}", path.display()))?;
        }
        None => println!("{text}"),
    }
    Ok(())
}

fn dump_via_storage(port: u16) -> Result<serde_json::Value> {
    let mut client = cdp_ninja::Client::connect_browser(port)?;
    let resp = client.call(None, &storage::GetCookies::default())?;
    Ok(serde_json::to_value(resp.cookies)?)
}

fn dump_via_network(port: u16, target: Option<&str>) -> Result<serde_json::Value> {
    let mut client = cdp_ninja::Client::connect_browser(port)?;

    let target_info = super::pick_target(port, target)?;
    let target_id = target_info
        .id
        .ok_or_else(|| anyhow::anyhow!("selected target has no id"))?;
    let session = client.attach_target(&target_id)?;

    let resp = client.call(Some(&session), &network::GetAllCookies)?;
    let _ = client.detach(&session);
    Ok(serde_json::to_value(resp.cookies)?)
}
