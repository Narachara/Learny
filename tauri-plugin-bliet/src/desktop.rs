use serde::de::DeserializeOwned;
use tauri::{ plugin::PluginApi, AppHandle, Runtime, Manager };
use crate::Result;
use std::fs;
use std::path::{ Path, PathBuf };
use futures::channel::oneshot;
use tauri_plugin_dialog::{ DialogExt, FileDialogBuilder, FilePath };
use shared::FileResponse;
use uuid::Uuid;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>
) -> crate::Result<Bliet<R>> {
    Ok(Bliet(app.clone()))
}

pub enum PickKind {
    Image,
    Archive,
}

impl PickKind {
    fn dialog_filter(&self) -> (&'static str, &'static [&'static str]) {
        match self {
            PickKind::Image => ("Images", &["png", "jpg", "jpeg", "webp"]),
            PickKind::Archive => ("Archives", &["zip", "tar", "gz", "7z"]),
        }
    }

    fn default_extension(&self, picked: &Path) -> String {
        picked
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or(match self {
                PickKind::Image => "png",
                PickKind::Archive => "zip",
            })
            .to_string()
    }
}


/// Access to the bliet APIs.
pub struct Bliet<R: Runtime>(pub AppHandle<R>);

impl<R: Runtime> Bliet<R> {
    pub async fn pick_file(
        &self,
        kind: PickKind,
    ) -> crate::Result<Option<FileResponse>> {
        let app = self.0.clone();

        let picked_path: Option<PathBuf> = {
            let (tx, rx) = oneshot::channel();

            let (label, extensions) = kind.dialog_filter();

            FileDialogBuilder::new(app.dialog().clone())
                .add_filter(label, extensions)
                .pick_file(move |file| {
                    let _ = tx.send(file);
                });

            match rx.await? {
                Some(FilePath::Path(path)) => Some(path),
                Some(_) => {
                    return Err("Unsupported file path type".into());
                }
                None => None, // user cancelled
            }
        };

        let Some(picked_path) = picked_path else {
            return Ok(None);
        };

        // --- app data dir ---
        let app_data_dir = app.path().app_data_dir()?;
        let files_dir = app_data_dir.join("files");
        fs::create_dir_all(&files_dir)?;

        let extension = kind.default_extension(&picked_path);
        let file_name = format!("{}.{}", Uuid::new_v4(), extension);
        let target_path = files_dir.join(&file_name);

        fs::copy(&picked_path, &target_path)?;

        let virtual_path = format!("files/{}", file_name);

        Ok(Some(FileResponse {
            path: virtual_path,
        }))
    }

    // Convenience wrappers (optional, nice API)
    pub async fn pick_image(&self) -> crate::Result<Option<FileResponse>> {
        self.pick_file(PickKind::Image).await
    }

    pub async fn pick_archive(&self) -> crate::Result<Option<FileResponse>> {
        self.pick_file(PickKind::Archive).await
    }

    // TODO: 
    // Fix this function to return the bytes instead of the writing directly to system
    // this makes the plugin uncoupled from desktop

    pub async fn pick_import_file(app: tauri::AppHandle) -> Result<i64, String> {
        // 1️⃣ Ask user for file
        let (tx, rx) = oneshot::channel();

        FileDialogBuilder::new(app.dialog().clone())
            .pick_file(move |file| {
                let _ = tx.send(file);
            });

        let Some(FilePath::Path(path)) = rx.await.map_err(|e| e.to_string())?
        else {
            return Ok(0); // user cancelled
        };

        // 2️⃣ Read + import on blocking thread
        let app_clone = app.clone();

        tauri::async_runtime::spawn_blocking(move || {
            let json = std::fs::read_to_string(path)
                .map_err(|e| e.to_string())?;

            let export = import_deck_json(&json)?;
            import_deck_export(&app_clone, export)
        })
        .await
        .map_err(|e| e.to_string())?
    }
}