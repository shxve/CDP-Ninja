//! `cdp` — Rust CLI for the Chrome DevTools Protocol.
//!
//! Drives any already-open CDP port. Does NOT enable CDP on a browser
//! process; use it against a browser that was launched with
//! `--remote-debugging-port` or against a port that some other tool opened.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

mod commands;

/// `cdp` — Chrome DevTools Protocol CLI.
#[derive(Parser)]
#[command(name = "cdp", version, about, long_about = None)]
struct Cli {
    /// Loopback port CDP is listening on. Defaults to Chrome's usual port.
    #[arg(long, short, default_value_t = 8181, global = true)]
    port: u16,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Probe a port for an open CDP endpoint. Prints browser version on success.
    Probe,
    /// Target enumeration and inspection.
    Targets(commands::targets::TargetsArgs),
    /// Cookie extraction.
    Cookies(commands::cookies::CookiesArgs),
    /// Capture a screenshot of a page target.
    Screenshot(commands::screenshot::ScreenshotArgs),
    /// Evaluate a JavaScript expression in a page context.
    Eval(commands::eval::EvalArgs),
    /// Navigate a page target to a URL.
    Navigate(commands::navigate::NavigateArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Probe => commands::probe::run(cli.port).context("probe failed"),
        Command::Targets(args) => commands::targets::run(cli.port, args).context("targets failed"),
        Command::Cookies(args) => commands::cookies::run(cli.port, args).context("cookies failed"),
        Command::Screenshot(args) => {
            commands::screenshot::run(cli.port, args).context("screenshot failed")
        }
        Command::Eval(args) => commands::eval::run(cli.port, args).context("eval failed"),
        Command::Navigate(args) => {
            commands::navigate::run(cli.port, args).context("navigate failed")
        }
    }
}
