
use std::net::TcpListener;

use sqlx::{PgConnection, Connection, PgPool};
use zeroProdRust::startup::run;

use zeroProdRust::configuration::{get_configurations};



#[tokio::main]
async fn main() -> std::io::Result<()> {

    let configuration = get_configurations().expect("Failed to Read config");

    // let connection = PgConnection::connect(&configuration.database.connection_string()).await.expect("Failed to connect postgress");

    let address = format!("127.0.0.1:{}",configuration.application_port);

    let connection_pool = PgPool::connect(&configuration.database.connection_string()).await.expect("Failed to connect postgress");

//   run()?.await
    let listener = TcpListener::bind(address).unwrap();
    let server = match run(listener,connection_pool) {
        Ok(_)=>Ok(()),
        Err(e)=>return Err(e)
    };
    return server;
}
// https://stackoverflow.com/questions/36653678/angular2-input-to-a-property-with-get-set