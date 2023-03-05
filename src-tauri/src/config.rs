use std::{
    fs::{read_to_string, write},
    mem::swap,
    path::{Path, PathBuf},
};

use anyhow::Result;
use getset::{CopyGetters, Getters};
use log::error;
use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    thread_rng, Rng,
};
use serde::{Deserialize, Serialize};

use crate::constants::CONFIG_PATH;

#[derive(Clone, CopyGetters, Deserialize, Getters, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[getset(get = "pub")]
    audio_device: String,
    #[getset(get_copy = "pub")]
    music: bool,
    #[getset(get_copy = "pub")]
    reverse_chance: f64,
    speed_reduce_max: f64,
    speed_reduce_min: f64,
    #[getset(get_copy = "pub")]
    speed_slow_limit: f64,
    speed_slow_reduce_max: f64,
    speed_slow_reduce_min: f64,
    speed_start_max: f64,
    speed_start_min: f64,
    speed_stop_max: f64,
    speed_stop_min: f64,
    #[getset(get_copy = "pub")]
    start_fullscreen: bool,
    #[getset(get_copy = "pub")]
    volume: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            audio_device: String::new(),
            music: true,
            reverse_chance: 0.25,
            speed_reduce_max: 0.05,
            speed_reduce_min: 0.03,
            speed_slow_limit: 1.0,
            speed_slow_reduce_max: 0.001,
            speed_slow_reduce_min: 0.01,
            speed_start_max: 5.0,
            speed_start_min: 4.5,
            speed_stop_max: 0.5,
            speed_stop_min: 0.25,
            start_fullscreen: false,
            volume: 100,
        }
    }
}

impl Config {
    pub(crate) fn load() -> Self {
        let config_path = config_path();
        let mut result = if config_path.is_file() {
            match Config::read_config(config_path.as_path()) {
                Ok(config) => config,
                Err(e) => {
                    error!("Failed to load config, using default. Error: {:#?}", e);
                    Self::default()
                }
            }
        } else {
            Self::default()
        };

        if result.speed_reduce_max < result.speed_reduce_min {
            swap(&mut result.speed_reduce_max, &mut result.speed_reduce_min);
        }

        if result.speed_slow_reduce_max < result.speed_slow_reduce_min {
            swap(
                &mut result.speed_slow_reduce_max,
                &mut result.speed_slow_reduce_min,
            );
        }

        if result.speed_start_max < result.speed_start_min {
            swap(&mut result.speed_start_max, &mut result.speed_start_min);
        }

        if result.speed_stop_max < result.speed_stop_min {
            swap(&mut result.speed_stop_max, &mut result.speed_stop_min);
        }

        result
    }

    pub(crate) fn save(&self) -> Result<()> {
        write(config_path(), toml::to_string(self)?)?;

        Ok(())
    }

    pub(crate) fn speed_reduce(&self) -> f64 {
        random_from_range(self.speed_reduce_min..self.speed_reduce_max)
    }

    pub(crate) fn speed_slow_reduce(&self) -> f64 {
        random_from_range(self.speed_slow_reduce_min..self.speed_slow_reduce_max)
    }

    pub(crate) fn speed_start(&self) -> f64 {
        random_from_range(self.speed_start_min..self.speed_start_max)
    }

    pub(crate) fn speed_stop(&self) -> f64 {
        random_from_range(self.speed_stop_min..self.speed_stop_max)
    }

    fn read_config(config_path: &Path) -> Result<Self> {
        Ok(toml::from_str(read_to_string(config_path)?.as_str())?)
    }
}

fn config_path() -> PathBuf {
    let mut result = CONFIG_PATH.clone();
    result.push("config.toml");
    result
}

fn random_from_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    let mut rng = thread_rng();
    rng.gen_range(range)
}
