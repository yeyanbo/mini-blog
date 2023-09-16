use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use chrono::NaiveDate;
use axum_template::engine::Engine;
use handlebars::Handlebars;
use serde::Serialize;

pub type AppEngine = Engine<Handlebars<'static>>;

#[derive(Clone)]
pub struct AppState {
    pub engine: AppEngine,
    pub cache: Arc<Mutex<Cache>>,
}

impl AppState {
    pub fn new(engine: AppEngine) -> Self {
        AppState {
            engine,
            cache: Arc::new(Mutex::new(Cache::new())),
        }
    }
}

#[derive(Clone)]
pub struct Cache {
    pub metadata: HashMap<String, PostMetadata>,
    pub category_cloud: HashSet<String>,
    pub tag_cloud: HashSet<String>,
    pub lasted_update: Option<i64>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            metadata: HashMap::new(),
            category_cloud: HashSet::new(),
            tag_cloud: HashSet::new(),
            lasted_update: None,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct PostMetadata {
    pub name: String,
    pub title: String,
    pub date: Option<NaiveDate>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
}

impl Default for PostMetadata {
    fn default() -> Self {
        PostMetadata {
            name: "".to_string(),
            title: "".to_string(),
            date: None,
            categories: vec!["Uncategorized".to_string()],
            tags: vec![],
        }

    }
}

#[derive(Serialize, Default)]
pub struct Post {
    pub title: String,
    pub date: Option<NaiveDate>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub content: String,
}

impl From<PostMetadata> for Post {
    fn from(value: PostMetadata) -> Self {
        Post {
            title: value.title,
            date: value.date,
            categories: value.categories,
            tags: value.tags,
            content: "".to_string(),
        }
    }
}

#[derive(Clone, Serialize, Default)]
pub struct PostMetadataView {
    pub name: String,
    pub title: String,
    pub date: Option<NaiveDate>,
    pub categories: String,
    pub tags: String,
}

impl From<&PostMetadata> for PostMetadataView {
    fn from(value: &PostMetadata) -> Self {
        PostMetadataView {
            name: value.name.clone(),
            title: value.title.clone(),
            date: value.date.clone(),
            categories: value.categories.iter().map(|c| format!("<span class='category'>{0}</span>",c)).collect::<Vec<String>>().join(", "),
            tags: value.tags.iter().map(|c| format!("<span class='category'>{0}</span>",c)).collect::<Vec<String>>().join(", "),
        }
    }
}

#[derive(Serialize)]
pub struct PostList {
    pub filter: Option<String>,
    pub category_cloud: HashSet<String>,
    pub tag_cloud: HashSet<String>,
    pub metadata: Vec<PostMetadataView>,
}

#[derive(Serialize)]
pub struct PostView {
    pub category_cloud: HashSet<String>,
    pub post: Post,
}

pub enum PostError {
    InvalidPostError,
}