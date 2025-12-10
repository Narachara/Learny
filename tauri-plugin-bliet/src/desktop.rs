use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Bliet<R>> {
  Ok(Bliet(app.clone()))
}

/// Access to the bliet APIs.
pub struct Bliet<R: Runtime>(AppHandle<R>);

use tauri::api::dialog::FileDialogBuilder;

impl<R: Runtime> Bliet<R> {
    pub fn pick_image(&self) -> crate::Result<String> {
        let (tx, rx) = std::sync::mpsc::channel();

        FileDialogBuilder::new()
            .add_filter("Images", &["png", "jpg", "jpeg"])
            .pick_file(move |file_path| {
                let _ = tx.send(
                    file_path
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_default(),
                );
            });

        let result = rx.recv().unwrap_or_default();
        Ok(result)
    }
}
