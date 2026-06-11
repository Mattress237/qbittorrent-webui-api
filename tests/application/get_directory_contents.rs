use std::{env, fs};

use crate::{create_test_data, login_default_client};
use qbit::models::DirMode;

// Yes we might not need to test this 3 times. But eh, whatever. It makes sure it works at least.

#[tokio::test]
#[ignore = "Test hits api endpoint"]
pub async fn list_directory() {
    let folder = create_test_data();
    let client = login_default_client().await;
    let contents = client
        .get_directory_contents(&format!("{folder}/dummy"), &DirMode::default())
        .await
        .unwrap();
    println!("{:?}", contents);

    assert!(contents.contains(&format!("{folder}/dummy/dummy.txt")));
}

#[tokio::test]
#[ignore = "Test hits api endpoint"]
pub async fn list_directory_files() {
    let folder = create_test_data();
    let client = login_default_client().await;
    let contents = client
        .get_directory_contents(&format!("{folder}/dummy"), &DirMode::Files)
        .await
        .unwrap();
    println!("{:?}", contents);

    assert!(contents.contains(&format!("{folder}/dummy/dummy.txt")));
}

#[tokio::test]
#[ignore = "Test hits api endpoint"]
pub async fn list_directory_dirs() {
    let folder = create_test_data();
    let temp_dir = env::var("temp_dir").unwrap();
    if fs::exists(format!("{temp_dir}/dir_test_empty_list")).unwrap() {
        fs::remove_dir_all(format!("{temp_dir}/dir_test_empty_list")).unwrap();
    }
    fs::create_dir(format!("{temp_dir}/dir_test_empty_list")).unwrap_or_default();
    let client = login_default_client().await;
    let contents = client
        .get_directory_contents(&format!("{folder}/dir_test_empty_list"), &DirMode::All)
        .await
        .unwrap();
    println!("{:?}", contents);

    assert!(contents.is_empty());
}

#[tokio::test]
#[ignore = "Test hits api endpoint"]
pub async fn list_directory_dirs_not_empty() {
    let folder = create_test_data();
    let client = login_default_client().await;
    let contents = client
        .get_directory_contents(&format!("{folder}/dummy"), &DirMode::All)
        .await
        .unwrap();
    println!("{:?}", contents);

    assert!(!contents.is_empty());
}
