mod client;
mod server;

use crate::server::start_server;
use crate::client::start_client;

#[tokio::main(flavor = "multi_thread", worker_threads = 100)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let t = tokio::spawn(async { start_server().await;});
    let t1 = tokio::spawn(async { start_client().await});
    t.await.unwrap();
    t1.await.unwrap();
    Ok(())
}