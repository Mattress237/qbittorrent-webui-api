use qbit::models::Torrent;

use crate::{DEBIAN_HASH, add_debian_torrent, login_default_client};

/// This test ensures that the API correctly deserialize the torrents the response.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn correctly_deserialize_torrents_from_response() {
    let client = login_default_client().await;
    add_debian_torrent(&client).await;

    let torrents: Vec<Torrent> = client
        .torrents(None)
        .await
        .expect("Failed to fetch main data: ");

    assert!(!torrents.is_empty());
    assert!(torrents.into_iter().any(|x| x.hash == DEBIAN_HASH));
}
