use miniserde::{json, Deserialize};
use std::env;
use std::fs;
use std::process::Command;

use crate::xdg;

#[derive(Debug)]
pub(crate) struct Config {
    pub token: String,
    // TODO: make this some kind of fancy thing, rather than
    // a single script we call
    pub command: String,
    // TODO: add different behavior options, e.g. maximum poll interval,
    // whether to repeat notifications already sent, etc
    pub unread_only: bool,
}

#[derive(Deserialize, Debug, Clone)]
struct PartialConfig {
    token: Option<String>,
    command: Option<String>,
    unread_only: Option<bool>,
}

fn from_file() -> PartialConfig {
    fs::read_to_string(&xdg::config_file_path())
        .ok()
        .and_then(|s| json::from_str(&s).ok())
        .unwrap_or(PartialConfig {
            token: None,
            command: None,
            unread_only: None,
        })
}

fn from_env() -> PartialConfig {
    PartialConfig {
        token: env::var("GITHUB_TOKEN").ok(),
        command: env::var("GH_NOTIFIER_COMMAND").ok(),
        unread_only: env::var("GH_NOTIFIER_UNREAD_ONLY")
            .ok()
            .map(|s| s == "true"),
    }
}

fn get_git_config_value(key: &str) -> Option<String> {
    let output = Command::new("git").args(["config", "--get", key]).output();
    match output {
        Ok(raw) => {
            let result = String::from_utf8(raw.stdout);
            match result {
                Ok(s) => {
                    if s == "" {
                        None
                    } else {
                        Some(s)
                    }
                }
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}

fn from_git_config() -> PartialConfig {
    PartialConfig {
        token: get_git_config_value("github.oauth-token"),
        command: get_git_config_value("github.notifier-command"),
        unread_only: get_git_config_value("github.notifier-unread-only").map(|s| s == "true"),
    }
}

// TODO: make this more efficient, don't attempt to load things if a previous
// loader has the value
// TODO: make this return Result and have nice Errs
pub(crate) fn load() -> Option<Config> {
    let f = from_file();
    let e = from_env();
    let g = from_git_config();
    let token = e.token.or(f.token.or(g.token));
    let command = e.command.or(f.command.or(g.command));
    let unread_only = e.unread_only.or(f.unread_only.or(g.unread_only));

    match (token, command, unread_only) {
        (Some(tkn), Some(cmd_str), uo) => {
            // TODO: verify that `command` is callable
            Some(Config {
                token: tkn.trim().to_owned(),
                command: cmd_str.trim().to_owned(),
                unread_only: uo.unwrap_or(false),
            })
        }
        _ => None,
    }
}
