use bincode::Error;
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::FindOptions;
use mongodb::{options::ClientOptions, Client};
#[macro_use]
extern crate failure;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

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

    // Get a handle to a database.
    let db = client.database("my");

    // List the names of the collections in that database.
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }
    let collection = db.collection::<Document>("my");

    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];

    // Insert some documents into the "mydb.books" collection.
    collection.insert_many(docs, None).await?;

    let filter = doc! { "author": "George Orwell" };
    let find_options = FindOptions::builder()
        .sort(doc! { "title": 1 })
        .limit(5)
        .build();
    let mut cursor = collection.find(filter, find_options).await?;

    // Iterate over the results of the cursor.
    while let Some(book) = cursor.try_next().await? {
        println!(
            "title: {}",
            book.get("title").ok_or(format_err!("title error"))?
        );
    }
    return Ok(());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test().await.expect("test");
    Ok(())
}
