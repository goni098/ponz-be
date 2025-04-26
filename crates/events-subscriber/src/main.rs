use futures_util::StreamExt;

use shared::{AppResult, env::ENV};

#[tokio::main]
async fn main() -> AppResult<()> {
    let client = redis::Client::open(ENV.redis_url.as_str())?;

    let (mut sink, mut stream) = client.get_async_pubsub().await?.split();

    sink.subscribe("event_contract_channel").await?;

    while let Some(message) = stream.next().await {
        dbg!(message);
    }

    Ok(())
}
