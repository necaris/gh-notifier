use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

pub(crate) fn config_file_path() -> PathBuf {
    let base = env::var("XDG_CONFIG_HOME")
        .ok()
        .and_then(|d| Some(PathBuf::from(d)))
        .or(env::home_dir().and_then(|d| Some(d.join(".config"))))
        .unwrap_or_default();
    // TODO: ensure the directory exists and is readable
    base.join("gh-notifier.json")
}

// $XDG_STATE_HOME defines the base directory relative to which user-specific state files should be stored. If $XDG_STATE_HOME is either not set or empty, a default equal to $HOME/.local/state should be used.
pub(crate) fn state_file_path() -> PathBuf {
    let base = env::var("XDG_STATE_HOME")
        .ok()
        .and_then(|d| Some(PathBuf::from(d)))
        .or(env::home_dir().and_then(|d| Some(d.join(".local").join("state"))))
        .unwrap_or_default();
    // TODO: ensure the directory exists and is readable
    base.join("gh-notifier.json")
}
