use std::net::TcpListener;
use actix_web::{web, App,HttpServer, dev::Server};



pub fn run(listener: TcpListener) -> Result<Server,std::io::Error> {
    //HttpServer handle all transport level concerns
     let server = HttpServer::new(|| {
        App::new()
        .route("/health_check", web::get().to(crate::routes::health_checker))
        .route("/subscriptions", web::post().to(crate::routes::subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

