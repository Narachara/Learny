use serde::de::DeserializeOwned;
use serde_json::json;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_bliet);

// ================================
// Plugin Initialization
// ================================
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
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
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        self.0.run_mobile_plugin("ping", payload).map_err(Into::into)
    }

    pub fn pick_image(&self) -> crate::Result<String> {
        // Match the JSON returned by Kotlin:
        // { "path": "..." }
        #[derive(serde::Deserialize)]
        struct PickImageResponse {
            path: Option<String>,
        }

        let resp: PickImageResponse =
            self.0.run_mobile_plugin("pickImage", json!({}))?;

        Ok(resp.path.unwrap_or_default())
    }
}