use std::collections::HashMap;

use reqwest::multipart;

use crate::{
    error::Error,
    models::{
        FilePriority, PiecesState, Torrent, TorrentContent, TorrentProperties, Tracker, WebSeed,
    },
    parameters::{AddTorrent, AddTorrentType, TorrentListParams},
};

impl super::Api {
    /// Get torrent listf
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-torrent-list)
    ///
    /// # Arguments
    ///
    /// * `parames` - Parameter object
    ///
    pub async fn torrents(&self, params: Option<TorrentListParams>) -> Result<Vec<Torrent>, Error> {
        let mut query = vec![];

        let params = params.unwrap_or_default();

        query.push(("reverse", params.reverse.to_string()));
        if let Some(filter) = params.filter {
            query.push(("filter", filter.to_string()));
        }
        if let Some(category) = params.category {
            query.push(("category", category));
        }
        if let Some(tag) = params.tag {
            query.push(("tag", tag));
        }
        if let Some(sort) = params.sort {
            query.push(("sort", sort.to_string()));
        }
        if let Some(limit) = params.limit {
            query.push(("limit", limit.to_string()));
        }
        if let Some(offset) = params.offset {
            query.push(("offset", offset.to_string()));
        }
        if let Some(hashes) = params.hashes {
            query.push(("hashes", hashes.join("|")));
        }

        let torrents = self
            ._get("torrents/info")
            .await?
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Torrent>>()
            .await?;

        Ok(torrents)
    }

    /// Get torrent generic properties
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-torrent-generic-properties)
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent you want to get the generic properties of.
    ///
    pub async fn torrent(&self, hash: &str) -> Result<TorrentProperties, Error> {
        let query = vec![("hash", hash)];

        let torrent = self
            ._get("torrents/properties")
            .await?
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<TorrentProperties>()
            .await?;

        Ok(torrent)
    }

    /// Get torrent trackers
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-torrent-trackers)
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent you want to get the trackers of.
    ///
    pub async fn trackers(&self, hash: &str) -> Result<Vec<Tracker>, Error> {
        let query = vec![("hash", hash)];

        let trackers = self
            ._get("torrents/trackers")
            .await?
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Tracker>>()
            .await?;

        Ok(trackers)
    }

    /// Get torrent web seeds
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-torrent-web-seeds)
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent you want to get the webseeds of.
    ///
    pub async fn webseeds(&self, hash: &str) -> Result<Vec<WebSeed>, Error> {
        let query = vec![("hash", hash)];

        let webseeds = self
            ._get("torrents/webseeds")
            .await?
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<WebSeed>>()
            .await?;

        Ok(webseeds)
    }

    /// Get torrent contents
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-torrent-contents)
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent you want to get the files of.
    /// * `indexes` - The indexes of the files you want to retrieve. If `None`
    ///   all files will be selected.
    ///
    pub async fn files(
        &self,
        hash: &str,
        indexes: Option<Vec<i64>>,
    ) -> Result<Vec<TorrentContent>, Error> {
        let mut query = vec![];
        query.push(("hash", hash.to_string()));
        if let Some(indexes) = indexes {
            query.push((
                "filter",
                indexes
                    .iter()
                    .map(|&x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("|"),
            ));
        }

        let webseeds = self
            ._get("torrents/files")
            .await?
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<TorrentContent>>()
            .await?;

        Ok(webseeds)
    }

    /// Get torrent pieces' states
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-torrent-pieces-states)
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent you want to get the piece states of.
    ///
    pub async fn pieces_states(&self, hash: &str) -> Result<Vec<PiecesState>, Error> {
        let query = vec![("hash", hash)];

        let pieces = self
            ._get("torrents/pieceStates")
            .await?
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<PiecesState>>()
            .await?;

        Ok(pieces)
    }

    /// Get torrent pieces' hashes
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-torrent-pieces-hashes)
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent you want to get the pieces hashes of.
    ///
    pub async fn pieces_hashes(&self, hash: &str) -> Result<Vec<String>, Error> {
        let query = vec![("hash", hash)];

        let pieces = self
            ._get("torrents/pieceHashes")
            .await?
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<String>>()
            .await?;

        Ok(pieces)
    }

    /// Pause torrents
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#pause-torrents)
    ///
    /// # Arguments
    ///
    /// * `hashes` - Hashes list of torrents to stop.
    ///
    pub async fn stop(&self, hashes: Vec<&str>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("hashes", hashes.join("|"));

        self._post("torrents/stop")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Resume torrents
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#resume-torrents)
    ///
    /// # Arguments
    ///
    /// * `hashes` - Hashes list of torrents to start.
    ///
    pub async fn start(&self, hashes: Vec<&str>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("hashes", hashes.join("|"));

        self._post("torrents/start")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Delete torrents
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#delete-torrents)
    ///
    /// # Arguments
    ///
    /// * `hashes` - Hashes list of torrents to delete.
    /// * `delete_files` - If set to `true`, the downloaded data will also be deleted,
    ///   otherwise has no effect.
    ///
    pub async fn delete(&self, hashes: Vec<&str>, delete_files: bool) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("hashes", hashes.join("|"));
        form = form.text("deleteFiles", delete_files.to_string());

        self._post("torrents/delete")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Recheck torrents
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#recheck-torrents)
    ///
    /// # Arguments
    ///
    /// * `hashes` - Hashes list of torrents to recheck.
    ///
    pub async fn recheck(&self, hashes: Vec<&str>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("hashes", hashes.join("|"));

        self._post("torrents/recheck")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Reannounce torrents
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#reannounce-torrents)
    ///
    /// # Arguments
    ///
    /// * `hashes` - Hashes list of torrents to reannounce.
    ///
    pub async fn reannounce(&self, hashes: Vec<&str>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("hashes", hashes.join("|"));

        self._post("torrents/reannounce")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Add new torrent
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#add-new-torrent)
    ///
    /// # Arguments
    ///
    /// * `params` - Torrent parameters
    ///
    pub async fn add_torrent(&self, params: AddTorrent) -> Result<(), Error> {
        if params.torrents.is_empty() {
            return Err(Error::InvalidRequest(
                "Expected urls or torrents to not be empty!".to_string(),
            ));
        }

        let mut form = multipart::Form::new();
        match params.torrents {
            AddTorrentType::Links(items) => {
                form = form.text("urls", items.join("\n"));
            }
            AddTorrentType::Files(torrent_files) => {
                for file in torrent_files {
                    let mut filename = file.filename;
                    if !filename.ends_with(".torrent") {
                        filename.insert_str(0, ".torrent");
                    }

                    form = form.part(
                        "torrents",
                        multipart::Part::bytes(file.data)
                            .file_name(filename)
                            .mime_str("application/x-bittorrent")?,
                    );
                }
            }
        };

        form = form.text("skip_checking", params.skip_checking.to_string());
        form = form.text("paused", params.paused.to_string());
        form = form.text("autoTMM", params.auto_tmm.to_string());
        form = form.text("sequentialDownload", params.sequential_download.to_string());
        form = form.text("contentLayout", params.content_layout.to_string());
        form = form.text(
            "firstLastPiecePrio",
            params.first_last_piece_prio.to_string(),
        );
        if let Some(savepath) = params.savepath {
            form = form.text("savepath", savepath);
        }
        if let Some(category) = params.category {
            form = form.text("category", category);
        }
        if let Some(tags) = params.tags {
            form = form.text("tags", tags.join(","));
        }
        if let Some(rename) = params.rename {
            form = form.text("rename", rename);
        }
        if let Some(up_limit) = params.up_limit {
            form = form.text("upLimit", up_limit.to_string());
        }
        if let Some(dl_limit) = params.dl_limit {
            form = form.text("dlLimit", dl_limit.to_string());
        }
        if let Some(ratio_limit) = params.ratio_limit {
            form = form.text("ratioLimit", ratio_limit.to_string());
        }
        if let Some(seeding_time_limit) = params.seeding_time_limit {
            form = form.text("seedingTimeLimit", seeding_time_limit.to_string());
        }

        self._post("torrents/add")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Add trackers to torrent
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#add-trackers-to-torrent)
    ///
    /// # Arguments
    ///
    /// * `hash` - Torrent hash.
    /// * `urls` - Trackers urls to add.
    ///
    pub async fn add_trackers_to_torrent(&self, hash: &str, urls: Vec<&str>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("hash", hash.to_string());
        form = form.text("urls", urls.join("%0A"));

        self._post("torrents/addTrackers")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Edit trackers
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#edit-trackers)
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent.
    /// * `orig_url` - The tracker URL you want to edit.
    /// * `new_url` - The new URL to replace the `orig_url`.
    ///
    pub async fn edit_tracker_for_torrent(
        &self,
        hash: &str,
        orig_url: &str,
        new_url: &str,
    ) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("hash", hash.to_string());
        form = form.text("origUrl", orig_url.to_string());
        form = form.text("newUrl", new_url.to_string());

        self._post("torrents/editTracker")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Remove trackers from torrent
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#remove-trackers)
    ///
    /// # Arguments
    ///
    /// * `hash` - Torrent hash.
    /// * `urls` - Trackers urls to remove.
    ///
    pub async fn remove_trackers_from_torrent(
        &self,
        hash: &str,
        urls: Vec<&str>,
    ) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("hash", hash.to_string());
        form = form.text("urls", urls.join("|"));

        self._post("torrents/removeTrackers")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Remove trackers from torrent
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#add-peers)
    ///
    /// # Arguments
    ///
    /// * `hashes` - Torrent hash.
    /// * `peers` - The peer to add. Each peer is a colon-separated `host:port`.
    ///
    pub async fn add_peers(&self, hashes: Vec<&str>, peers: Vec<&str>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("hashes", hashes.join("|"));
        form = form.text("peers", peers.join("|"));

        self._post("torrents/addPeers")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Increase torrent priority
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#increase-torrent-priority)
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to increase the priority of.
    ///   If `None` all torrents are selected.
    ///
    pub async fn increase_priority(&self, hashes: Option<Vec<&str>>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }

        self._post("torrents/increasePrio")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Decrease torrent priority
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#decrease-torrent-priority)
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to decrease the priority of.
    ///   If `None` all torrents are selected.
    ///
    pub async fn decrease_priority(&self, hashes: Option<Vec<&str>>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }

        self._post("torrents/decreasePrio")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Maximal torrent priority
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#maximal-torrent-priority)
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to max the priority of.
    ///   If `None` all torrents are selected.
    ///
    pub async fn max_priority(&self, hashes: Option<Vec<&str>>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }

        self._post("torrents/topPrio")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Minimal torrent priority
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#minimal-torrent-priority)
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to min the priority of.
    ///   If `None` all torrents are selected.
    ///
    pub async fn min_priority(&self, hashes: Option<Vec<&str>>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }

        self._post("torrents/bottomPrio")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Set file priority
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#set-file-priority)
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent.
    /// * `file_ids` - File ids.
    /// * `priority` - File priority to set.
    ///
    pub async fn set_file_priority(
        &self,
        hash: &str,
        file_ids: Vec<u64>,
        priority: FilePriority,
    ) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("hash", hash.to_string());
        form = form.text(
            "id",
            file_ids
                .iter()
                .map(|&num| num.to_string())
                .collect::<Vec<String>>()
                .join("|"),
        );
        form = form.text("priority", serde_json::to_string(&priority)?);

        self._post("torrents/filePrio")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Get torrent download limit
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-torrent-download-limit)
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to get the download limit of.
    ///   If `None` all torrents are selected.
    ///
    pub async fn download_limit(
        &self,
        hashes: Option<Vec<&str>>,
    ) -> Result<HashMap<String, u64>, Error> {
        let mut query = vec![];
        if let Some(hashes) = hashes {
            query.push(("hashes", hashes.join("|")));
        } else {
            query.push(("hashes", "all".to_string()));
        }

        let limites = self
            ._get("torrents/downloadLimit")
            .await?
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<HashMap<String, u64>>()
            .await?;

        Ok(limites)
    }

    /// Set torrent download limit
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#set-torrent-download-limit)
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to set the download limit of.
    ///   If `None` all torrents are selected.
    /// * `limit` - Download limit
    ///
    pub async fn set_download_limit(
        &self,
        hashes: Option<Vec<&str>>,
        limit: u64,
    ) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("limit", limit.to_string());

        self._post("torrents/setDownloadLimit")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Set torrent share limit
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#set-torrent-share-limit)
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to set the share limit of.
    ///   If `None` all torrents are selected.
    /// * `ratio_limit` - The maximum seeding ratio for the torrent. `-2` means
    ///   the global limit should be used, `-1` means no limit.
    /// * `seeding_time_limit` - The maximum seeding time (minutes) for the torrent.
    ///   `-2` means the global limit should be used, `-1` means no limit.
    /// * `inactive_seeding_time_limit` - The maximum amount of time (minutes) the
    ///   torrent is allowed to seed while being inactive. `-2` means the global limit
    ///   should be used, `-1` means no limit.
    ///
    pub async fn set_share_limit(
        &self,
        hashes: Option<Vec<&str>>,
        ratio_limit: f64,
        seeding_time_limit: i64,
        inactive_seeding_time_limit: i64,
    ) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("ratioLimit", ratio_limit.to_string());
        form = form.text("seedingTimeLimit", seeding_time_limit.to_string());
        form = form.text(
            "inactiveSeedingTimeLimit",
            inactive_seeding_time_limit.to_string(),
        );

        self._post("torrents/setShareLimits")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Get torrent upload limit
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-torrent-upload-limit)
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want the upload limit of.
    ///   If `None` all torrents are selected.
    ///
    pub async fn upload_limit(
        &self,
        hashes: Option<Vec<&str>>,
    ) -> Result<HashMap<String, i64>, Error> {
        let mut query = vec![];
        if let Some(hashes) = hashes {
            query.push(("hashes", hashes.join("|")));
        } else {
            query.push(("hashes", "all".to_string()));
        }

        let limites = self
            ._get("torrents/uploadLimit")
            .await?
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<HashMap<String, i64>>()
            .await?;

        Ok(limites)
    }

    /// Set torrent upload limit
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#set-torrent-upload-limit)
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to set the upload limit of.
    ///   If `None` all torrents are selected.
    /// * `limit` - Upload limit
    ///
    pub async fn set_upload_limit(
        &self,
        hashes: Option<Vec<&str>>,
        limit: u64,
    ) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("limit", limit.to_string());

        self._post("torrents/setUploadLimit")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Set torrent location
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#set-torrent-location)
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to set the location of.
    ///   If `None` all torrents are selected.
    /// * `location` - Location to download the torrent to.
    ///
    pub async fn set_location(
        &self,
        hashes: Option<Vec<&str>>,
        location: &str,
    ) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("location", location.to_string());

        self._post("torrents/setLocation")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Set torrent name
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#set-torrent-name)
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent you want to set the name of.
    /// * `name` - Location to download the torrent to.
    ///
    pub async fn set_name(&self, hash: &str, name: &str) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("hash", hash.to_string());
        form = form.text("name", name.to_string());

        self._post("torrents/setLocation")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Set torrent category
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#set-torrent-category)
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to set the category of.
    ///   If `None` all torrents are selected.
    /// * `category` - Name of the category you want to set.
    ///
    pub async fn set_category(
        &self,
        hashes: Option<Vec<&str>>,
        category: &str,
    ) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("category", category.to_string());

        self._post("torrents/setCategory")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Get all categories
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-all-categories)
    ///
    pub async fn categories(&self) -> Result<Vec<String>, Error> {
        let categories = self
            ._get("torrents/categories")
            .await?
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<String>>()
            .await?;

        Ok(categories)
    }

    /// Add new category
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#add-new-category)
    ///
    /// # Arguments
    ///
    /// * `category` - Name for the category to create.
    /// * `save_path` - Path to download torrents for the category.
    ///
    pub async fn create_category(&self, category: &str, save_path: &str) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("category", category.to_string());
        form = form.text("savePath", save_path.to_string());

        self._post("torrents/createCategory")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Edit category
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#edit-category)
    ///
    /// # Arguments
    ///
    /// * `category` - Name for the category to edit.
    /// * `save_path` - Path to download torrents for the category.
    ///
    pub async fn edit_category(&self, category: &str, save_path: &str) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("category", category.to_string());
        form = form.text("savePath", save_path.to_string());

        self._post("torrents/editCategory")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Remove categories
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#remove-categories)
    ///
    /// # Arguments
    ///
    /// * `categories` - List of category names to remove.
    ///
    pub async fn remove_categories(&self, categories: Vec<&str>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("categories", categories.join("\n"));

        self._post("torrents/removeCategories")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Add torrent tags
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#add-torrent-tags)
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to set the tags of.
    ///   If `None` all torrents are selected.
    /// * `tags` - List of names for the tags you want to set.
    ///
    pub async fn add_tags(&self, hashes: Option<Vec<&str>>, tags: Vec<&str>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("tags", tags.join(","));

        self._post("torrents/addTags")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Remove torrent tags
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#remove-torrent-tags)
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to remove the tags of.
    ///   If `None` all torrents are selected.
    /// * `tags` - List of names for the tags you want to remove.
    ///
    pub async fn remove_tags(
        &self,
        hashes: Option<Vec<&str>>,
        tags: Vec<&str>,
    ) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("tags", tags.join(","));

        self._post("torrents/removeTags")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Get all tags
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-all-tags)
    ///
    pub async fn tags(&self) -> Result<Vec<String>, Error> {
        let tags = self
            ._get("torrents/tags")
            .await?
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<String>>()
            .await?;

        Ok(tags)
    }

    /// Create tags
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#create-tags)
    ///
    /// # Arguments
    ///
    /// * `tags` - List of tags to create.
    ///
    pub async fn create_tags(&self, tags: Vec<&str>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("tags", tags.join(","));

        self._post("torrents/createTags")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Delete tags
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#delete-tags)
    ///
    /// # Arguments
    ///
    /// * `tags` - List of tags to delete.
    ///
    pub async fn torrent_delete_tags(&self, tags: Vec<&str>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("tags", tags.join(","));

        self._post("torrents/deleteTags")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Set automatic torrent management
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#set-automatic-torrent-management)
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to set automatic torrent management of.
    ///   If `None` all torrents are selected.
    /// * `enable`
    ///
    pub async fn set_automatic_torrent_management(
        &self,
        hashes: Option<Vec<&str>>,
        enable: bool,
    ) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("enable", enable.to_string());

        self._post("torrents/setAutoManagement")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Toggle sequential download
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#toggle-sequential-download)
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to toggle sequential download for.
    ///   If `None` all torrents are selected.
    ///
    pub async fn toggle_sequential_download(&self, hashes: Option<Vec<&str>>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }

        self._post("torrents/toggleSequentialDownload")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Toggle first/last piece priority
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#set-firstlast-piece-priority)
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to toggle first/last piece priority for.
    ///   If `None` all torrents are selected.
    ///
    pub async fn toggle_first_last_priority(&self, hashes: Option<Vec<&str>>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }

        self._post("torrents/toggleFirstLastPiecePrio")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Set force start
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#set-force-start)
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to set force start of.
    ///   If `None` all torrents are selected.
    /// * `enable`
    ///
    pub async fn set_force_start(
        &self,
        hashes: Option<Vec<&str>>,
        enable: bool,
    ) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("value", enable.to_string());

        self._post("torrents/setForceStart")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Set super seeding
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#set-super-seeding)
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to set super seeding of.
    ///   If `None` all torrents are selected.
    /// * `enable`
    ///
    pub async fn set_super_seeding(
        &self,
        hashes: Option<Vec<&str>>,
        enable: bool,
    ) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("value", enable.to_string());

        self._post("torrents/setSuperSeeding")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Rename file
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#rename-file)
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent
    /// * `oldPath` - The old path of the torrent
    /// * `newPath` - The new path to use for the file
    ///
    pub async fn rename_file(
        &self,
        hash: &str,
        old_path: &str,
        new_path: &str,
    ) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("hash", hash.to_string());
        form = form.text("oldPath", old_path.to_string());
        form = form.text("newPath", new_path.to_string());

        self._post("torrents/renameFile")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Rename folder
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#rename-folder)
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent
    /// * `oldPath` - The old path of the torrent
    /// * `newPath` - The new path to use for the file
    ///
    pub async fn torrent_rename_folder(
        &self,
        hash: &str,
        old_path: &str,
        new_path: &str,
    ) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("hash", hash.to_string());
        form = form.text("oldPath", old_path.to_string());
        form = form.text("newPath", new_path.to_string());

        self._post("torrents/renameFolder")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
