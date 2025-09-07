use std::collections::HashMap;

use bytes::Bytes;

use crate::{
    Error,
    models::{TorrentCreator, TorrentCreatorTask, TorrentCreatorTaskStatus},
};

impl super::Api {
    /// Create a task to eventually make a new torrent.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    /// use qbit::models::TorrentCreator;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let torrent = TorrentCreator::default();
    ///     let result = client.create_task(&torrent).await;
    ///
    ///     assert!(result.is_ok());
    ///     println!("{}", result.ok().unwrap().task_id);
    /// }
    /// ```
    pub async fn create_task(&self, params: &TorrentCreator) -> Result<TorrentCreatorTask, Error> {
        let mut form = HashMap::new();
        form.insert("sourcePath", params.source_path.clone());

        // apparently all of these are optional...
        if let Some(format) = &params.format {
            form.insert("format", format.to_string());
        }
        if let Some(piece) = &params.piece_size {
            form.insert("pieceSize", piece.0.to_string());
        }
        if let Some(optimize) = &params.optimize_alignment {
            form.insert("optimizeAlignment", optimize.to_string());
        }
        if let Some(padded_limit) = &params.padded_file_size_limit {
            form.insert("paddedFileSizeLimit", padded_limit.to_string());
        }
        if let Some(private) = params.private {
            form.insert("private", private.to_string());
        }
        if let Some(seeding) = params.start_seeding {
            form.insert("startSeeding", seeding.to_string());
        }
        if let Some(file_path) = &params.torrent_file_path {
            form.insert("torrentFilePath", file_path.clone());
        }
        if let Some(trackers) = &params.trackers {
            form.insert("trackers", trackers.join("|"));
        }
        if let Some(seeds) = &params.url_seeds {
            form.insert("urlSeeds", seeds.join("|"));
        }
        if let Some(source) = &params.source {
            form.insert("source", source.clone());
        }
        if let Some(comment) = &params.comment {
            form.insert("comment", comment.clone());
        }

        Ok(self
            ._post("torrentcreator/addTask")
            .await?
            .form(&form)
            .send()
            .await?
            .error_for_status()?
            .json::<TorrentCreatorTask>()
            .await?)
    }

    /// List all tasks that have been created before.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let list = client.list_tasks().await.unwrap();
    ///
    ///     for item in list {
    ///         println!("{:?}", item);
    ///     }
    /// }
    /// ```
    pub async fn list_tasks(&self) -> Result<Vec<TorrentCreatorTaskStatus>, Error> {
        Ok(self
            ._get("torrentcreator/status")
            .await?
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<TorrentCreatorTaskStatus>>()
            .await?)
    }

    /// Get the `.torrent` file for a given task id. (Task must be finished)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    /// use std::fs;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let raw_task = client.get_task_file("task_id".to_string())
    ///         .await
    ///         .unwrap();
    ///
    ///     fs::write("task.torrent", raw_task);
    /// }
    /// ```
    pub async fn get_task_file(
        &self,
        task_id: impl Into<TorrentCreatorTask>,
    ) -> Result<Bytes, Error> {
        let mut data = HashMap::new();
        data.insert("taskID", task_id.into().task_id.to_owned());

        let data = self
            ._post("torrentcreator/torrentFile")
            .await?
            .form(&data)
            .send()
            .await?;

        match data.error_for_status_ref() {
            Ok(_) => Ok(data.bytes().await?),
            Err(e) => {
                if e.status().unwrap().as_u16() == 409 {
                    Err(Error::Http409(data.text().await.unwrap()))
                } else {
                    Err(Error::ReqwestError(e))
                }
            }
        }
    }

    /// Delete the task with the given id.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    /// use qbit::models::TorrentCreator;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let torrent = TorrentCreator::default();
    ///     let torrent_task = client.create_task(&torrent).await.unwrap();
    ///     let result = client.delete_task(torrent_task).await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn delete_task(&self, task_id: impl Into<TorrentCreatorTask>) -> Result<(), Error> {
        let mut data = HashMap::new();
        data.insert("taskID", task_id.into().task_id.to_owned());

        self._post("torrentcreator/deleteTask")
            .await?
            .form(&data)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
