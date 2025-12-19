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
        #[derive(serde::Deserialize)]
        struct PickFileResponse {
            path: Option<String>,
        }

        let resp: PickFileResponse =
            self.0.run_mobile_plugin("pickImage", json!({}))?;

        Ok(resp.path.map(|path| FileResponse { path }))
    }

    pub async fn pick_archive(&self) -> crate::Result<Option<FileResponse>> {
        #[derive(serde::Deserialize)]
        struct PickFileResponse {
            path: Option<String>,
        }

        let resp: PickFileResponse =
            self.0.run_mobile_plugin("pickArchive", json!({}))?;

        Ok(resp.path.map(|path| FileResponse { path }))
    }

    pub async fn pick_import_file(&self) -> crate::Result<Option<Vec<u8>>> {
        #[derive(serde::Deserialize)]
        struct PickImportResponse {
            data: Option<String>,
        }

        let resp: PickImportResponse =
            self.0.run_mobile_plugin("pickImportFile", serde_json::json!({}))?;

        let bytes = match resp.data {
            Some(encoded) => Some(
                base64::decode(encoded)
                    .map_err(|e| crate::Error::from(e))?
            ),
            None => None,
        };

        Ok(bytes)
    }


}