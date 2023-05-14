use std::{net::TcpListener, fmt::format, assert_eq};


#[tokio::test]
async fn health_check(){
    // Arrange
    let address = format!("{}/health_check",spawn_app());

    let client = reqwest::Client::new();

    
    //Act
    let response = client.get(address).send().await.expect("Failed to excute test");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data(){
    //Arrang
    let app_address = format!("{}/subscriptions",spawn_app());
    let client = reqwest::Client::new();

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

}


#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing(){
    //Arrang
    let app_address = format!("{}/subscriptions",spawn_app());
    let client = reqwest::Client::new(); 
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
    assert_eq!(200,response.status().as_u16());
    }

}


 fn spawn_app()->String{
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failde to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zeroProdRust::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}",port)
}
    

