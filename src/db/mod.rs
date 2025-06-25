use mongodb::{ Client, options::ClientOptions };
use std::error::Error;

pub async fn connect() -> Result<Client, Box<dyn Error>> {
    let client_uri = "mongodb://localhost:27017";
    let mut client_options = ClientOptions::parse(client_uri).await?;
    client_options.app_name = Some("RustMongoApp".to_string());
    let client = Client::with_options(client_options)?;
    client.database("admin").run_command(doc! { "ping": 1 }, None).await?;
    println!("MongoDB bağlantısı başarılı.");
    Ok(client)
}