use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, dev::Server};

//impl Responder trait meaning we are returing any type that returing Responder trait
//Question:: How come rust knows format implementing Responder trait?
//“Responder is nothing more than a conversion trait into a HttpResponse.”
//any type that implement Responder should return this function
//Since HttpResponse impl this trait we can return HttpRespone
async fn health_checker(req: HttpRequest) -> impl Responder{
    HttpResponse::Ok()
}


pub fn run(address: &str) -> Result<Server,std::io::Error> {
    //HttpServer handle all transport level concerns
     let server = HttpServer::new(|| {
        App::new()
        .route("/health_check", web::get().to(health_checker))
    })
    .bind(address)?
    .run();
    Ok(server)
}