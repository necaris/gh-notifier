use miniserde::{json, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct User {
    pub login: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Notification {
    pub id: String,
    pub unread: bool,
    pub reason: String,
    pub updated_at: String,
    pub last_read_at: Option<String>,
    pub subject: NotificationSubject,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct NotificationSubject {
    pub title: String,
    pub url: String,
    #[serde(rename = "type")]
    pub type_: String, // TODO: an enum
}
