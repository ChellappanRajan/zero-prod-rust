use std::{net::TcpListener, fmt::format};


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

 fn spawn_app()->String{
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failde to bind random port");
    let port = listener.local_addr().unwrap().port();
 let server = zeroProdRust::run(listener).expect("Failed to bind address");
 let _ = tokio::spawn(server);
 format!("http://127.0.0.1:{}",port)
}
    