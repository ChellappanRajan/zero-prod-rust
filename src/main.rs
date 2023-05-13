
use std::net::TcpListener;

use zeroProdRust::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
//   run()?.await
    let listener = TcpListener::bind("127.0.0.1").unwrap();
    let server = match run(listener) {
        Ok(_)=>Ok(()),
        Err(e)=>return Err(e)
    };
    return server;
}
// https://stackoverflow.com/questions/36653678/angular2-input-to-a-property-with-get-set