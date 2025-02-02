use std::fmt::Debug;

use serde::Deserialize;
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

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct S3Config {
    pub bucket: String,
    pub region: String,
    pub access_key: String,
    pub secret_access_key: String,
}

impl Default for S3Config {
    fn default() -> Self {
        S3Config {
            bucket: String::from("default_bucket"),
            region: String::from("default_region"),
            access_key: String::from("default_access_key"),
            secret_access_key: String::from("default_secret_key"),
        }
    }
}
// Define the plugin config
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub aws: S3Config,
    pub download_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            aws: S3Config::default(), // S3Config도 Default를 구현해야 합니다.
            download_url: String::from("default_url"),
        }
    }
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
