
use std::net::TcpListener;
use sqlx::{PgPool};
use tracing_log::LogTracer;
use zeroProdRust::startup::run;
use env_logger::Env;
use zeroProdRust::configuration::{get_configurations};
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer,JsonStorageLayer};
use tracing_subscriber::{Layer::SubscriberExt,  EnvFilter,Registry, prelude::__tracing_subscriber_SubscriberExt};



#[tokio::main]
async fn main() -> std::io::Result<()> {


    LogTracer::init().expect("Failed to set logger");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(
        |_| EnvFilter::new("info")
    );

    let formatting_layer = BunyanFormattingLayer::new("zeroToProd".into(),std::io::stdout);


    let subscriber = Registry::default().with(env_filter).with(JsonStorageLayer).with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber");

    //Override rust global default log  
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

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