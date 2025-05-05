use futures_util::TryStreamExt;
use reqwest_eventsource::{Event, EventSource};
use tokio::sync::mpsc::Sender;
use tracing::warn;

use super::modules::ChatCompletionDelta;

pub async fn forward_deserialized_chat_response_stream(
    stream: EventSource,
    tx: Sender<ChatCompletionDelta>,
) -> anyhow::Result<()> {
    stream
        .try_for_each(async |event| {
            match event {
                Event::Message(event) => {
                    match serde_json::from_str::<ChatCompletionDelta>(&event.data) {
                        Ok(completion) => {
                            if tx.send(completion).await.is_err() {
                                warn!("Failed to send completion delta: channel closed");
                            }
                        }
                        Err(e) => {
                            warn!(
                                "Failed to deserialize ChatCompletionDelta from JSON data '{}': {}",
                                &event.data, e
                            );
                        }
                    }
                }
                _ => {}
            }
            Ok::<_, reqwest_eventsource::Error>(())
        })
        .await?;
    drop(tx);
    Ok(())
}
