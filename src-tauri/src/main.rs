#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod error;

use std::{error::Error, fs::read};

use log::error;
use tauri::{
    http::{Request, Response, ResponseBuilder},
    AppHandle, Manager, RunEvent,
};
use tokio::sync::RwLock;

use randomizer_core::{data_path, Randomizer};

use crate::commands::{get_audio_devices, get_config, lists, random_bg, roll, set_config, stop};

pub struct AppState {
    pub randomizer: RwLock<Randomizer>,
}

fn main() -> anyhow::Result<()> {
    let state = AppState {
        randomizer: RwLock::new(Randomizer::new()?),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_audio_devices,
            get_config,
            lists,
            random_bg,
            roll,
            set_config,
            stop
        ])
        .register_uri_scheme_protocol("data", data_protocol_handler)
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(run_event_handler);

    Ok(())
}

fn data_protocol_handler(
    _app_handle: &AppHandle,
    request: &Request,
) -> Result<Response, Box<dyn Error>> {
    let not_found = ResponseBuilder::new().status(404).body(Vec::new());
    if request.method() != "GET" {
        return not_found;
    }
    let response = ResponseBuilder::new();
    match request.uri().strip_prefix("data://localhost/") {
        Some(path) => {
            let mut file_path = data_path();
            file_path.push(
                percent_encoding::percent_decode(path.as_bytes())
                    .decode_utf8_lossy()
                    .to_string(),
            );
            if !file_path.is_file() {
                return not_found;
            }

            let mime_type = mime_guess::from_path(&file_path).first_or_octet_stream();
            response
                .mimetype(mime_type.essence_str())
                .status(200)
                .body(read(file_path)?)
        }
        None => not_found,
    }
}

fn run_event_handler(app_handle: &AppHandle, event: RunEvent) {
    if let tauri::RunEvent::Ready = event {
        let state = app_handle.state::<AppState>();

        if state.randomizer.blocking_read().config().start_fullscreen() {
            if let Some(window) = app_handle.get_window("main") {
                if let Err(e) = window.set_fullscreen(true) {
                    error!("Failed to enable full screen mode: {:#?}", e);
                }
            }
        }
    }
}
