use crate::{get_server_api_key, get_server_details};
use qbit::{Api, Credentials};

#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn correct_credentials() {
    Api::new_login(
        &get_server_details(),
        Credentials::APIKey(get_server_api_key()),
    )
    .await
    .expect("Incorrect credentials");
}

#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn incorrect_credentials() {
    Api::new_login(
        &get_server_details(),
        Credentials::APIKey("qbt_random_creds".into()),
    )
    .await
    .expect_err("Correct credentials should fail");
}

#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn test_api_key_working_withe_requests() {
    let api = Api::new_login(
        &get_server_details(),
        Credentials::APIKey(get_server_api_key()),
    )
    .await
    .expect("Failed to login with api key");

    api.version().await.expect("Failed to get version");

    api.log(None, None).await.expect("Failed to get log");
}
