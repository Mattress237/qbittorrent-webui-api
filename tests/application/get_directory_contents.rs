use crate::{create_test_data, login_default_client};
use qbit::models::DirMode;

// Yes we might not need to test this 3 times. But eh, whatever. It makes sure it works at least.

#[tokio::test]
#[ignore = "Test hits api endpoint"]
pub async fn list_directory() {
    let folder = create_test_data(None);
    let client = login_default_client().await;
    let contents = client
        .get_directory_contents(&folder, &DirMode::default())
        .await
        .unwrap();
    println!("{:?}", contents);

    assert!(contents.contains(&format!("{folder}/dummy.txt")));
}

#[tokio::test]
#[ignore = "Test hits api endpoint"]
pub async fn list_directory_files() {
    let folder = create_test_data(None);
    let client = login_default_client().await;
    let contents = client
        .get_directory_contents(&folder, &DirMode::Files)
        .await
        .unwrap();
    println!("{:?}", contents);

    assert!(contents.contains(&format!("{folder}/dummy.txt")));
}

#[tokio::test]
#[ignore = "Test hits api endpoint"]
pub async fn list_directory_dirs() {
    let folder = create_test_data(None);
    let client = login_default_client().await;
    let contents = client
        .get_directory_contents(&folder, &DirMode::Dirs)
        .await
        .unwrap();
    println!("{:?}", contents);

    // In theory, this should return true because `create_test_data` should have no directories inside, just files.
    assert!(contents.is_empty());
}
