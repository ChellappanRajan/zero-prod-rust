
use std::net::TcpListener;

use zeroProdRust::startup::run;

use zeroProdRust::configuration::{get_configurations};


#[tokio::main]
async fn main() -> std::io::Result<()> {

    let configuration = get_configurations().expect("Failed to Read config");

    let address = format!("127.0.0.1:{}",configuration.application_port);
    
//   run()?.await
    let listener = TcpListener::bind(address).unwrap();
    let server = match run(listener) {
        Ok(_)=>Ok(()),
        Err(e)=>return Err(e)
    };
    return server;
}
// https://stackoverflow.com/questions/36653678/angular2-input-to-a-property-with-get-set