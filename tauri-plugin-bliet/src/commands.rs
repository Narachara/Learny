use tauri::{AppHandle, command, Runtime };
use crate::Result;
use crate::BlietExt;
use shared::FileResponse;


#[tauri::command]
pub(crate) async fn pick_image<R: Runtime>(
    app: AppHandle<R>,
) -> crate::Result<Option<FileResponse>> {
    app.bliet().pick_image().await
}

#[tauri::command]
pub(crate) async fn pick_archive<R: Runtime>(
    app: AppHandle<R>,
) -> crate::Result<Option<FileResponse>> {
    app.bliet().pick_archive().await
}