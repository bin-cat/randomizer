#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod audio_player;
mod commands;
mod config;
mod constants;
mod error;
mod func;

use std::{
    error::Error,
    fs::{create_dir_all, read},
};

use anyhow::bail;
use log::{error, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
};
use tauri::{
    http::{Request, Response, ResponseBuilder},
    AppHandle, Manager, RunEvent,
};
use tokio::sync::RwLock;

use crate::{
    audio_player::Player,
    commands::{get_audio_devices, get_config, lists, random_bg, roll, set_config, stop},
    config::Config,
    constants::{APP_PATH, CONFIG_PATH, DATA_PATH, ROLL_SOUNDS, STOP_SOUNDS},
    func::load_sound_lists,
};

const LOG_FILE_NAME: &'static str = "randomizer.log";
const PLUGINS_DIR: &'static str = "plugins";

pub struct AppState {
    pub config: RwLock<Config>,
    pub stop_roll: RwLock<bool>,
}

fn main() -> anyhow::Result<()> {
    create_dir_all(CONFIG_PATH.as_path())?;

    init_log()?;

    let config = Config::load();
    let mut plugins_dir = APP_PATH.clone();
    plugins_dir.push(PLUGINS_DIR);
    Player::init(plugins_dir, config.audio_device().as_str(), config.volume()).unwrap();

    if ROLL_SOUNDS.set(load_sound_lists("roll")).is_err() {
        bail!("Failed to load roll sounds");
    }
    if STOP_SOUNDS.set(load_sound_lists("stop")).is_err() {
        bail!("Failed to load stop sounds");
    }

    let state = AppState {
        config: RwLock::new(config),
        stop_roll: RwLock::new(false),
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
            let mut file_path = DATA_PATH.clone();
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

fn init_log() -> anyhow::Result<()> {
    log_panics::init();

    let mut file_path = CONFIG_PATH.clone();
    file_path.push(LOG_FILE_NAME);

    let log_file = FileAppender::builder()
        .append(false)
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} [{l}] {m}{n}",
        )))
        .build(file_path)?;

    let config = log4rs::Config::builder()
        .appender(Appender::builder().build("log_file", Box::new(log_file)))
        .build(
            Root::builder()
                .appender("log_file")
                .build(LevelFilter::Info),
        )?;

    log4rs::init_config(config)?;

    Ok(())
}

fn run_event_handler(app_handle: &AppHandle, event: RunEvent) {
    if let tauri::RunEvent::Ready = event {
        let state = app_handle.state::<AppState>();

        if state.config.blocking_read().start_fullscreen() {
            if let Some(window) = app_handle.get_window("main") {
                if let Err(e) = window.set_fullscreen(true) {
                    error!("Failed to enable full screen mode: {:#?}", e);
                }
            }
        }
    }
}
