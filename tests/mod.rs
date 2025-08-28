use dotenv::dotenv;
use qbit::{
    Api,
    models::{Torrent, TorrentCreatorBuilder, TorrentCreatorTask},
    parameters::AddTorrentBuilder,
};
use rand::{Rng, distr::Alphabetic, rngs};
use std::{env, fs, path};

pub mod application;
pub mod authentication;
pub mod sync;
pub mod torrents;

pub const DEBIAN_HASH: &str = "6f4370df4304609a8793ce2b59178dcc8febf5e2";
pub const DEBIAN_TRACKER: &str = "magnet:?xt=urn:btih:6f4370df4304609a8793ce2b59178dcc8febf5e2&dn=debian-12.11.0-amd64-netinst.iso&xl=702545920&tr=http%3A%2F%2Fbttracker.debian.org%3A6969%2Fannounce&ws=https://cdimage.debian.org/cdimage/archive/12.11.0/amd64/iso-cd/debian-12.11.0-amd64-netinst.iso&ws=https://cdimage.debian.org/cdimage/release/12.11.0/amd64/iso-cd/debian-12.11.0-amd64-netinst.iso";

pub fn get_server_details() -> String {
    dotenv().ok();

    let url = env::var("url");
    let port = env::var("port");

    if url.is_err() || port.is_err() {
        println!("Default to `http://localhost:45378` as couldn't fully load data from .env");
        return String::from("http://localhost:45378");
    }

    let finished_url = format!("{}:{}", url.unwrap(), port.unwrap());
    println!("Using {} from .env file", finished_url);
    finished_url
}

pub fn get_server_username() -> String {
    dotenv().ok();
    env::var("username").unwrap_or("admin".to_string())
}

pub fn get_server_password() -> String {
    dotenv().ok();
    env::var("password").unwrap_or("adminadmin".to_string())
}

pub async fn login_default_client() -> Api {
    Api::new_login_username_password(
        &get_server_details(),
        &get_server_username(),
        &get_server_password(),
    )
    .await
    .expect("Failed to log in to the default client. Please check the server details, username, and password.")
}

pub async fn add_debian_torrent(client: &Api) {
    let param = AddTorrentBuilder::default()
        .torrents(vec![DEBIAN_TRACKER.to_string()])
        .paused(true)
        .build()
        .expect("Failed to build AddTorrent");

    client
        .add_torrent(param)
        .await
        .expect("Failed to add torrent");
    // Note: Added the stop call since the paused parameter doesn't work for some reason.
    client
        .stop(vec![DEBIAN_HASH])
        .await
        .expect("Failed to stop torrent");
}

pub async fn get_debian_torrent(client: &Api) -> Option<Torrent> {
    let torrents = client
        .torrents(None)
        .await
        .expect("Failed to fetch main data:");

    torrents
        .iter()
        .filter(|t| t.hash == DEBIAN_HASH)
        .next()
        .map(|t| t.to_owned())
}

pub fn create_random_name() -> Option<String> {
    Some(
        rand::rng()
            .sample_iter(&Alphabetic)
            .take(7)
            .map(char::from)
            .collect::<String>(),
    )
}

pub fn create_test_data(random_name: Option<String>) -> String {
    dotenv().ok();
    // persionally did not want to have to do this, but `/tmp` can cause some issues so...
    let folder = env::var("temp_dir").unwrap();

    if !fs::exists(&folder).unwrap() {
        fs::create_dir(&folder).unwrap_or_default();
    }
    if !fs::exists(format!("{folder}_data")).unwrap() {
        fs::create_dir(format!("{folder}_data")).unwrap_or_default();
    }

    fs::write(
        format!("{folder}/dummy{}.txt", random_name.unwrap_or_default()),
        "This is a dummy file. You are a dummy for downloading this file.",
    )
    .expect("Failed to write dummy file");

    path::absolute(folder).unwrap().display().to_string()
}

pub async fn create_dummy_torrent(
    client: &Api,
    random_name: Option<String>,
) -> Result<TorrentCreatorTask, qbit::Error> {
    let folder = create_test_data(random_name.clone());
    let torrent = TorrentCreatorBuilder::default()
        .source_path(&folder)
        .start_seeding(true)
        .private(true)
        .comment("Dummy comment for a dummy torrent")
        .source("https://example.com/dummy")
        .torrent_file_path(format!(
            "{folder}_data/dummy{}.torrent",
            random_name.unwrap_or_default()
        ))
        .build()
        .expect("Failed to build torrent creator");

    client.create_task(&torrent).await
}
