use miniserde::{json, Deserialize};
use std::env;
use std::fs;
use std::process::Command;

use crate::xdg;

#[derive(Debug)]
pub(crate) struct Config {
    pub token: String,
    pub command: (String, Vec<String>),
    // TODO: add different behavior options, e.g. maximum poll interval,
    // whether to repeat notifications already sent, etc
}

#[derive(Deserialize, Debug, Clone)]
struct PartialConfig {
    token: Option<String>,
    command: Option<String>,
}

fn from_file() -> PartialConfig {
    fs::read_to_string(&xdg::config_file_path())
        .ok()
        .and_then(|s| json::from_str(&s).ok())
        .unwrap_or(PartialConfig {
            token: None,
            command: None,
        })
}

fn from_env() -> PartialConfig {
    PartialConfig {
        token: env::var("GITHUB_TOKEN").ok(),
        command: env::var("GH_NOTIFIER_COMMAND").ok(),
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
    }
}

fn parse_command_string(s: &str) -> Option<(String, Vec<String>)> {
    // TODO: verify this program is callable
    let mut command_words = s.split(' ');
    match command_words.next() {
        Some(program) => {
            let program = program.to_owned();
            let args: Vec<String> = command_words.map(|s| s.to_owned()).collect();
            Some((program, args))
        }
        None => None,
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

    match (token, command) {
        (Some(tkn), Some(cmd_str)) => {
            if let Some(parsed_command) = parse_command_string(&cmd_str) {
                Some(Config {
                    token: tkn.trim().to_owned(),
                    command: parsed_command,
                })
            } else {
                None
            }
        }
        _ => None,
    }
}
