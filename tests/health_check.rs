
#[tokio::test]
async fn health_check(){
    // Arrange
    spawn_app().await.expect("Failed to spawn our app.");

    let client = reqwest::Client::new();

    //Act
    let response = client.get("http://127.0.0.1:8000/health_check").send().await.expect("Failed to excute test");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app()->std::io::Result<()>{
 zeroProdRust::run()?.await
}
    