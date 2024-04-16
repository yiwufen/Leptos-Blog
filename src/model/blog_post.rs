use std::default;


use serde::{Deserialize, Serialize};

#[cfg(feature="hydrate")]
use chrono::{Local, NaiveDateTime};
#[cfg(feature="ssr")]
use sqlx::FromRow;
#[cfg(feature="ssr")]
use sqlx::types::chrono::{NaiveDateTime, Local};

#[cfg_attr(feature="ssr", derive(Serialize, Deserialize, Debug, Clone, FromRow))]
#[cfg_attr(feature="hydrate", derive(Clone, Serialize, Deserialize, Debug))]
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