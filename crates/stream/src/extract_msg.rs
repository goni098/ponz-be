use alloy::rpc::types::Log;
use serde::Deserialize;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;

#[derive(Deserialize)]
struct InCommingLogResutMsg {
    params: LogResult,
}

#[derive(Deserialize)]
struct LogResult {
    result: Value,
}

pub fn extract(message: Message) -> Result<Option<Log>, serde_json::Error> {
    if let Message::Text(message) = message {
        let Some(incomming) = serde_json::from_str::<InCommingLogResutMsg>(&message).ok() else {
            return Ok(None);
        };
        let log: Log = serde_json::from_value(incomming.params.result)?;
        Ok(Some(log))
    } else {
        Ok(None)
    }
}
