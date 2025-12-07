use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::BlietExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.bliet().ping(payload)
}


#[command]
pub(crate) async fn pick_image<R: Runtime>(app: AppHandle<R>) -> Result<String> {
    app.bliet().pick_image()
}
