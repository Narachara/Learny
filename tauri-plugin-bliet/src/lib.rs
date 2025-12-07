use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Bliet;
#[cfg(mobile)]
use mobile::Bliet;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the bliet APIs.
pub trait BlietExt<R: Runtime> {
  fn bliet(&self) -> &Bliet<R>;
}

impl<R: Runtime, T: Manager<R>> crate::BlietExt<R> for T {
  fn bliet(&self) -> &Bliet<R> {
    self.state::<Bliet<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("bliet")
    .invoke_handler(tauri::generate_handler![commands::ping, commands::pick_image])
    .setup(|app, api| {
      #[cfg(mobile)]
      let bliet = mobile::init(app, api)?;
      #[cfg(desktop)]
      let bliet = desktop::init(app, api)?;
      app.manage(bliet);
      Ok(())
    })
    .build()
}
