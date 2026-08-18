//! `cdp eval` — evaluate a JavaScript expression in a target's default
//! execution context, printing the returned value as JSON.

use anyhow::Result;
use clap::Args;

use cdp_ninja::domains::runtime;

#[derive(Args)]
pub struct EvalArgs {
    /// Target id to run against. Defaults to the first page target.
    #[arg(long)]
    target: Option<String>,
    /// JavaScript expression. Wrap in an IIFE if you need statements.
    expression: String,
    /// Await the expression when it evaluates to a Promise.
    #[arg(long)]
    await_promise: bool,
}

pub fn run(port: u16, args: EvalArgs) -> Result<()> {
    let target = super::pick_target(port, args.target.as_deref())?;
    let target_id = target
        .id
        .ok_or_else(|| anyhow::anyhow!("selected target has no id"))?;

    let mut client = cdp_ninja::Client::connect_browser(port)?;
    let session = client.attach_target(&target_id)?;

    let mut cmd = runtime::Evaluate::returning_value(&args.expression);
    if args.await_promise {
        cmd.await_promise = Some(true);
    }
    let resp = client.call(Some(&session), &cmd)?;
    let _ = client.detach(&session);

    if let Some(exc) = resp.exception_details {
        anyhow::bail!("evaluation threw: {}", serde_json::to_string_pretty(&exc)?);
    }
    println!("{}", serde_json::to_string_pretty(&resp.result)?);
    Ok(())
}
