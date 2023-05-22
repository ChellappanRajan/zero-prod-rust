use std::print;

use actix_web::{HttpRequest, Responder, HttpResponse, web};
use sqlx::{PgPool};
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData{
  email:String,
  name:String
}

pub async fn subscribe(
  form:web::Form<FormData>,
  //Retrieving connection from app state
  pool:web::Data<PgPool>
)->HttpResponse{
  let request_id =  Uuid::new_v4();
  let request_span = tracing::info_span!(
    "Adding new Subscriber",
    %request_id,
    subscriber_email = %form.email,
    subscriber_name = %form.name
  );

  let request_guard = request_span.enter();
  // tracing::info!("request_id {}- Adding '{}' '{}' as a new subscriber.",request_id,form.email,form.name);
  // tracing::info!("Saving new subscriber details in the database");
  match sqlx::query!(
    r#"
    INSERT INTO subscriptions (id,email,name,subscribed_at)
    VALUES ($1,$2,$3,$4)
    "#,
    Uuid::new_v4(),
    form.email,
    form.name,
    Utc::now()
  ).execute(pool.get_ref()).await{
    Ok(_)=> {
      tracing::info!("request_id {} - New subscribe have been saved",request_id);
      HttpResponse::Ok().finish()
    },
    Err(e)=>{
      tracing::info!("request_id {}, Failed to excute query,{:?}",request_id,e);
      HttpResponse::InternalServerError().finish()
    }
  }
}