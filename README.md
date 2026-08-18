# CDP-Ninja

Chrome DevTools Protocol client library **and** CLI toolkit. Sync-first
Rust. Typed method bindings grounded in the canonical protocol schema.
Single-socket flat-session multiplexing. No `tokio`, no code-gen.

Ships two things in one crate:

* **Library `cdp_ninja`** — the Rust API. Consume as a dep from any
  Rust project that needs to talk CDP.
* **Binary `cdp`** — a CLI post-exploitation surface over any
  already-open CDP port.

## Install

```
cargo install --git https://github.com/shxve/CDP-Ninja
# Binary at ~/.cargo/bin/cdp
```

Or as a Rust library dep:

```toml
[dependencies]
cdp-ninja = { git = "https://github.com/shxve/CDP-Ninja", branch = "main" }
```

## CLI usage

```
cdp [--port N] <command> [args...]

Commands:
  probe                              Check if CDP is open + print browser version
  targets list                       Enumerate all debuggable targets as JSON
  cookies dump [--via storage|network] [--target ID] [--out FILE]
                                     Extract cookies (storage = browser-wide,
                                     network = per-target)
  screenshot [--target ID] [--format png|jpeg|webp] [--full-page] [--out FILE]
                                     Capture a page screenshot
  eval [--target ID] [--await-promise] <js>
                                     Evaluate a JS expression, print result
  navigate [--target ID] <url>       Navigate a page target
```

`--port` defaults to `8181` (a common Chrome CDP port). Override for Edge
(`8182`), Brave (`8183`), Opera (`8184`), Vivaldi (`8185`), or any custom
port a companion tool opened.

### Examples

```
# Is CDP open?
cdp probe

# Grab all cookies from the browser-wide store as JSON
cdp cookies dump > cookies.json

# Same, but per-target via Network.getAllCookies on the first page target
cdp cookies dump --via network

# Screenshot the current page
cdp screenshot --out home.png

# Full-page (beyond the viewport) at reduced quality
cdp screenshot --format jpeg --quality 70 --full-page --out home.jpg

# Evaluate an expression and print the result
cdp eval 'document.title'

# Await a Promise
cdp eval --await-promise 'fetch("/api/me").then(r => r.json())'

# Navigate
cdp navigate https://example.com/
```

## Library usage

```rust
use cdp_ninja::{Client, domains::{storage::GetCookies, page::Navigate}};

let mut client = Client::connect_browser(8181)?;
let session    = client.attach_target("target-id-here")?;

let cookies = client.call(None, &GetCookies::default())?.cookies;
let nav     = client.call(Some(&session), &Navigate::to("https://example.com/"))?;

let _ = client.detach(&session);
```

## Design

* **One socket per browser.** `Target.attachToTarget {flatten: true}`
  returns a `SessionId`; subsequent commands take it as an argument and
  are routed by the browser to the right target. Per-tab-connect models
  drop events and serialise operations that should run in parallel.
* **Sync transport.** [`tungstenite`] for WebSocket, [`ureq`] for the
  `/json`, `/json/version`, `/json/list` HTTP endpoints. No async runtime.
* **Typed commands.** Each CDP method is a struct implementing a
  [`Command`] trait:
  ```rust
  pub trait Command: Serialize {
      const METHOD: &'static str;
      type Return: DeserializeOwned;
  }
  ```
  `Client::call(session, &Cmd)` sends the params (`Serialize`), waits for
  the response by `id`, deserialises the return, and hands you a typed
  result. Events received during the wait are queued and delivered via
  `Client::next_event`.
* **No code-gen.** Domain modules are hand-written and cover only what
  consumers actually use — currently `target`, `network`, `storage`,
  `runtime`, `page`, `fetch`, `input`, `accessibility`, `browser`
  (~30 commands). Add on demand; wire-format tests keep every command
  aligned with the canonical schema (`browser_protocol.json`, ToT 1.3).

[`tungstenite`]: https://crates.io/crates/tungstenite
[`ureq`]: https://crates.io/crates/ureq

## How does CDP get opened?

CDP-Ninja is intentionally the client only. You need CDP already open on
the target port. Options:

* Launch the browser yourself with `--remote-debugging-port=8181`.
* Use a companion tool that enables CDP on a running browser at runtime.
  Public references for that primitive: SpecterOps'
  [CDP-Enable-BOF](https://github.com/KingOfTheNOPs/CDP-Enable-BOF).

## History

Renamed from `cdp-core` on 2026-08-18 as the crate outgrew being just a
transport library — it now ships a CLI, is designed as the operator-facing
CDP framework, and works as a general-purpose client for anyone who wants
to talk CDP from Rust. The old repo URL (github.com/shxve/cdp-core) 301s
to this one, and consumers pinned to it via a Cargo git dep still resolve
via GitHub's redirect.

## License

MIT.
