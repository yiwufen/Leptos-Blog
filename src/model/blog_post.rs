use std::default;

// #[cfg(feature="hydrate")]
use chrono::{Local, NaiveDateTime};

#[derive(Clone)]
pub struct Post {
    pub id: String,
    pub dt: NaiveDateTime,
    pub image_url: String,
    pub title: String,
    pub text: String,
}

impl default::Default for Post {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            dt: Local::now().naive_local(),
            image_url: "".to_string(),
            title: "".to_string(),
            text: "".to_string(),
        }
    }
}