use bincode::Error;

use mongodb::{options::ClientOptions, Client};
async fn test() -> Result<(), failure::Error> {
    println!("test");

    let user = std::env::var("MONGOUSER")?;
    let pw = std::env::var("MONGOPW")?;

    let server = "127.0.0.1";
    let port = 27017;
    let m = format!("mongodb://{}:{}@{}:{}", user, pw, server, port);
    let mut client_options = ClientOptions::parse(m).await?;

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await? {
        println!("db={}", db_name);
    }

    return Ok(());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test().await.expect("test");
    Ok(())
}
