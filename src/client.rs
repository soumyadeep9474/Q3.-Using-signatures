use tokio::time::{sleep, Duration};
use tokio_tungstenite::WebSocketStream;
use futures_util::stream::StreamExt;
use tokio_tungstenite::connect_async;
use serde::Deserialize;
use crate::signature::{sign_message, verify_signature};

#[derive(Debug, Deserialize)]
struct TickerPrice {
    symbol: String,
    price: String,
    signature: String,
}

pub async fn run_client(client_id: usize, sender: tokio::sync::mpsc::UnboundedSender<(f64, String)>) {
    let api_url = "wss://stream.binance.com:9443/ws/btcusdt@trade";

    if let Ok(mut ws_stream) = connect_to_binance_ws(api_url).await {
        let start_time = std::time::Instant::now();
        let mut total_price = 0.0;
        let mut count = 0;

        while start_time.elapsed().as_secs() < 10 {
            if let Some(Ok(msg)) = ws_stream.next().await {
                if let Ok(response) = serde_json::from_str::<TickerPrice>(&msg.to_text().unwrap()) {
                    if response.symbol == "BTCUSDT" {
                        if let Ok(price) = response.price.parse::<f64>() {
                            // Verify the signature before processing the message
                            if verify_signature(&msg.to_text().unwrap(), &response.signature) {
                                total_price += price;
                                count += 1;
                            } else {
                                eprintln!("Error: Invalid signature for client {}", client_id);
                            }
                        }
                    }
                }
            }
            sleep(Duration::from_millis(100)).await;
        }

        // Calculate the average for the client
        let average_price = if count > 0 {
            total_price / count as f64
        } else {
            0.0
        };

        // Sign the average price before sending to the aggregator
        let signature = sign_message(&average_price.to_string());

        // Send the average price and signature to the aggregator
        sender.send((average_price, signature)).unwrap();
    } else {
        eprintln!("Error: Failed to establish WebSocket connection for client {}", client_id);
    }
}

async fn connect_to_binance_ws(api_url: &str) -> Result<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::Error> {
    let (ws_stream, _) = connect_async(api_url).await?;
    Ok(ws_stream)
}
