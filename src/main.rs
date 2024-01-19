mod client;
mod aggregator;
mod signature;

use clap::{App, Arg};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let matches = App::new("use_signature")
        .arg(Arg::with_name("clients").long("clients").takes_value(true).required(true))
        .get_matches();

    let num_clients = matches.value_of("clients").unwrap_or("5").parse::<usize>().unwrap();

    let (sender, receiver) = mpsc::unbounded_channel();

    let mut handles = vec![];

    // Start client processes
    for i in 0..num_clients {
        let sender_clone = sender.clone();
        let handle = tokio::spawn(async move {
            client::run_client(i, sender_clone).await;
        });

        handles.push(handle);
    }

    // Wait for 10 seconds (time for clients to run)
    sleep(Duration::from_secs(10)).await;

    // Close the sender channel to signal aggregator to complete
    drop(sender);

    // Start aggregator process
    let aggregator_handle = tokio::spawn(async move {
        aggregator::run_aggregator(receiver, num_clients).await;
    });

    // Wait for all client processes to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Wait for the aggregator process to complete
    aggregator_handle.await.unwrap();
}
