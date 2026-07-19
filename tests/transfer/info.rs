use crate::login_default_client;

/// This test ensures that the API correctly deserialize the info response.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn correctly_deserialize_from_response() {
    let client = login_default_client().await;

    let info = client.global_transfer_info().await.unwrap();
    assert!(info.global_download_rate >= -1);
    assert!(info.global_upload_rate >= -1);
    assert!(info.downloaded_this_session >= -1);
    assert!(info.uploaded_this_session >= -1);
}
