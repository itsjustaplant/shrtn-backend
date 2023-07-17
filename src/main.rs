mod db;
mod endpoints;
mod models;
mod server;

use futures::join;
use actix_web::main;

#[main]
async fn main() -> std::io::Result<()> {
    let db_future = db::test_db();
    let server_future = server::start_server();
    let _ = join!(db_future, server_future);

    Ok(())
}
