use qbit::{
    models::{TaskStatus, TorrentFormat, TorrentPieceSize},
    parameters::TorrentCreatorBuilder,
};

use crate::{
    create_dummy_torrent, create_random_name, create_test_data_dummy_folder, login_default_client,
};

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

#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn get_tasks() {
    let client = login_default_client().await;
    let torrent_name = create_random_name(&format!("{}-2_", TORRENT_PREFIX));
    create_dummy_torrent(&client, torrent_name.clone())
        .await
        .unwrap();

    let tasks = client.list_tasks().await.unwrap();
    assert!(!tasks.is_empty());

    println!("{:?}", tasks);
    for task in tasks.clone() {
        assert!(!task.task_id.is_empty());
        assert!(!task.source_path.is_empty());
        assert!(task.piece_size.0 > i32::MIN);
        assert!(
            task.status == TaskStatus::Failed
                || task.status == TaskStatus::Finished
                || task.status == TaskStatus::Queued
                || task.status == TaskStatus::Running
        );
        if task.status == TaskStatus::Failed {
            assert!(task.error_message.is_some());
        }
        if task.status == TaskStatus::Running {
            assert!(task.progress.is_some());
        }
    }

    cleanup_torrent_and_task(&client, &torrent_name, tasks[0].task_id.clone()).await;
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
