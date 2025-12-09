use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::BlietExt;


#[command]
pub(crate) async fn pick_image<R: Runtime>(app: AppHandle<R>) -> Result<String> {
    app.bliet().pick_image()
}
