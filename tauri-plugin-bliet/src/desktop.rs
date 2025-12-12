use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};
use crate::Result;
use crate::models::*;
use tauri_plugin_dialog::DialogExt;


pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Bliet<R>> {
  Ok(Bliet(app.clone()))
}

/// Access to the bliet APIs.
pub struct Bliet<R: Runtime>(pub AppHandle<R>);


impl<R: Runtime> Bliet<R> {
    pub async fn pick_image(&self) -> Result<String> {
        let (tx, mut rx) = tauri::async_runtime::channel(1);

        self.0
            .dialog()
            .file()
            .add_filter("Images", &["png", "jpg", "jpeg"])
            .pick_file(move |path| {
                let _ = tx.send(
                    path.map(|p| p.to_string().to_string())
                        .unwrap_or_default(),
                );
            });

        // Async await — DOES NOT BLOCK UI
        let result = rx.recv().await.unwrap_or_default();
        Ok(result)
    }
}