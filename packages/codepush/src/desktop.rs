use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use std::sync::Mutex;
use tauri::async_runtime::spawn;
use tauri::{Manager, State, WebviewUrl, WebviewWindowBuilder};
use tokio::time::{sleep, Duration};

pub struct SetupState {
    frontend_task: bool,
    backend_task: bool,
}

use crate::models::*;

#[tauri::command]
async fn set_complete<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, Mutex<SetupState>>,
    task: String,
) -> Result<(), ()> {
    let mut state_lock = state.lock().unwrap();
    match task.as_str() {
        "frontend" => state_lock.frontend_task = true,
        "backend" => state_lock.backend_task = true,
        _ => panic!("invalid task completed!"),
    }
    if state_lock.backend_task && state_lock.frontend_task {
        let splash_window = app.get_webview_window("splashscreen").unwrap();
        let main_window = app.get_webview_window("main").unwrap();
        splash_window.close().unwrap();
        main_window.show().unwrap();
    }
    Ok(())
}

// An async function that does some heavy setup task
async fn setup<R: Runtime>(app: AppHandle<R>) -> Result<(), ()> {
    // let main_window = app.get_webview_window("main").unwrap();
    // main_window.set_visible_on_all_workspaces(false).unwrap();
    println!("Performing really heavy backend setup task...");
    let initial_url = WebviewUrl::App("splashscreen.html".into());
    let splash_window = WebviewWindowBuilder::new(&app, "splashscreen", initial_url)
        .title("Splashscreen")
        .visible(true)
        .build()
        .unwrap();
    splash_window.show().unwrap();
    println!("Splashscreen window created!");
    // NOTE: 코드 푸시 받는 거 구현해야함.
    sleep(Duration::from_secs(3)).await;
    println!("Backend setup task completed!");
    set_complete(
        app.clone(),
        app.state::<Mutex<SetupState>>(),
        "backend".to_string(),
    )
    .await?;
    Ok(())
}

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Codepush<R>> {
    app.manage(Mutex::new(SetupState {
        frontend_task: true,
        backend_task: false,
    }));
    spawn(setup(app.app_handle().clone()));
    Ok(Codepush(app.clone()))
}

/// Access to the codepush APIs.
pub struct Codepush<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Codepush<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }
}
