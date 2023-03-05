use anyhow::{bail, Result};
use bass_sys::BASS_ErrorGetCode;

pub(crate) fn get_bass_error(message: &str) -> Result<()> {
    let error_code = BASS_ErrorGetCode();
    if error_code == 0 {
        return Ok(());
    }

    let message = message.to_string();
    let error_text = match BASS_ErrorGetCode() {
        1 => "Memory error",
        2 => "Can't open the file",
        3 => "Can't find a free/valid driver",
        4 => "The sample buffer was lost",
        5 => "Invalid handle",
        6 => "Unsupported sample format",
        7 => "Invalid position",
        8 => "BASS_Init has not been successfully called",
        9 => "BASS_Start has not been successfully called",
        10 => "SSL/HTTPS support isn't available",
        14 => "Already initialized/paused/whatever",
        17 => "File does not contain audio",
        18 => "Can't get a free channel",
        19 => "An illegal type was specified",
        20 => "An illegal parameter was specified",
        21 => "No 3D support",
        22 => "No EAX support",
        23 => "Illegal device number",
        24 => "Not playing",
        25 => "Illegal sample rate",
        27 => "The stream is not a file stream",
        29 => "No hardware voices available",
        31 => "The MOD music has no sequence data",
        32 => "No internet connection could be opened",
        33 => "Couldn't create the file",
        34 => "Effects are not available",
        37 => "Requested data/action is not available",
        38 => "The channel is/isn't a \"decoding channel\"",
        39 => "A sufficient DirectX version is not installed",
        40 => "Connection timed out",
        41 => "Unsupported file format",
        42 => "Unavailable speaker",
        43 => "Invalid BASS version (used by add-ons)",
        44 => "Codec is not available/supported",
        45 => "The channel/file has ended",
        46 => "The device is busy",
        47 => "Unstreamable file",
        _ => "Unknown",
    };

    bail!("BASS error: {} ({})", message, error_text);
}
