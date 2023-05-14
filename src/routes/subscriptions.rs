use actix_web::{HttpRequest, Responder, HttpResponse};

#[derive(serde::Deserialize)]
struct FormData{
   name:String,
   email:String
}

pub async fn subscribe()->HttpResponse{
  HttpResponse::Ok().finish()
}