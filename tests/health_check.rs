use std::{net::TcpListener, assert_eq};
// use zeroProdRust::configuration::DataBaseSettings;

use sqlx::{PgPool, PgConnection, Connection, Executor};
use uuid::Uuid;
use zeroProdRust::{configuration::{get_configurations,DataBaseSettings}, telemetry::{get_subscriber, init_subscriber}};
// Ensure that the `tracing` stack is only initialised once using `once_cell`”
use once_cell::sync::Lazy;



pub struct TestApp{
    pub address:String,
    pub db_pool: PgPool
} 

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".into();
    let subscriber_name = "test".into();

    if std::env::var("TEST_LOG").is_ok(){
        let subscriber = get_subscriber(subscriber_name,default_filter_level,std::io::stdout);
    }else{

        let subscriber = get_subscriber(subscriber_name,default_filter_level,std::io::sink);
    }
   
    init_subscriber(subscriber);
});


#[tokio::test]
async fn health_check(){

    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());

}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data(){
    //Arrange
    let app = spawn_app().await;
    // let configuration = get_configurations().expect("failed to read config");
    // let connection_string = configuration.database.connection_string();
    let client = reqwest::Client::new();
    let app_address = format!("{}/subscriptions",&app.address);

    //Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client.post(app_address)
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failded to excute request.");
 
   //Assert
   assert_eq!(200,response.status().as_u16());

   let saved = sqlx::query!("SELECT email, name FROM subscriptions",).fetch_one(&app.db_pool).await.expect("Failed to fetch");
   assert_eq!(saved.email, "ursula_le_guin@gmail.com");
   assert_eq!(saved.name, "le guin");
}


#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing(){
    //Arrang
    let app: TestApp = spawn_app().await;
    let client = reqwest::Client::new(); 
    let app_address = format!("{}/subscriptions",&app.address);
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for(invalid_body,error_msg) in test_cases{
    //Act
    let response = client.post(&app_address)
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(invalid_body)
    .send()
    .await
    .expect("Failded to excute request.");

    //Assert
    assert_eq!(400,response.status().as_u16(),
    "The API did not fail with 400 Bad Request when the payload was {}",
    error_msg);
    }

}


 async fn spawn_app()->TestApp{
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failde to bind random port");
    let port = listener.local_addr().unwrap().port();
    let mut configuration = get_configurations().expect("Failed to read configuration.");
    configuration.database.database_name =  Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let server = zeroProdRust::startup::run(listener,connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp { address: format!("http://127.0.0.1:{}",port), db_pool:connection_pool  }
}
    

pub async fn configure_database(config: &DataBaseSettings) -> PgPool {

    //Only once it will excute and all the other time it will be skipped.
    Lazy::force(&TRACING);

    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    
         connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

        sqlx::migrate!("./scripts/migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}