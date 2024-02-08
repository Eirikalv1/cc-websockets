pub mod run;
pub mod turtle_manager;

use run::run;

#[tokio::main]
pub async fn main() {
    run().await;
}

#[cfg(test)]
mod tests {
    use super::run;
    use futures_util::sink::SinkExt;
    use futures_util::StreamExt;
    #[tokio::test]
    async fn test_async() {
        tokio::spawn(run());
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        let (mut turtle1, _) =
            tokio_tungstenite::connect_async("ws://127.0.0.1:1234/ws?turtle_id=turtle1")
                .await
                .expect("Failed to connect");
        let (mut turtle2, _) =
            tokio_tungstenite::connect_async("ws://127.0.0.1:1234/ws?turtle_id=turtle2")
                .await
                .expect("Failed to connect");
        turtle1
            .send(tokio_tungstenite::tungstenite::Message::Text(
                "turtle2".to_string(),
            ))
            .await
            .unwrap();
        let a = turtle2.next().await.unwrap().unwrap();
        assert_eq!(a, "turtle1 sendte deg en melding!!!".into())
    }
}
