use serde::{Deserialize, Serialize};
use tauri::{
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime,
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

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Latest {
    /// main build version
    pub main_version: String,
    /// code push version
    pub current_version: String,
    pub hash: String,
    /// falback version
    pub fallback_version: String,
}

impl Latest {
    fn new<R: Runtime>(app: &AppHandle<R>) -> Self {
        let version = app.config().version.as_ref();

        Latest {
            main_version: version.unwrap().to_string(),
            hash: String::from("default_hash"),
            current_version: version.unwrap().to_string(),
            fallback_version: version.unwrap().to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct S3Config {
    pub bucket: String,
    pub region: String,
    pub access_key: String,
    pub secret_access_key: String,
}

// Define the plugin config
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub aws: S3Config,
    pub download_url: String,
}

pub fn init<R: Runtime>() -> TauriPlugin<R, Option<Config>> {
    Builder::<R, Option<Config>>::new("codepush")
        .invoke_handler(tauri::generate_handler![commands::ping])
        .setup(|app, api| {
            #[cfg(mobile)]
            let codepush = mobile::init(app, api)?;
            #[cfg(desktop)]
            let codepush = desktop::init(app, api)?;

            app.manage(codepush);
            println!("Codepush plugin initialized");
            Ok(())
        })
        .build()
}
