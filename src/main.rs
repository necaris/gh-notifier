use std::process::Command;

use miniserde::json;
use tinyget::Request;

mod config;
mod models;
mod xdg;

struct State {
    last_poll,
    last_unread_id,
    next_allowable_poll,

}

// TODO next: long-running process time! Maybe one day with ability to
// autospawn to background -- for now, do it `goimapnotify` style

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

    let response = Request::new("https://api.github.com/notifications?all=true")
        .with_header("User-Agent", "gh-notifier 0.1")
        .with_header("Authorization", format!("Bearer {}", conf.token))
        .send();
    if let Ok(res) = response {
        let res_string = res
            .as_str()
            .expect("Could somehow not format response as string");
        let notifications: Vec<models::Notification> =
            json::from_str(res_string).expect(&format!("Could not parse {res_string} as JSON!"));
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
                .expect("Failed to execute process");
        }
    } else {
        eprintln!("No happy");
    }
}
