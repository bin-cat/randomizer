use std::{
    collections::HashMap,
    ffi::{c_void, CString},
    path::PathBuf,
    ptr::null_mut,
    time::Duration,
};

use anyhow::Context;
use bass_sys::{BASS_SYNC_END, BASS_SYNC_ONETIME, BASS_SYNC_SLIDE, DWORD, HSYNC, SYNCPROC};
use log::error;
use rand::{
    random,
    seq::{IteratorRandom, SliceRandom},
};
use tauri::Window;

use crate::{
    audio_player::BassStream,
    constants::{PLAYER, ROLL_SOUNDS, STOP_SOUNDS},
    error::Result,
    func::lines_from_file,
    AppState, DATA_PATH,
};

#[tauri::command]
pub async fn roll(
    list_name: String,
    state: tauri::State<'_, AppState>,
    window: Window,
) -> Result<()> {
    {
        let mut w = state.stop_roll.write().await;
        *w = false;
    }

    if state.config.read().await.music() {
        if let Some(sound_path) = random_sound(
            ROLL_SOUNDS
                .get()
                .with_context(|| "Roll sounds are not loaded yet")?,
            Some(&list_name),
        ) {
            let stream = BassStream::from_file(&sound_path.to_string_lossy(), true)
                .with_context(|| "Failed to create BASS stream from file")?;
            stream
                .set_sync(
                    BASS_SYNC_SLIDE | BASS_SYNC_ONETIME,
                    0,
                    callback_music as *mut SYNCPROC,
                    CString::new(list_name.as_str())
                        .with_context(|| "Failed to generate C string from list name")?
                        .into_raw()
                        .cast(),
                )
                .with_context(|| "Failed to set BASS sync function for roll music")?;
            let mut player = PLAYER.write().await;
            player.set_stream(Some(stream));
            player
                .play(false)
                .with_context(|| "Failed to play roll music")?;
        }
    }

    let mut path = DATA_PATH.clone();
    path.push("lists");
    path.push(format!("{list_name}.txt"));

    let items = {
        let mut rng = rand::thread_rng();
        let mut lines = lines_from_file(path);
        lines.shuffle(&mut rng);
        lines
    };

    let mut i: f64 = 0.0;
    let total = items.len() as isize;

    let mut speed = state.config.read().await.speed_start();
    let direction = if random::<f64>() < state.config.read().await.reverse_chance() {
        -1.0
    } else {
        1.0
    };

    loop {
        let pos = (i.round() as isize).rem_euclid(total);
        window
            .emit(
                "wheel-list",
                (pos - 2..pos + 3)
                    .map(|x| items[x.rem_euclid(total) as usize].to_string())
                    .collect::<Vec<String>>(),
            )
            .with_context(|| "Failed to emit 'wheel-list' event")?;

        tokio::time::sleep(Duration::from_millis(100)).await;
        i += speed * direction;
        speed -= if speed < state.config.read().await.speed_slow_limit() {
            state.config.read().await.speed_slow_reduce()
        } else {
            state.config.read().await.speed_reduce()
        };

        if *state.stop_roll.read().await {
            speed = speed.min(state.config.read().await.speed_stop());
        }

        if speed < 0.0 {
            PLAYER
                .write()
                .await
                .fade_out()
                .with_context(|| "Failed to start roll music fade out")?;
            break;
        }
    }

    Ok(window
        .emit("stop", ())
        .with_context(|| "Failed to emit 'stop' signal")?)
}

fn callback_music(_handle: HSYNC, _channel: DWORD, _data: DWORD, user: *mut c_void) {
    let list_name = if user.is_null() {
        None
    } else {
        Some(
            unsafe { CString::from_raw(user.cast()) }
                .to_string_lossy()
                .to_string(),
        )
    };

    if let Err(e) = play_stop_sound(list_name.as_deref()) {
        error!("{:#?}", e);
    }
}

fn callback_sound(_handle: HSYNC, _channel: DWORD, _data: DWORD, _user: *mut c_void) {
    PLAYER.blocking_write().stop();
}

fn play_stop_sound(list_name: Option<&str>) -> Result<()> {
    if let Some(sound_path) = random_sound(
        STOP_SOUNDS
            .get()
            .with_context(|| "Roll sounds are not loaded yet")?,
        list_name,
    ) {
        let stream = BassStream::from_file(&sound_path.to_string_lossy(), false)
            .with_context(|| format!("Failed to load stop sound {:?}", &sound_path))?;
        stream
            .set_sync(
                BASS_SYNC_END | BASS_SYNC_ONETIME,
                0,
                callback_sound as *mut SYNCPROC,
                null_mut(),
            )
            .with_context(|| "Failed to set sync function for stop sound")?;

        let mut player = PLAYER.blocking_write();
        player.set_stream(Some(stream));
        player
            .play(false)
            .with_context(|| "Failed to play stop sound")?;
    }

    Ok(())
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
