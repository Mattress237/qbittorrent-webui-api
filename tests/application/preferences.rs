use qbit::parameters;

use crate::login_default_client;

#[tokio::test]
#[ignore = "Test hits api endpoint"]
pub async fn can_get_preferences_and_prefs_are_serializable() {
    let client = login_default_client().await;

    let result = client.preferences().await;
    println!("{:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore = "Test hits api endpoint"]
pub async fn preferences_are_serializable() {
    let client = login_default_client().await;

    let result = client.preferences().await;
    assert!(result.is_ok());

    let prefs = result.unwrap();
    assert!(serde_json::to_string(&prefs).is_ok());
}

#[tokio::test]
#[ignore = "Test hits api endpoint"]
pub async fn can_update_preferences() {
    let client = login_default_client().await;

    let prefs = parameters::PreferencesBuilder::default()
        .locale("en")
        .build()
        .unwrap();

    let result = client.set_preferences(prefs).await;
    println!("{:?}", result);
    assert!(result.is_ok());
}
