//! Blocking WebSocket transport wrapper.
//!
//! Owns a `tungstenite::WebSocket<TcpStream>` and provides send/receive of raw
//! text frames. [`crate::client::Client`] sits on top and does multiplexing +
//! response dispatch.

use std::net::TcpStream;
use std::time::Duration;

use tungstenite::{client, Message, WebSocket};

use crate::{CdpError, Result};

pub struct Transport {
    socket: WebSocket<TcpStream>,
}

impl Transport {
    /// Connect to a CDP WebSocket endpoint (`ws://host:port/devtools/...`).
    /// Read timeout defaults to 30 seconds; caller can override via
    /// [`Self::set_read_timeout`] afterwards.
    pub fn connect(ws_url: &str) -> Result<Self> {
        let addr = extract_host_port(ws_url)?;
        let tcp = TcpStream::connect(&addr)
            .map_err(|e| CdpError::Transport(format!("tcp connect {addr}: {e}")))?;
        tcp.set_read_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| CdpError::Transport(e.to_string()))?;

        let (socket, _) = client(ws_url, tcp)
            .map_err(|e| CdpError::Transport(format!("ws handshake: {e}")))?;

        Ok(Self { socket })
    }

    /// Override the read timeout of the underlying TCP socket.
    pub fn set_read_timeout(&mut self, dur: Option<Duration>) -> Result<()> {
        self.socket
            .get_ref()
            .set_read_timeout(dur)
            .map_err(|e| CdpError::Transport(e.to_string()))
    }

    /// Send a UTF-8 text frame.
    pub fn send_text(&mut self, text: String) -> Result<()> {
        self.socket
            .send(Message::Text(text))
            .map_err(|e| CdpError::Transport(format!("send: {e}")))
    }

    /// Read the next frame. Returns `Ok(Some(text))` for a text frame, `Ok(None)`
    /// for a non-text frame the caller should skip (ping/pong/binary), and
    /// `Err(Closed)` when the peer closed the socket.
    pub fn read_frame(&mut self) -> Result<Option<String>> {
        match self.socket.read() {
            Ok(Message::Text(t)) => Ok(Some(t)),
            Ok(Message::Close(_)) => Err(CdpError::Closed),
            Ok(_) => Ok(None),
            Err(tungstenite::Error::ConnectionClosed) => Err(CdpError::Closed),
            Err(tungstenite::Error::AlreadyClosed) => Err(CdpError::Closed),
            Err(e) => Err(CdpError::Transport(format!("read: {e}"))),
        }
    }

    /// Best-effort clean close.
    pub fn close(&mut self) -> Result<()> {
        self.socket
            .close(None)
            .map_err(|e| CdpError::Transport(format!("close: {e}")))
    }
}

fn extract_host_port(ws_url: &str) -> Result<String> {
    let stripped = ws_url
        .strip_prefix("ws://")
        .or_else(|| ws_url.strip_prefix("wss://"))
        .ok_or_else(|| CdpError::Transport(format!("invalid ws url: {ws_url}")))?;
    stripped
        .split('/')
        .next()
        .filter(|s| !s.is_empty())
        .map(String::from)
        .ok_or_else(|| CdpError::Transport(format!("no host in ws url: {ws_url}")))
}

#[cfg(test)]
mod tests {
    use super::extract_host_port;

    #[test]
    fn extract_from_ws_url() {
        assert_eq!(
            extract_host_port("ws://127.0.0.1:8181/devtools/browser/abc").unwrap(),
            "127.0.0.1:8181"
        );
        assert_eq!(
            extract_host_port("wss://example.com/devtools").unwrap(),
            "example.com"
        );
    }

    #[test]
    fn reject_non_ws() {
        assert!(extract_host_port("http://x/y").is_err());
    }
}
