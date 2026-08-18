# cdp-core

Sync-first Rust client for the [Chrome DevTools Protocol](https://chromedevtools.github.io/devtools-protocol/).

Small, no code-gen, no `tokio`. Owns one WebSocket per browser session and
multiplexes per-target sessions across it via CDP's flat-session model.
Typed method bindings for the common post-exploitation surface, grounded in
the canonical protocol schema.

## Design

* **One socket per browser.** `Target.attachToTarget {flatten: true}`
  returns a `SessionId`; subsequent commands take it as an argument and
  are routed by the browser to the right target. Per-tab-connect models
  drop events and serialise operations that should run in parallel.
* **Sync transport.** [`tungstenite`] for WebSocket, [`ureq`] for the
  `/json`, `/json/version`, `/json/list` HTTP endpoints. Reaches into
  neither tokio nor async-std; drops straight into a synchronous CLI or a
  worker thread.
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
  aligned with the canonical schema.

[`tungstenite`]: https://crates.io/crates/tungstenite
[`ureq`]: https://crates.io/crates/ureq

## Quick tour

```rust
use cdp_core::{Client, domains::{storage::GetCookies, page::Navigate}};

let mut client = Client::connect_browser(8181)?;
let session = client.attach_target("target-id-here")?;

let cookies = client.call(None, &GetCookies::default())?.cookies;
let nav     = client.call(Some(&session), &Page::Navigate::to("https://example.com/"))?;

let _ = client.detach(&session);
```

For a fuller runnable example, see the [CDP-Enabler](https://github.com/shxve/CDP-Enabler)
CLI, built on top of this crate.

## Non-goals

* Not a Puppeteer replacement. No headless-Chromium orchestration, no
  page-lifecycle abstractions, no BiDi. If you want that, look at
  [`chromiumoxide`](https://github.com/mattsse/chromiumoxide) or
  [`headless_chrome`](https://github.com/rust-headless-chrome/rust-headless-chrome).
* Not a CDP-enable primitive. Point cdp-core at a browser that's already
  running with CDP open (`--remote-debugging-port=…` or an injection tool
  that opened the port).
* No async runtime. That's the point.

## License

MIT.
