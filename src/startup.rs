use std::net::TcpListener;
use actix_web::{web, App,HttpServer, dev::Server};
use sqlx::{PgConnection, PgPool};



pub fn run(listener: TcpListener,db_pool:PgPool) -> Result<Server,std::io::Error> {

    //HttpServer handle all transport level concerns
     let server = HttpServer::new(move|| {
        App::new()
        .route("/health_check", web::get().to(crate::routes::health_checker))
        .route("/subscriptions", web::post().to(crate::routes::subscribe))
        .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

