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
use desktop::Codepush;
#[cfg(mobile)]
use mobile::Codepush;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the codepush APIs.
pub trait CodepushExt<R: Runtime> {
  fn codepush(&self) -> &Codepush<R>;
}

impl<R: Runtime, T: Manager<R>> crate::CodepushExt<R> for T {
  fn codepush(&self) -> &Codepush<R> {
    self.state::<Codepush<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("codepush")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let codepush = mobile::init(app, api)?;
      #[cfg(desktop)]
      let codepush = desktop::init(app, api)?;
      app.manage(codepush);
      Ok(())
    })
    .build()
}
