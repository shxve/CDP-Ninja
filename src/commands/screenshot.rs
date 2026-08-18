//! `cdp screenshot` — capture a page target as PNG (or JPEG/WebP).

use anyhow::{Context, Result};
use base64::Engine;
use clap::Args;
use std::fs;
use std::path::PathBuf;

use cdp_ninja::domains::page;

#[derive(Args)]
pub struct ScreenshotArgs {
    /// Target id to capture. Defaults to the first page target.
    #[arg(long)]
    target: Option<String>,
    /// `png` (default), `jpeg`, or `webp`.
    #[arg(long, default_value = "png")]
    format: String,
    /// JPEG/WebP quality 0–100.
    #[arg(long)]
    quality: Option<u8>,
    /// Capture the full page beyond the current viewport.
    #[arg(long)]
    full_page: bool,
    /// Output file. Defaults to `screenshot.<format>` in the current directory.
    #[arg(long, short)]
    out: Option<PathBuf>,
}

pub fn run(port: u16, args: ScreenshotArgs) -> Result<()> {
    let target = super::pick_target(port, args.target.as_deref())?;
    let target_id = target
        .id
        .ok_or_else(|| anyhow::anyhow!("selected target has no id"))?;

    let mut client = cdp_ninja::Client::connect_browser(port)?;
    let session = client.attach_target(&target_id)?;

    let cmd = page::CaptureScreenshot {
        format: Some(args.format.clone()),
        quality: args.quality,
        capture_beyond_viewport: Some(args.full_page),
        ..Default::default()
    };
    let resp = client.call(Some(&session), &cmd)?;
    let _ = client.detach(&session);

    let bytes = base64::engine::general_purpose::STANDARD
        .decode(&resp.data)
        .context("decode screenshot base64")?;

    let out = args
        .out
        .unwrap_or_else(|| PathBuf::from(format!("screenshot.{}", args.format)));
    fs::write(&out, &bytes).with_context(|| format!("write {}", out.display()))?;
    eprintln!("wrote {} bytes to {}", bytes.len(), out.display());
    Ok(())
}
