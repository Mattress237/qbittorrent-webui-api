use dotenv::dotenv;
use qbit::{
    Api,
    models::{Torrent, TorrentCreatorBuilder, TorrentCreatorTask},
    parameters::AddTorrentBuilder,
};
use rand::{RngExt, distr::Alphabetic};
use std::{env, fs};

pub mod application;
pub mod authentication;
pub mod sync;
pub mod torrents;

pub const DEBIAN_HASH: &str = "6f4370df4304609a8793ce2b59178dcc8febf5e2";
pub const DEBIAN_TRACKER: &str = "magnet:?xt=urn:btih:6f4370df4304609a8793ce2b59178dcc8febf5e2&dn=debian-12.11.0-amd64-netinst.iso&xl=702545920&tr=http%3A%2F%2Fbttracker.debian.org%3A6969%2Fannounce&ws=https://cdimage.debian.org/cdimage/archive/12.11.0/amd64/iso-cd/debian-12.11.0-amd64-netinst.iso&ws=https://cdimage.debian.org/cdimage/release/12.11.0/amd64/iso-cd/debian-12.11.0-amd64-netinst.iso";

// Relative path to the dummy file within the test data directory.
const DUMMY_FILE: &str = "dummy/dummy.txt";

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

    if client.torrent(DEBIAN_HASH).await.is_ok() {
        return;
    }

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

pub fn create_random_name(prefix: &str) -> String {
    let name = rand::rng()
        .sample_iter(&Alphabetic)
        .take(7)
        .map(char::from)
        .collect::<String>();
    format!("{prefix}{name}")
}

// NOTE: The random name is ignored and a dummy file is always created.
pub fn create_test_data() -> String {
    dotenv().ok();
    if env::var("temp_dir").is_err() {
        panic!("'temp_dir' not set. Required for test data creation.");
    }
    if env::var("server_temp_dir").is_err() {
        panic!("'server_temp_dir' not set. Required for test data creation.");
    }

    // persionally did not want to have to do this, but `/tmp` can cause some issues so...
    // Create the temp directory if it doesn't exist
    let folder = env::var("temp_dir").unwrap();
    if !fs::exists(format!("{folder}")).unwrap() {
        fs::create_dir(format!("{folder}")).unwrap_or_default();
    }
    if !fs::exists(format!("{folder}/dummy")).unwrap() {
        fs::create_dir(format!("{folder}/dummy")).unwrap_or_default();
    }
    if !fs::exists(format!("{folder}/_data")).unwrap() {
        fs::create_dir(format!("{folder}/_data")).unwrap_or_default();
    }

    // Create dummy file if it doesn't exist
    if !fs::exists(format!("{folder}/dummy/dummy.txt")).unwrap() {
        fs::write(
            format!("{folder}/{DUMMY_FILE}"),
            "This is a dummy file. You are a dummy for downloading this file.",
        )
        .expect("Failed to write dummy file");
    }

    // Edited so that the folder path is returned as the server temporary test directory
    // path::absolute(folder).unwrap().display().to_string()
    env::var("server_temp_dir").unwrap()
}

pub async fn create_dummy_torrent(
    client: &Api,
    name: String,
) -> Result<TorrentCreatorTask, qbit::Error> {
    let server_folder = create_test_data();
    let folder = format!("{}/_data/{name}", env::var("temp_dir").unwrap());
    fs::create_dir_all(&folder).unwrap_or_default();
    println!(
        "{} : {}",
        format!("{}/{DUMMY_FILE}", env::var("temp_dir").unwrap()),
        format!("{folder}/dummy.txt")
    );
    println!(
        "{:?} : {:?}",
        fs::exists(format!("{}/{DUMMY_FILE}", env::var("temp_dir").unwrap())),
        fs::exists(format!("{folder}/dummy.txt"))
    );
    fs::copy(
        format!("{}/{DUMMY_FILE}", env::var("temp_dir").unwrap()),
        format!("{folder}/dummy.txt"),
    )
    .unwrap();

    let torrent = TorrentCreatorBuilder::default()
        .source_path(format!("{server_folder}/_data/{name}"))
        .start_seeding(true)
        .private(true)
        .comment("Dummy comment for a dummy torrent")
        .source("https://example.com/dummy")
        .torrent_file_path(format!("{server_folder}/_data/{name}.torrent"))
        .build()
        .expect("Failed to build torrent creator");

    client.create_task(&torrent).await
}
