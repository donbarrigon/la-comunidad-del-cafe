use ironforge::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = Server::new("0.0.0.0", 3000);
    server.listen().await?;
    Ok(())
}