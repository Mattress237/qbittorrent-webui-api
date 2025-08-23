use std::collections::HashMap;

use bytes::Bytes;

use crate::{
    Error, insert_optional,
    models::{
        TorrentCreator, TorrentCreatorTask, TorrentCreatorTaskStatus, TorrentFormat,
        TorrentPieceSize,
    },
};

impl super::Api {
    /// Create a task to eventually make a new torrent.
    pub async fn create_task(&self, params: &TorrentCreator) -> Result<TorrentCreatorTask, Error> {
        let mut form = HashMap::new();
        form.insert("sourcePath", params.source_path.clone());

        // apparently all of these are optional...
        insert_optional!(form, "private", params.private, |v: bool| v.to_string());
        insert_optional!(form, "comment", &params.comment, |v: &String| v.to_owned());
        insert_optional!(form, "source", &params.source, |v: &String| v.to_owned());
        insert_optional!(form, "startSeeding", params.start_seeding, |v: bool| v
            .to_string());
        insert_optional!(form, "trackers", &params.trackers, |v: &Vec<String>| v
            .join("|"));
        insert_optional!(form, "urlSeeds", &params.url_seeds, |v: &Vec<String>| v
            .join("|"));
        insert_optional!(form, "format", &params.format, |v: &TorrentFormat| v
            .to_string());
        insert_optional!(
            form,
            "pieceSize",
            &params.piece_size,
            |v: &TorrentPieceSize| v.0.to_string()
        );
        insert_optional!(
            form,
            "optimizeAlignment",
            params.optimize_alignment,
            |v: bool| v.to_string()
        );
        insert_optional!(
            form,
            "paddedFileSizeLimit",
            params.padded_file_size_limit,
            |v: i64| v.to_string()
        );
        insert_optional!(
            form,
            "torrentFilePath",
            &params.torrent_file_path,
            |v: &String| v.to_owned()
        );

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
