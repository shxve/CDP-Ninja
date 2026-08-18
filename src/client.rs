//! Single-socket CDP client with session multiplexing.
//!
//! One [`Transport`] is shared across the browser session and any number of
//! target sessions attached via `Target.attachToTarget {flatten: true}`.
//! Response dispatch is by `(id, session_id)`; unrelated frames received
//! during a wait are queued and returned on the next matching call.

use serde_json::Value;
use std::collections::VecDeque;

use crate::message::{Event, Frame, Request};
use crate::transport::Transport;
use crate::{CdpError, Result};

/// A per-target session id returned by [`Client::attach_target`]. Passed back
/// to [`Client::send`] as `Some(SessionId(...))` to route commands to that
/// target instead of the browser session.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(pub String);

impl SessionId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct Client {
    transport: Transport,
    next_id: u64,
    /// Frames received during a `wait_for` that didn't match — held for the
    /// next call so no events are dropped on the floor.
    pending_events: VecDeque<Event>,
}

impl Client {
    /// Wrap an existing [`Transport`]. Prefer [`Self::connect_browser`] to
    /// discover + connect to the browser session in one call.
    pub fn new(transport: Transport) -> Self {
        Self {
            transport,
            next_id: 1,
            pending_events: VecDeque::new(),
        }
    }

    /// Discover the browser WebSocket URL via `/json/version` at
    /// `127.0.0.1:port` and connect. This is the browser-scoped session that
    /// accepts `Target.*` methods.
    pub fn connect_browser(port: u16) -> Result<Self> {
        let ws_url = crate::discovery::browser_ws_url(port)?;
        let transport = Transport::connect(&ws_url)?;
        Ok(Self::new(transport))
    }

    /// Send a command and block until its response arrives. Events received in
    /// the meantime are queued and delivered via [`Self::next_event`].
    pub fn send(
        &mut self,
        session: Option<&SessionId>,
        method: &str,
        params: Value,
    ) -> Result<Value> {
        let id = self.next_id;
        self.next_id += 1;

        let request = Request {
            id,
            session_id: session.map(|s| s.as_str()),
            method,
            params,
        };
        let text = serde_json::to_string(&request)
            .map_err(|e| CdpError::Parse(format!("serialize request: {e}")))?;
        self.transport.send_text(text)?;

        loop {
            let Some(text) = self.transport.read_frame()? else {
                continue;
            };
            let frame = Frame::parse(&text)
                .map_err(|e| CdpError::Parse(format!("deserialize frame: {e}")))?;

            match (frame.id, frame.method.clone()) {
                // Matched response.
                (Some(fid), _) if fid == id => {
                    if let Some(err) = frame.error {
                        return Err(CdpError::Command {
                            code: err.code,
                            message: err.message,
                        });
                    }
                    return frame
                        .result
                        .ok_or_else(|| CdpError::Parse("response missing result".into()));
                }
                // Someone else's response — drop; we don't multiplex across
                // callers in the sync API.
                (Some(_), _) => continue,
                // Event — queue for later delivery.
                (None, Some(m)) => {
                    self.pending_events.push_back(Event {
                        method: m,
                        session_id: frame.session_id,
                        params: frame.params.unwrap_or(Value::Null),
                    });
                }
                _ => continue,
            }
        }
    }

    /// Return the next queued event, if any. Does not block.
    pub fn next_event(&mut self) -> Option<Event> {
        self.pending_events.pop_front()
    }

    /// Convenience for the flatten-session attach pattern. Sends
    /// `Target.attachToTarget {targetId, flatten: true}` and returns the
    /// assigned session id.
    pub fn attach_target(&mut self, target_id: &str) -> Result<SessionId> {
        let result = self.send(
            None,
            "Target.attachToTarget",
            serde_json::json!({ "targetId": target_id, "flatten": true }),
        )?;
        let sid = result
            .get("sessionId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| CdpError::Parse("attachToTarget response missing sessionId".into()))?;
        Ok(SessionId(sid.to_string()))
    }

    /// Send `Target.detachFromTarget {sessionId}`.
    pub fn detach(&mut self, session: &SessionId) -> Result<()> {
        self.send(
            None,
            "Target.detachFromTarget",
            serde_json::json!({ "sessionId": session.as_str() }),
        )?;
        Ok(())
    }

    /// Close the underlying transport.
    pub fn close(mut self) -> Result<()> {
        self.transport.close()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_id_roundtrip() {
        let s = SessionId("abc".to_string());
        assert_eq!(s.as_str(), "abc");
    }
}
