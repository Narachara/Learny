use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime, Manager};
use crate::Result;
use std::fs;
use std::path::{Path, PathBuf};
use futures::channel::oneshot;
use tauri_plugin_dialog::{DialogExt, FileDialogBuilder, FilePath};
use shared::ImageResponse;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Bliet<R>> {
  Ok(Bliet(app.clone()))
}

/// Access to the bliet APIs.
pub struct Bliet<R: Runtime>(pub AppHandle<R>);


impl<R: Runtime> Bliet<R> {
    pub async fn pick_image(&self) -> crate::Result<ImageResponse> {
        let app = self.0.clone();

        // --- open dialog ---
        let picked_path: PathBuf = {
            let (tx, rx) = oneshot::channel();

            FileDialogBuilder::new(app.dialog().clone())
                .add_filter("Images", &["png", "jpg", "jpeg", "webp"])
                .pick_file(move |file| {
                    let _ = tx.send(file);
                });

            match rx.await?
                .ok_or("No file selected")?
            {
                FilePath::Path(path) => path,
                _ => return Err("Unsupported file path type".into()),
            }
        };

        // --- app data dir ---
        let app_data_dir = app.path().app_data_dir()?;
        let files_dir = app_data_dir.join("Files");

        fs::create_dir_all(&files_dir)?;

        // --- copy file ---
        let file_name = picked_path
            .file_name()
            .ok_or("Invalid file name")?;

        let target_path = files_dir.join(file_name);

        fs::copy(&picked_path, &target_path)?;

        Ok(ImageResponse { path: target_path.to_string_lossy().to_string() })
    }
}