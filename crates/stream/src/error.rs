use tokio_tungstenite::tungstenite;

#[derive(Debug)]
#[allow(dead_code)]
pub enum SocketError {
    Socket(tungstenite::Error),
    Extract(serde_json::Error),
}

impl From<serde_json::Error> for SocketError {
    fn from(error: serde_json::Error) -> Self {
        Self::Extract(error)
    }
}

impl From<tungstenite::Error> for SocketError {
    fn from(error: tungstenite::Error) -> Self {
        Self::Socket(error)
    }
}
