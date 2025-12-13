use tauri::{AppHandle, command, Runtime };
use crate::Result;
use crate::BlietExt;
use shared::ImageResponse;


#[command]
pub(crate) async fn pick_image<R: Runtime>(app: AppHandle<R>) -> crate::Result<ImageResponse> {
    app.bliet().pick_image().await
}