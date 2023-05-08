use std::os::unix::process::ExitStatusExt;
use std::thread;
use std::time::Duration;
use std::{process::Command, process::ExitStatus, time::Instant};

use miniserde::json;
use tinyget::{Request, Response};

mod config;
mod models;
mod xdg;

struct State {
    last_poll: Instant,
    last_unread_id: u64,
    next_allowable_poll: Instant,
}

// TODO next: long-running process time! Maybe one day with ability to
// autospawn to background -- for now, do it `goimapnotify` style

fn poll(token: &str) -> (Instant, Option<Vec<models::Notification>>) {
    // TODO: return next allowable poll so we don't have ot
    let response = Request::new("https://api.github.com/notifications?all=true")
        .with_header("User-Agent", "gh-notifier 0.1")
        .with_header("Authorization", format!("Bearer {}", token))
        .send();

    if response.is_err() {
        eprintln!("Error response from API: {:?}", response);
        return (Instant::now(), None);
    }

    let res = response.unwrap();
    let next_poll = match res.headers.get("x-poll-interval") {
        None => Instant::now(),
        Some(interval_value) => match interval_value.parse::<u64>() {
            Ok(interval) => Instant::now() + Duration::from_secs(interval),
            Err(_) => Instant::now(),
        },
    };
    let r2 = res.as_str();
    if r2.is_err() {
        eprintln!("  Could not parse API response as string: {:?}", res);
        return (next_poll, None);
    }
    let s = r2.unwrap();
    match json::from_str::<Vec<models::Notification>>(s) {
        Ok(notifications) => (next_poll, Some(notifications)),
        Err(_) => {
            eprintln!("Could not parse {:?} as JSON", s);
            (next_poll, None)
        }
    }
}

// TODO: handle errors here, e.g. unknown fields
fn main() {
    // -- maybe one day, be an OAuth app
    // TODO: store last check time, use in If-Modified-Since
    // TODO: obey X-Poll-Interval
    // TODO: neat and clean errors
    // TODO: log somewhere debuggable
    // NOTE: would be nice to have an on-click activity
    // TODO: also, an emacs mode to listen to it, call it, listen to a unix socket
    // TODO only notify for unreads? And on-click of notification, go to
    // GH notifs page? Make this an option, maybe?
    // TODO maybe some way to tell `gh-notifier` to mark certain IDs as read?
    let conf = config::load().expect("Could not load all configuration fields!");

    let interval = Duration::from_secs(conf.poll_interval.into());
    let mut state = State {
        last_poll: Instant::now(),
        last_unread_id: 0,
        next_allowable_poll: Instant::now(),
    };
    loop {
        let (next_allowable, possible_notifications) = poll(&conf.token);
        state.next_allowable_poll = next_allowable;
        if let Some(notifications) = possible_notifications {
            let mut cmd = Command::new(&conf.command);
            for n in notifications {
                cmd.env("NOTIFICATION_ID", n.id)
                    .env("NOTIFICATION_UNREAD", n.unread.to_string())
                    .env("NOTIFICATION_REASON", n.reason)
                    .env("NOTIFICATION_UPDATED_AT", n.updated_at)
                    .env("NOTIFICATION_TITLE", n.subject.title)
                    .env("NOTIFICATION_URL", n.subject.url)
                    .env("NOTIFICATION_TYPE", n.subject.type_)
                    .status()
                    .unwrap_or_else(|e| {
                        eprintln!("Failed to execute process: {:?}", e);
                        ExitStatus::from_raw(255)
                    });
            }
        }
        thread::sleep();
    }
}
