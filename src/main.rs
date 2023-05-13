
use zeroProdRust::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
//   run()?.await
    let server = match run("127.0.0.1:8000") {
        Ok(_)=>Ok(()),
        Err(e)=>return Err(e)
    };
    return server;
}
// https://stackoverflow.com/questions/36653678/angular2-input-to-a-property-with-get-set