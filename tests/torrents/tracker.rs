use crate::{DEBIAN_HASH, add_debian_torrent, login_default_client};

/// This test ensures that the API correctly deserialize the tracker response.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn correctly_deserialize_from_response() {
    let client = login_default_client().await;
    add_debian_torrent(&client).await;

    let trackers = client.trackers(DEBIAN_HASH).await.unwrap();
    assert!(trackers.len() > 0);
}
