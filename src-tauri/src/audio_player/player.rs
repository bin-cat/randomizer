use std::{
    env::consts::DLL_EXTENSION,
    ffi::{c_void, CStr, CString},
    ops::Drop,
    path::Path,
    ptr::null_mut,
};

use anyhow::{Context, Result};
use bass_sys::{
    BASS_Free, BASS_GetDevice, BASS_GetDeviceInfo, BASS_Init, BASS_PluginFree, BASS_PluginLoad,
    BASS_SetConfig, BASS_SetDevice, BassDeviceInfo, BASS_CONFIG_GVOL_STREAM, BASS_CONFIG_UNICODE,
    BASS_DEVICE_ENABLED,
};
use log::error;
use walkdir::WalkDir;

use super::{error::get_bass_error, BassStream};

pub struct Player {
    stream: Option<BassStream>,
}

impl Player {
    pub fn new() -> Self {
        Self { stream: None }
    }

    pub fn find_device_index(name: &str) -> Result<u32> {
        let mut i = 1;
        let mut device_info = BassDeviceInfo::new(null_mut::<c_void>(), null_mut::<c_void>(), 0);

        if name.is_empty() {
            return Ok(1);
        }

        loop {
            if BASS_GetDeviceInfo(i, &mut device_info) == 0 {
                break;
            }

            if device_info.driver.is_null() {
                continue;
            }

            let c_str: &CStr = unsafe { CStr::from_ptr(device_info.driver.cast()) };
            if c_str.to_str()? == name {
                return Ok(i as u32);
            }

            i += 1;
        }

        Ok(1)
    }

    pub fn set_device(device: u32) -> Result<()> {
        let old_device = BASS_GetDevice();

        if device == old_device {
            return Ok(());
        }

        Player::init_device(device as i32)?;

        if BASS_SetDevice(old_device) == 0 {
            get_bass_error("Failed to set default device")?;
        }
        if BASS_Free() == 0 {
            get_bass_error("Failed to free device")?;
        }
        if BASS_SetDevice(device) == 0 {
            get_bass_error("Failed to set default device")?;
        }

        Ok(())
    }

    pub fn set_volume(volume: u32) -> Result<()> {
        if BASS_SetConfig(BASS_CONFIG_GVOL_STREAM, volume.min(100) * 100) == 0 {
            get_bass_error("Failed to set volume")?;
        }
        Ok(())
    }

    pub fn init(plugins_dir: impl AsRef<Path>, device: &str, volume: u32) -> Result<()> {
        if BASS_SetConfig(BASS_CONFIG_UNICODE, 1) == 0 {
            get_bass_error("Failed to enable Unicode for device information")?;
        }
        Player::load_plugins(plugins_dir)?;
        Player::init_device(Player::find_device_index(device)? as i32)?;
        Player::set_volume(volume)?;

        Ok(())
    }

    pub fn list_devices() -> Result<Vec<(String, String)>> {
        let mut result = vec![];

        let mut i = 1;
        let mut device_info: BassDeviceInfo = BassDeviceInfo::new(null_mut(), null_mut(), 0);
        loop {
            if BASS_GetDeviceInfo(i, &mut device_info) == 0 {
                break;
            }

            if device_info.flags & BASS_DEVICE_ENABLED != 0 {
                result.push((
                    unsafe { CStr::from_ptr(device_info.driver.cast()) }
                        .to_str()?
                        .to_string(),
                    unsafe { CStr::from_ptr(device_info.name.cast()) }
                        .to_str()?
                        .to_string(),
                ));
            }

            i += 1;
        }

        Ok(result)
    }

    pub fn fade_out(&mut self) -> Result<()> {
        if let Some(stream) = &self.stream {
            stream.slide_volume(-1.0, 1000)?;
        }

        Ok(())
    }

    pub fn play(&self, restart: bool) -> Result<()> {
        if let Some(stream) = &self.stream {
            stream.play(restart)?;
        }

        Ok(())
    }

    pub fn set_stream(&mut self, stream: Option<BassStream>) {
        self.stream = stream;
    }

    pub fn stop(&mut self) {
        self.stream = None;
    }

    fn init_device(device: i32) -> Result<()> {
        if BASS_Init(device, 44100, 0, null_mut::<c_void>(), null_mut::<c_void>()) == 0 {
            get_bass_error("Failed to initialize audio device")?;
        }

        Ok(())
    }

    fn load_plugins(plugins_dir: impl AsRef<Path>) -> Result<()> {
        for file_path in WalkDir::new(plugins_dir)
            .follow_links(false)
            .max_depth(1)
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|file| {
                file.path()
                    .extension()
                    .map_or(false, |ext| ext == DLL_EXTENSION)
            })
        {
            let c_path = CString::new(
                file_path
                    .path()
                    .to_str()
                    .context("Failed to convert path to string")?,
            )?;
            let handle = BASS_PluginLoad(c_path.as_ptr().cast::<c_void>(), 0);
            if handle == 0 {
                get_bass_error("Failed to load BASS plugin")?;
            }
        }

        Ok(())
    }

    fn free(&mut self) -> Result<()> {
        self.stream = None;

        if BASS_PluginFree(0) == 0 {
            get_bass_error("Failed to free plugins")?;
        }

        if BASS_Free() == 0 {
            get_bass_error("Failed to free BASS")?;
        }

        Ok(())
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        if let Err(e) = self.free() {
            error!("{:#?}", e);
        }
    }
}
