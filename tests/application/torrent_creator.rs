use qbit::{
    models::{TorrentFormat, TorrentPieceSize},
    parameters::TorrentCreatorBuilder,
};

use crate::{create_random_name, create_test_data_dummy_folder, login_default_client};

const TORRENT_PREFIX: &str = "t-creator";

#[tokio::test]
#[ignore = "Test hits api endpoint"]
pub async fn can_create_torrent() {
    let client = login_default_client().await;
    let torrent_name = create_random_name(&format!("{}-1_", TORRENT_PREFIX));
    let (path, _) = create_test_data_dummy_folder(torrent_name.clone());
    let parameters = TorrentCreatorBuilder::default()
        .source_path(path)
        .build()
        .unwrap();

    println!("{:?}", parameters);

    let result = client.create_task(&parameters).await;
    assert!(result.is_ok());

    cleanup_torrent_and_task(&client, &torrent_name, result.unwrap()).await
}

#[tokio::test]
#[ignore = "Test hits api endpoint"]
pub async fn can_create_torrent_with_parameters() {
    let client = login_default_client().await;
    let torrent_name = create_random_name(&format!("{}-2_", TORRENT_PREFIX));
    let (path, _) = create_test_data_dummy_folder(torrent_name.clone());
    let parameters = TorrentCreatorBuilder::default()
        .source_path(path)
        .comment("test")
        .optimize_alignment(true)
        .piece_size(TorrentPieceSize::k16())
        .private(true)
        .source("test:source")
        .start_seeding(true)
        .format(TorrentFormat::Hybrid)
        .build()
        .unwrap();

    let result = client.create_task(&parameters).await;
    assert!(result.is_ok());

    cleanup_torrent_and_task(&client, &torrent_name, result.unwrap()).await
}

async fn cleanup_torrent_and_task(client: &crate::Api, torrent_name: &str, task_id: String) {
    client.delete_task(task_id).await.unwrap();
    let torrent = client
        .torrents(None)
        .await
        .unwrap()
        .into_iter()
        .find(|x| x.name == torrent_name);

    if let Some(t) = torrent {
        client.delete(vec![&t.hash], false).await.unwrap();
    }
}
