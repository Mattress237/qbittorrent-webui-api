use crate::{DEBIAN_HASH, add_debian_torrent, login_default_client};

#[tokio::test]
#[ignore = "Test hits api endpoint"]
// NOTE: This test dont hit and active running torrent. The torrent is not running.
pub async fn can_get_peers_data() {
    let client = login_default_client().await;

    let hash = DEBIAN_HASH;
    add_debian_torrent(&client).await;

    let result = client.peers_data(hash, None).await;
    println!("{:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore = "Test hits api endpoint"]
// NOTE: This test dont hit and active running torrent. The torrent is not running.
pub async fn can_get_peers_data_with_rid() {
    let client = login_default_client().await;

    let hash = DEBIAN_HASH;
    add_debian_torrent(&client).await;

    let result = client.peers_data(hash, None).await;
    let result = client.peers_data(hash, Some(result.unwrap().rid)).await;
    println!("{:?}", result);
    assert!(result.is_ok());
}
