use qbit::{
    Api,
    models::{AddTorrentParams, RssRule},
};

mod rules;

async fn seed_rss_rule(api: &Api) -> Result<(), qbit::Error> {
    let rules = api.rss_rules().await?;
    if rules.contains_key("test-rule") {
        return Ok(());
    }

    let def = RssRule {
        enabled: true,
        priority: 0,
        use_regex: false,
        must_contain: String::new(),
        must_not_contain: String::new(),
        episode_filter: String::new(),
        affected_feeds: Vec::new(),
        last_match: String::new(),
        ignore_days: 0,
        smart_filter: false,
        previously_matched_episodes: Vec::new(),
        torrent_params: AddTorrentParams::default(),
    };
    api.rss_set_rule("test-rule", def).await
}
