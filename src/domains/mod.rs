//! Typed CDP method bindings, grouped by protocol domain.
//!
//! Each submodule contains one `Command`-implementing request struct per
//! method plus its corresponding response struct. Field names and types are
//! kept aligned with the canonical CDP schema
//! (`browser_protocol.json` + `js_protocol.json`, ToT 1.3 at time of writing).
//!
//! Domain coverage is added on demand — we keep only what consumers actually
//! use rather than generating the full 52-domain surface.

pub mod accessibility;
pub mod browser;
pub mod fetch;
pub mod input;
pub mod network;
pub mod page;
pub mod runtime;
pub mod storage;
pub mod target;
