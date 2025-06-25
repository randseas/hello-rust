// main.rs
use db;

#[tokio::main]
async fn main() {
    match connect().await {
        Ok(client) => {
            // Bağlantı başarılı, client ile işlem yapabilirsin
        }
        Err(e) => {
            eprintln!("Bağlantı hatası: {}", e);
        }
    }
}
