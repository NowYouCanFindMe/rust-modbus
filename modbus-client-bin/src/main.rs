use modbus_core::client::run_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_client("192.168.0.1", 503).await
}
