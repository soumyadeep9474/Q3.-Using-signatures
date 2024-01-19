use tokio::sync::mpsc::UnboundedReceiver;

pub async fn run_aggregator(mut receiver: UnboundedReceiver<(f64, String)>, num_clients: usize) {
    let mut total_average = 0.0;
    let mut count = 0;

    // Receive values from all clients
    while let Some((average_price, signature)) = receiver.recv().await {
        // Validate the signature before processing the message
        if crate::signature::verify_signature(&average_price.to_string(), &signature) {
            total_average += average_price;
            count += 1;
        } else {
            eprintln!("Error: Invalid signature for a client's contribution");
        }

        // Exit loop when all clients have sent their values
        if count == num_clients {
            break;
        }
    }

    // Calculate the final average of valid average prices
    let overall_average = if count > 0 {
        total_average / count as f64
    } else {
        0.0
    };

    println!("Aggregator complete. The overall average USD price of BTC is: {}", overall_average);
}
