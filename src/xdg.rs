use std::env;
use std::path::PathBuf;

#[allow(deprecated)]
pub(crate) fn config_file_path() -> PathBuf {
    let base = env::var("XDG_CONFIG_HOME")
        .ok()
        .and_then(|d| Some(PathBuf::from(d)))
        .or(env::home_dir().and_then(|d| Some(d.join(".config"))))
        .unwrap_or_default();
    // TODO: ensure the directory exists and is readable
    base.join("gh-notifier.json")
}

#[allow(deprecated)]
pub(crate) fn state_file_path() -> PathBuf {
    let base = env::var("XDG_STATE_HOME")
        .ok()
        .and_then(|d| Some(PathBuf::from(d)))
        .or(env::home_dir().and_then(|d| Some(d.join(".local").join("state"))))
        .unwrap_or_default();
    // TODO: ensure the directory exists and is readable
    base.join("gh-notifier.json")
}
