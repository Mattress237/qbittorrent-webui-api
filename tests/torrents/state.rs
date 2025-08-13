use qbit::parameters::TorrentState;

use crate::{add_debian_torrent, get_debian_torrent, login_default_client};

/// As the torrent is automatically stopped (in theory). We should confirm it's stopped.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn torrent_is_stopped() {
    let client = login_default_client().await;
    add_debian_torrent(&client).await;

    // This is a required delay (and also a kinda sub test) for when this test runs before the other tests.
    let mut torrent_state = TorrentState::default();
    while torrent_state.is_checking()
        || torrent_state == TorrentState::MetadataDownloading
        || torrent_state == TorrentState::Unknown
        || torrent_state.is_queued()
    {
        torrent_state = get_debian_torrent(&client).await.unwrap().state;
        println!("{:?}", get_debian_torrent(&client).await.unwrap().state);
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    println!("{:?}", get_debian_torrent(&client).await.unwrap().state);

    assert!(
        get_debian_torrent(&client)
            .await
            .unwrap()
            .state
            .is_stopped()
    )
}
