#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("BASS error: {0} ({1})")]
    Bass(String, String),
    #[error("Source for C string contains NULL byte")]
    FfiNul(#[from] std::ffi::NulError),
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Failed to build logger configuration")]
    LoggerConfig(#[from] log4rs::config::runtime::ConfigErrors),
    #[error("Failed to initialize logger")]
    LoggerSet(#[from] log::SetLoggerError),
    #[error("Failed to strip path prefix")]
    StripPrefix(#[from] std::path::StripPrefixError),
    #[error("Failed to deserialize Toml")]
    TomlDeserialize(#[from] toml::de::Error),
    #[error("Failed to serialize Toml")]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("UTF-8 error")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Source for UTF-16 string contains NULL byte")]
    Utf16Nul(#[from] widestring::error::ContainsNul<u16>),
}

pub type Result<T> = std::result::Result<T, Error>;
