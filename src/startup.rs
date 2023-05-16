use std::net::TcpListener;
use actix_web::{web, App,HttpServer, dev::Server};
use sqlx::PgConnection;



pub fn run(listener: TcpListener,connection:PgConnection) -> Result<Server,std::io::Error> {

            //Extracter- Wrap the connection in a smart pointe
            let connection_pointer = web::Data::new(connection);

    //HttpServer handle all transport level concerns
     let server = HttpServer::new(move|| {
        App::new()
        .route("/health_check", web::get().to(crate::routes::health_checker))
        .route("/subscriptions", web::post().to(crate::routes::subscribe))
        .app_data(connection_pointer.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

