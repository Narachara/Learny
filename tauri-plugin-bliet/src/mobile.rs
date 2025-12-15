use serde::de::DeserializeOwned;
use serde_json::json;
use tauri::{ plugin::{ PluginApi, PluginHandle }, AppHandle, Runtime };
use shared::FileResponse;
use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_bliet);

// ================================
// Plugin Initialization
// ================================
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>
) -> crate::Result<Bliet<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.bliet", "ExamplePlugin")?;

    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_bliet)?;

    Ok(Bliet(handle))
}

// ================================
// Plugin Struct
// ================================
pub struct Bliet<R: Runtime>(PluginHandle<R>);

// ================================
// API Implementations
// ================================
impl<R: Runtime> Bliet<R> {
    pub async fn pick_image(&self) -> crate::Result<Option<FileResponse>> {
        // Match the JSON returned by Kotlin:
        // { "path": "..." }
        #[derive(serde::Deserialize)]
        struct PickFileResponse {
            path: Option<String>,
        }

        // TODO:
        // Must create pick_archive and pick_image here like in desktop.rs

        let resp: PickFileResponse = self.0.run_mobile_plugin("pickImage", json!({}))?;

        Ok(Some(FileResponse {
            path: resp.path.unwrap_or_default(),
        }))
    }
}
