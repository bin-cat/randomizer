use std::{collections::HashMap, ffi::c_void, fs::create_dir_all, path::PathBuf, time::Duration};

use bass_sys::{BASS_SYNC_END, BASS_SYNC_ONETIME, BASS_SYNC_SLIDE, DWORD, HSYNC, SYNCPROC};
use log::{error, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
};
use rand::{
    random,
    seq::{IteratorRandom, SliceRandom},
};
use tokio::sync::RwLock;

use crate::{
    audio_player::{BassStream, Player},
    constants::{APP_PATH, CONFIG_PATH, LIST_EXTENSION},
    data_path,
    func::{lines_from_file, load_sound_lists},
    Config, Result,
};

const LOG_FILE_NAME: &str = "randomizer.log";
const PLUGINS_DIR: &str = "plugins";

pub struct Randomizer {
    config: Config,
    current_list: RwLock<Option<String>>,
    player: RwLock<Player>,
    roll_sounds: HashMap<String, Vec<PathBuf>>,
    stop_roll: RwLock<bool>,
    stop_sounds: HashMap<String, Vec<PathBuf>>,
}

impl Randomizer {
    pub fn new() -> Result<Self> {
        create_dir_all(CONFIG_PATH.as_path())?;

        init_log()?;

        let config = Config::load();
        let mut plugins_dir = APP_PATH.clone();
        plugins_dir.push(PLUGINS_DIR);
        Player::init(plugins_dir, config.audio_device().as_str(), config.volume())?;

        Ok(Self {
            config,
            current_list: RwLock::new(None),
            player: RwLock::new(Player::new()),
            roll_sounds: load_sound_lists("roll"),
            stop_roll: RwLock::new(false),
            stop_sounds: load_sound_lists("stop"),
        })
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub async fn roll(&self, list_name: &str, names_callback: impl Fn(Vec<String>)) -> Result<()> {
        {
            let mut w = self.current_list.write().await;
            *w = Some(list_name.to_string());

            let mut w = self.stop_roll.write().await;
            *w = false;
        }

        if self.config.music() {
            if let Some(sound_path) = random_sound(&self.roll_sounds, Some(list_name)) {
                let stream = BassStream::from_file(&sound_path.to_string_lossy(), true)?;
                stream.set_sync(
                    BASS_SYNC_SLIDE | BASS_SYNC_ONETIME,
                    0,
                    callback_music as *mut SYNCPROC,
                    self as *const Randomizer as *mut c_void,
                )?;
                {
                    let mut player = self.player.write().await;
                    player.set_stream(Some(stream));
                    player.play(false)?;
                }
            }
        }

        let mut path = data_path();
        path.push("lists");
        path.push(format!("{list_name}.{LIST_EXTENSION}"));

        let items = {
            let mut rng = rand::thread_rng();
            let mut lines = lines_from_file(path);
            lines.shuffle(&mut rng);
            lines
        };

        let mut i: f64 = 0.0;
        let total = items.len() as isize;

        let mut speed = self.config.speed_start();
        let direction = if random::<f64>() < self.config.reverse_chance() {
            -1.0
        } else {
            1.0
        };

        loop {
            let pos = (i.round() as isize).rem_euclid(total);
            names_callback(
                (pos - 2..pos + 3)
                    .map(|x| items[x.rem_euclid(total) as usize].to_string())
                    .collect::<Vec<String>>(),
            );

            tokio::time::sleep(Duration::from_millis(100)).await;
            i += speed * direction;
            speed -= if speed < self.config.speed_slow_limit() {
                self.config.speed_slow_reduce()
            } else {
                self.config.speed_reduce()
            };

            if *self.stop_roll.read().await {
                speed = speed.min(self.config.speed_stop());
            }

            if speed < 0.0 {
                self.player.write().await.fade_out()?;
                break;
            }
        }

        Ok(())
    }

    pub fn set_config(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Player::set_volume(self.config.volume())?;
        Player::set_device(Player::find_device_index(self.config.audio_device())?)?;
        self.config.save()
    }

    pub fn stop_roll(&self) {
        let mut w = self.stop_roll.blocking_write();
        *w = true;
    }

    pub(crate) fn play_stop_sound(&self) -> Result<()> {
        if let Some(sound_path) = random_sound(
            &self.stop_sounds,
            self.current_list.blocking_read().as_deref(),
        ) {
            let stream = BassStream::from_file(&sound_path.to_string_lossy(), false)?;
            stream.set_sync(
                BASS_SYNC_END | BASS_SYNC_ONETIME,
                0,
                callback_sound as *mut SYNCPROC,
                self as *const Randomizer as *mut c_void,
            )?;

            let mut player = self.player.blocking_write();
            player.set_stream(Some(stream));
            player.play(false)?;
        }

        Ok(())
    }

    pub(crate) fn stop_audio(&self) {
        let mut player = self.player.blocking_write();
        player.stop();
    }
}

fn init_log() -> Result<()> {
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

fn callback_music(_handle: HSYNC, _channel: DWORD, _data: DWORD, user: *mut c_void) {
    if user.is_null() {
        return;
    }

    let randomizer: &Randomizer = unsafe { &*(user as *const Randomizer) };
    if let Err(e) = randomizer.play_stop_sound() {
        error!("{:#?}", e);
    }
}

fn callback_sound(_handle: HSYNC, _channel: DWORD, _data: DWORD, user: *mut c_void) {
    if user.is_null() {
        return;
    }

    let randomizer: &Randomizer = unsafe { &*(user as *const Randomizer) };
    randomizer.stop_audio();
}

fn random_sound(
    sounds: &HashMap<String, Vec<PathBuf>>,
    list_name: Option<&str>,
) -> Option<PathBuf> {
    let mut rng = rand::thread_rng();

    let mut playlists = Vec::with_capacity(2);
    if let Some(list_name) = list_name {
        if let Some(playlist) = sounds.get(list_name).map(|x| x.iter()) {
            playlists.push(playlist);
        }
    }
    if let Some(playlist) = sounds.get("").map(|x| x.iter()) {
        playlists.push(playlist);
    }

    for playlist in playlists {
        if let Some(entry) = playlist.choose(&mut rng) {
            return Some(entry.clone());
        }
    }

    None
}
