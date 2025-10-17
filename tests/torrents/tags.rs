use crate::{create_and_get_torrent, create_random_name, login_default_client};

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

    let result = client
        .create_tags(vec![
            &create_random_name().unwrap(),
            &create_random_name().unwrap(),
            &create_random_name().unwrap(),
        ])
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
// #[ignore = "Test hits API endpoint"]
pub async fn create_dummy_with_tags() {
    let client = login_default_client().await;
    let random = create_random_name();
    let torrent = create_and_get_torrent(&client, random).await;

    client
        .add_tags(
            Some(vec![&torrent.hash]),
            vec![&create_random_name().unwrap()],
        )
        .await
        .unwrap();
}
