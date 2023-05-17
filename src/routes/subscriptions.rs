use actix_web::{HttpRequest, Responder, HttpResponse, web};
use sqlx::PgConnection;
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
struct FormData{
   name:String,
   email:String
}

pub async fn subscribe(
  _form:web::Form<FormData>,
  //Retrieving connection from app state
  _connection:web::Data<PgConnection>
)->HttpResponse{
  sqlx::query!(
    r#"
    INSERT INTO subscriptions (id,email,name,subscribed_at)
    VALUES ($1,$2,$3,$4)
    "#,
    Uuid::new_v4(),
    _form.email,
    _form.name,
    Utc::now()
  ).excute(_connection.get_ref()).await;
  
  HttpResponse::Ok().finish()
}