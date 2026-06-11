use qbit::{
    Error,
    models::{TaskStatus, TorrentCreatorBuilder},
};

use crate::{create_dummy_torrent, create_random_name, create_test_data, login_default_client};
use std::{env, fs, thread, time::Duration};

/// This test makes sure that the endpoint can create a dummy task.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn create_task() {
    let client = login_default_client().await;
    let random_name = create_random_name("create1_");
    let result = create_dummy_torrent(&client, random_name.clone()).await;

    assert!(result.is_ok());

    // Clean up
    thread::sleep(Duration::from_secs(1));
    client.delete_task(result.unwrap()).await.unwrap();
    let hash = client
        .torrents(None)
        .await
        .unwrap()
        .iter()
        .filter(|t| t.name == random_name)
        .next()
        .map(|t| t.hash.clone())
        .unwrap();
    client.delete(vec![&hash], true).await.unwrap();
    fs::remove_file(format!(
        "{}/_data/{random_name}.torrent",
        env::var("temp_dir").unwrap()
    ))
    .unwrap();
}

/// This test makes sure that the list shows the dummy task we've just created.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn list_tasks() {
    let client = login_default_client().await;
    let random_name = create_random_name("list1_");
    let result = create_dummy_torrent(&client, random_name.clone())
        .await
        .unwrap();
    let list = client.list_tasks().await.unwrap();
    assert!(!list.is_empty());
    assert!(list.iter().any(|t| t.task_id == result));

    // Clean up
    thread::sleep(Duration::from_secs(1));
    client.delete_task(result).await.unwrap();
    fs::remove_dir_all(format!(
        "{}/_data/{random_name}",
        env::var("temp_dir").unwrap()
    ))
    .unwrap();
    fs::remove_file(format!(
        "{}/_data/{random_name}.torrent",
        env::var("temp_dir").unwrap()
    ))
    .unwrap();
}

/// Tests to see that upon the torrent being finished, it is the same as the information we have in the dummy file.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn get_torrent_file() {
    let client = login_default_client().await;
    let random_name = create_random_name("get1_");
    create_test_data();
    let task = create_dummy_torrent(&client, random_name.clone())
        .await
        .unwrap();
    println!("task: {}", task);
    let mut list = client.list_tasks().await.unwrap();

    // This should hopefully let the torrent finish creating before attempting to do other stuff.
    let mut limit = 10;
    while list
        .iter()
        .filter(|v| v.task_id == task)
        .next()
        .unwrap()
        .status
        != TaskStatus::Finished
    {
        println!(
            "{:?}",
            list.iter().filter(|v| v.task_id == task).next().unwrap()
        );
        if limit == 0 {
            panic!("Torrent has not finished creating after ~ 10 seconds of checking.");
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
        list = client.list_tasks().await.unwrap();
        limit -= 1;
    }

    // let folder = format!("{server_folder}", env::var("temp_dir").unwrap());
    let path = format!(
        "{}/_data/{random_name}.torrent",
        env::var("temp_dir").unwrap()
    );
    let data = fs::read(&path).unwrap();

    for item in list.iter() {
        let r = client
            .get_task_file(item.task_id.to_owned())
            .await
            .unwrap_or_default()
            .to_vec();
        if r == data {
            assert!(true);
        }
    }

    // Clean up
    thread::sleep(Duration::from_secs(1));
    let hash = client
        .torrents(None)
        .await
        .unwrap()
        .iter()
        .filter(|t| t.name == random_name)
        .next()
        .map(|t| t.hash.clone())
        .unwrap();
    client.delete(vec![&hash], true).await.unwrap();
    fs::remove_file(format!(
        "{}/_data/{random_name}.torrent",
        env::var("temp_dir").unwrap()
    ))
    .unwrap();
}

/// Make sure that we can delete the created task.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn delete_created_task() {
    let client = login_default_client().await;
    let random_name = create_random_name("delete1_");
    let task_id = create_dummy_torrent(&client, random_name.clone())
        .await
        .unwrap();
    let list = client.list_tasks().await.unwrap();
    assert!(!list.is_empty());
    assert!(list.iter().any(|t| t.task_id == task_id));
    client
        .delete_task(task_id.clone())
        .await
        .expect("Failed to delete task");
    let list = client.list_tasks().await.unwrap();
    assert!(!list.iter().any(|t| t.task_id == task_id));

    thread::sleep(Duration::from_secs(1));
    fs::remove_dir_all(format!(
        "{}/_data/{}",
        env::var("temp_dir").unwrap(),
        random_name
    ))
    .unwrap();
}

/// Test to check failed to create errors
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn make_failed_task() {
    let client = login_default_client().await;

    let torrent = TorrentCreatorBuilder::default()
        .start_seeding(true)
        .private(true)
        .source_path("/tmp/tmp.L9uwLe8LOA")
        .build()
        .expect("Failed to build torrent creator");

    let id = client.create_task(&torrent).await.unwrap();
    let result = client.get_task_file(id).await;

    assert!(result.is_err());
    if let Err(Error::Http409(_e)) = result {
        assert!(true);
        // eprintln!("error: {}", _e);
        // assert!(false);
    } else {
        panic!("Expected Http409 error");
    }
}
