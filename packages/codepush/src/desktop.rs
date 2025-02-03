use serde::Deserialize;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use std::env;
use std::sync::Mutex;
use tauri::async_runtime::spawn;
use tauri::{Manager, State, Url, WebviewUrl, WebviewWindowBuilder};
use tokio::time::{sleep, Duration};

use crate::{models::*, Config, Latest};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CodePushPluginState {
    frontend_task: bool,
    backend_task: bool,
    config: Config,
}

#[tauri::command]
async fn set_complete<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, Mutex<CodePushPluginState>>,
    task: String,
) -> Result<(), ()> {
    let mut state_lock = state.lock().unwrap();
    match task.as_str() {
        "frontend" => state_lock.frontend_task = true,
        "backend" => state_lock.backend_task = true,
        "config" => state_lock.config = Config::default(),
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
    let path = env::current_dir().unwrap().display().to_string();
    let file_path = format!("file:///{}/../dist/splashscreen.html", path); // 절대 경로 사용
    let initial_url = WebviewUrl::External(Url::parse(file_path.as_str()).unwrap());
    let splash_window = WebviewWindowBuilder::new(&app, "splashscreen", initial_url)
        .title("Splashscreen")
        .visible(true)
        .build()
        .unwrap();

    let current_url = splash_window.url();
    println!("Webview is currently looking at: {}", current_url.unwrap());

    splash_window.show().unwrap();
    println!("Splashscreen window created!");

    sleep(Duration::from_secs(2)).await;
    // NOTE: 코드 푸시 받는 거 구현해야함.
    // sleep(Duration::from_secs(100000)).await;
    println!("Backend setup task completed!");
    set_complete(
        app.clone(),
        app.state::<Mutex<CodePushPluginState>>(),
        "backend".to_string(),
    )
    .await?;
    Ok(())
}

pub fn init<R: Runtime>(
    app: &AppHandle<R>,
    _api: PluginApi<R, Option<Config>>,
) -> crate::Result<Codepush<R>> {
    let default_config = Config::default();
    let latest = Latest::new(app);
    let config = _api.config().as_ref().unwrap_or(&default_config).clone();

    println!("init codepush , {:?}", latest.current_version);

    app.manage(Mutex::new(CodePushPluginState {
        frontend_task: true,
        backend_task: false,
        config: config,
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
