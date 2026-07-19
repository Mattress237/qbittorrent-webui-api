use crate::{DEBIAN_HASH, add_debian_torrent, login_default_client};

/// This test ensures that the API correctly deserialize the webseed response.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn correctly_deserialize_from_response() {
    let client = login_default_client().await;
    add_debian_torrent(&client).await;

    let web_seeds = client.webseeds(DEBIAN_HASH).await.unwrap();
    assert!(web_seeds.len() > 0);
}
