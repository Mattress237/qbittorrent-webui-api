use qbit::parameters::TorrentListParamsBuilder;

use crate::{create_and_get_torrent, create_random_name, login_default_client};

/// Tests to see if creating tags is fine.
#[tokio::test]
#[ignore = "Test hits API endpoint"]
pub async fn create_empty_tags() {
    let client = login_default_client().await;
    let result = client.create_tags(vec![""]).await;
    assert!(result.is_ok());
}

/// Tests to see if creating random tags is fine.
#[tokio::test]
#[ignore = "Test hits API endpoint"]
pub async fn create_random_tags() {
    let client = login_default_client().await;

    let result = client
        .create_tags(vec![
            &create_random_name().unwrap(),
            &create_random_name().unwrap(),
            &create_random_name().unwrap(),
        ])
        .await;
    assert!(result.is_ok());
}

/// Tests to see if we can create a torrent, add tags to it, and find it when we filter for just those tags.
#[tokio::test]
#[ignore = "Test hits API endpoint"]
pub async fn create_dummy_with_tags() {
    let client = login_default_client().await;
    let random = create_random_name();
    let torrent = create_and_get_torrent(&client, random).await;
    let tag_name = create_random_name().unwrap();

    client
        .add_tags(Some(vec![&torrent.hash]), vec![&tag_name])
        .await
        .unwrap();

    // Assuming that the random name is random.
    assert_eq!(
        client
            .torrents(Some(
                TorrentListParamsBuilder::default()
                    .tag(tag_name)
                    .build()
                    .unwrap(),
            ))
            .await
            .unwrap()
            .len(),
        1
    );
}

/// Test to see other tags are being added correctly.
#[tokio::test]
#[ignore = "Test hits API endpoint"]
pub async fn get_tags() {
    let client = login_default_client().await;
    let random = create_random_name();
    let torrent = create_and_get_torrent(&client, random).await;
    let tag_name = create_random_name().unwrap();
    let tag_name2 = create_random_name().unwrap();
    let tag_name3 = create_random_name().unwrap();

    client
        .add_tags(
            Some(vec![&torrent.hash]),
            vec![&tag_name, &tag_name2, &tag_name3],
        )
        .await
        .unwrap();

    let torrents = client
        .torrents(Some(
            TorrentListParamsBuilder::default()
                .tag(tag_name)
                .build()
                .unwrap(),
        ))
        .await
        .unwrap();

    // Assuming that the random name is random.
    assert_eq!(torrents.len(), 1);

    let torrent = torrents.first().unwrap();
    assert!(torrent.tags.contains(&tag_name2));
    assert!(torrent.tags.contains(&tag_name3));
}
