use std::ffi::c_void;

use bass_sys::{
    BASS_ChannelPlay, BASS_ChannelSetSync, BASS_ChannelSlideAttribute, BASS_StreamCreateFile,
    BASS_StreamFree, BASS_ATTRIB_VOL, BASS_SAMPLE_LOOP, BASS_UNICODE, HSTREAM, SYNCPROC,
};
use log::error;
use widestring::U16CString;

use crate::Result;

use super::error::get_bass_error;

pub struct BassStream {
    file: U16CString,
    handle: HSTREAM,
}

impl BassStream {
    pub fn from_file(path: &str, loop_stream: bool) -> Result<Self> {
        let mut result = Self {
            file: U16CString::from_str(path)?,
            handle: 0,
        };
        result.create_stream(loop_stream)?;

        Ok(result)
    }

    pub fn free(&mut self) -> Result<()> {
        if BASS_StreamFree(self.handle) == 0 {
            get_bass_error("Failed to free stream")?;
        }
        self.handle = 0;

        Ok(())
    }

    pub fn play(&self, restart: bool) -> Result<()> {
        if BASS_ChannelPlay(self.handle, restart.into()) == 0 {
            get_bass_error("Failed to play stream")?;
        }

        Ok(())
    }

    pub fn set_sync(
        &self,
        sync_type: u32,
        parameter: u64,
        proc: *mut SYNCPROC,
        user: *mut c_void,
    ) -> Result<()> {
        if BASS_ChannelSetSync(self.handle, sync_type, parameter, proc, user) == 0 {
            get_bass_error("Failed to set device for stream")?;
        }

        Ok(())
    }

    pub fn slide_volume(&self, volume: f32, time: u32) -> Result<()> {
        if BASS_ChannelSlideAttribute(self.handle, BASS_ATTRIB_VOL, volume, time) == 0 {
            get_bass_error("Failed to slide volume for stream")?;
        }

        Ok(())
    }

    fn create_stream(&mut self, loop_stream: bool) -> Result<()> {
        let mut flags = BASS_UNICODE;
        if loop_stream {
            flags |= BASS_SAMPLE_LOOP;
        }
        let handle = BASS_StreamCreateFile(0, self.file.as_ptr().cast::<c_void>(), 0, 0, flags);

        if handle == 0 {
            get_bass_error("Failed to create stream")?;
        }

        self.handle = handle;

        Ok(())
    }
}

impl Drop for BassStream {
    fn drop(&mut self) {
        if let Err(e) = self.free() {
            error!("{:#?}", e);
        }
    }
}
