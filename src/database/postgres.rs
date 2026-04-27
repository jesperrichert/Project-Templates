use actix_web::Error;
use tokio_postgres::{GenericClient, NoTls};

pub async fn postgres() -> Result<tokio_postgres::Client, Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=testy", NoTls).await.
        unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    // let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await;
    Ok(client)
}