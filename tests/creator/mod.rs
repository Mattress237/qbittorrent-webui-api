mod create_torrents;

const TORRENT_PREFIX: &str = "t-creator";

async fn cleanup_torrent_and_task(client: &crate::Api, torrent_name: &str, task_id: String) {
    if !task_id.is_empty() {
        client.delete_task(task_id).await.unwrap();
    }
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

mod tasks {
    use qbit::models::TaskStatus;

    use super::*;

    use crate::{create_dummy_torrent, create_random_name, login_default_client};

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

        cleanup_torrent_and_task(&client, &torrent_name, "".to_string()).await;
    }
}
