use crate::{login_default_client, rss::seed_rss_rule};

#[tokio::test]
#[ignore = "Test hits api endpoint"]
pub async fn can_deserialize_rules_response() {
    let client = login_default_client().await;
    seed_rss_rule(&client).await.unwrap();

    let rules = client.rss_rules().await.unwrap();
    assert!(!rules.is_empty());

    assert!(rules.contains_key("test-rule"));
    let rule = &rules["test-rule"];
    assert_eq!(rule.enabled, true);
    assert_eq!(rule.torrent_params.category, "");
    assert_eq!(rule.torrent_params.tags, Vec::<String>::new());
}
