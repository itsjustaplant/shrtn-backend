use mongodb::{
    Database, Collection,
    bson::{doc, Document},
    options::ClientOptions,
    Client,
    error::Error,
};

async fn get_client() -> Result<Client, Error> {
    let uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;

    Ok(client)
}

async fn get_db() -> Result<Database, Error> {
    let client = get_client().await?;
    let db = client.database("shrtnDB");

    Ok(db)
}

pub async fn get_collection() -> Result<Collection<Document>, Error> {
    let db = get_db().await?;
    let collection = db.collection("links");

    Ok(collection)
}

// test connection
pub async fn test_db() -> mongodb::error::Result<()> {
    let client = get_client().await?;

    client
        .database("shrtnDB")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    println!("Pinged DB");

    Ok(())
}
