use actix_web::{HttpRequest, Responder, HttpResponse};

//impl Responder trait meaning we are returing any type that returing Responder trait
//Question:: How come rust knows format implementing Responder trait?
//“Responder is nothing more than a conversion trait into a HttpResponse.”
//any type that implement Responder should return this function
//Since HttpResponse impl this trait we can return HttpRespone
pub async fn health_checker(req: HttpRequest) -> impl Responder{
    HttpResponse::Ok()
}
