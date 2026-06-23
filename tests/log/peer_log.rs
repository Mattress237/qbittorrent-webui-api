use crate::login_default_client;

/// Test that the API Workes and it correctly deserializes log items from
/// the response.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn get_logs() {
    let client = login_default_client().await;

    let response = client.peer_log(None).await;
    assert!(response.is_ok());

    let logs = response.unwrap();
    assert!(!logs.is_empty());
}

/// Test that peer logs can be retrieved with a last ID.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn get_logs_withe_last_id() {
    let client = login_default_client().await;

    let id = client.peer_log(None).await.unwrap().first().unwrap().id;

    let response = client.peer_log(Some(id)).await;
    assert!(response.is_ok());
}
