
use std::net::TcpListener;
use sqlx::{PgPool};
use zeroProdRust::startup::run;
use env_logger::Env;
use zeroProdRust::configuration::{get_configurations};



#[tokio::main]
async fn main() -> std::io::Result<()> {

    //Override rust global default log  
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configurations().expect("Failed to Read config");

    // let connection = PgConnection::connect(&configuration.database.connection_string()).await.expect("Failed to connect postgress");

    let address = format!("127.0.0.1:{}",configuration.application_port);

    let connection_pool = PgPool::connect(&configuration.database.connection_string()).await.expect("Failed to connect postgress");
    let listener = TcpListener::bind(address).unwrap();
    run(listener,connection_pool)?.await
    // let server = match run(listener,connection_pool) {
    //     Ok(_)=>Ok(()),
    //     Err(e)=>return Err(e)
    // };
    // return server;
}
// https://stackoverflow.com/questions/36653678/angular2-input-to-a-property-with-get-set