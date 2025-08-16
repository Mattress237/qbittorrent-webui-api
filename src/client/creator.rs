use std::collections::HashMap;

use bytes::Bytes;

use crate::{
    Error,
    models::{TorrentCreator, TorrentCreatorTask, TorrentCreatorTaskStatus},
};

impl super::Api {
    /// Create a task to eventually make a new torrent.
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
            form.insert("trackers", seeds.join("|"));
        }
        if let Some(source) = &params.source {
            form.insert("trackers", source.clone());
        }
        if let Some(comment) = &params.comment {
            form.insert("trackers", comment.clone());
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

    /// Get the `.torrent` file for a given task id. (Task must have finished)
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
            .await?
            .error_for_status();

        match data {
            Ok(d) => Ok(d.bytes().await?),
            Err(e) => {
                if e.status().unwrap().as_u16() == 409 {
                    Err(Error::CreateTorrentNotFonshed)
                } else {
                    Err(Error::ReqwestError(e))
                }
            }
        }
    }

    /// Delete the task with the given id.
    pub async fn delete_task(&self, task_id: &TorrentCreatorTask) -> Result<(), Error> {
        let mut data = HashMap::new();
        data.insert("taskID", task_id.task_id.to_owned());

        self._post("torrentcreator/deleteTask")
            .await?
            .form(&data)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
