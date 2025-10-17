use crate::{create_dummy_torrent, create_random_name, login_default_client};

#[tokio::test]
#[ignore = "Test hits API endpoint"]
pub async fn create_empty_tags() {
    let client = login_default_client().await;
    let result = client.create_tags(vec![""]).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore = "Test hits API endpoint"]
pub async fn create_random_tags() {
    let client = login_default_client().await;
    let tags = vec![
        create_random_name().unwrap(),
        create_random_name().unwrap(),
        create_random_name().unwrap(),
    ];

    let result = client
        .create_tags(tags.iter().map(|t| t.as_ref()).collect())
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore = "Test hits API endpoint"]
pub async fn create_dummy_with_tags() {
    let client = login_default_client().await;
    let task = create_dummy_torrent(&client, create_random_name())
        .await
        .unwrap();
    let task_as_torrent = client.list_tasks().await.unwrap();
}
