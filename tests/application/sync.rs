use crate::login_default_client;

#[tokio::test]
#[ignore = "Test hits api endpoint"]
pub async fn can_get_main_data() {
    let client = login_default_client().await;

    let result = client.main_data(None).await;
    println!("{:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore = "Test hits api endpoint"]
pub async fn can_get_main_data_with_rid() {
    let client = login_default_client().await;

    let result = client.main_data(None).await;
    assert!(result.is_ok());

    let main_data = result.unwrap();
    let rid = main_data.rid;
    assert!(rid >= 0);

    let result = client.main_data(Some(rid)).await;
    println!("{:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore = "Test hits api endpoint"]
pub async fn main_data_can_serialized() {
    let client = login_default_client().await;

    let result = client.main_data(None).await;

    assert!(result.is_ok());
    let main_data = result.unwrap();

    let data = serde_json::to_string(&main_data);
    assert!(data.is_ok());
    let data = data.unwrap();
    assert!(data.contains("\"rid\":"));
    assert!(data.contains("\"full_update\":"));
    assert!(data.contains("\"categories\":"));
    assert!(data.contains("\"categories_removed\":"));
    assert!(data.contains("\"tags\":"));
    assert!(data.contains("\"tags_removed\":"));
    assert!(data.contains("\"torrents\":"));
    assert!(data.contains("\"torrents_removed\":"));
    assert!(data.contains("\"trackers\":"));
    assert!(data.contains("\"trackers_removed\":"));
    assert!(data.contains("\"server_state\":"));
}
