use std::process::Command;

use miniserde::{json, Deserialize, Serialize};
use tinyget::{Request, Response};

mod config;
mod models;
mod xdg;

// TODO: alerting template strings: DOCUMENT!
// TODO: handle errors here, e.g. unknown fields
fn fill_value_if_necessary(context: &models::Notification, candidate: &str) -> String {
    if candidate.starts_with("{") && candidate.ends_with("}") {
        // Maybe one day do something fancy with introspecting the context type?
        match candidate.trim_start_matches('{').trim_end_matches('}') {
            "id" => context.id.to_owned(),
            "unread" => context.unread.to_string(),
            "reason" => context.reason.to_owned(),
            "updated_at" => context.updated_at.to_owned(),
            "subject.title" => context.subject.title.to_owned(),
            "subject.url" => context.subject.url.to_owned(),
            "subject.type" => context.subject.type_.to_owned(),
            _ => candidate.to_owned(),
        }
    } else {
        candidate.to_owned()
    }
}

fn main() {
    // -- maybe one day, be an OAuth app
    // TODO: store last check time, use in If-Modified-Since
    // TODO: obey X-Poll-Interval
    // TODO: neat and clean errors
    // TODO: log somewhere debuggable
    let cfg = config::load().expect("Could not load all configuration fields!");

    let response = Request::new("https://api.github.com/notifications")
        .with_header("User-Agent", "gh-notifier 0.1")
        .with_header("Authorization", format!("Bearer {}", cfg.token))
        .send();
    if let Ok(res) = response {
        let b = res.as_str().unwrap();
        let d: Vec<models::Notification> =
            json::from_str(b).expect(&format!("Could not parse {b} as JSON!"));
        let (program, args) = cfg.command;
        let mut cmd = Command::new(program);
        for notif in d {
            cmd.args(args.iter().map(|a| fill_value_if_necessary(&notif, a)))
                .status()
                .expect("Failed to execute process");
        }
    } else {
        eprintln!("No happy");
    }
}
