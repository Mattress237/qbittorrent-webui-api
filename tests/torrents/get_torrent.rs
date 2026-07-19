use qbit::models::TorrentProperties;

use crate::{DEBIAN_HASH, add_debian_torrent, login_default_client};

/// This test ensures that the API correctly deserialize the torrent the response.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn correctly_deserialize_from_response() {
    let client = login_default_client().await;
    add_debian_torrent(&client).await;

    let torrent: TorrentProperties = client
        .torrent(DEBIAN_HASH)
        .await
        .expect("Failed to fetch main data: ");

    assert_eq!(torrent.hash, DEBIAN_HASH);
}
