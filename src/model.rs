use chrono::NaiveDate;
use axum_template::engine::Engine;
use handlebars::Handlebars;
use axum::extract::FromRef;
use serde::Serialize;

pub type AppEngine = Engine<Handlebars<'static>>;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub engine: AppEngine,
}

#[derive(Serialize, Default)]
pub struct PostMetadata {
    pub name: String,
    pub title: String,
    pub date: Option<NaiveDate>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Serialize, Default)]
pub struct Post {
    pub title: String,
    pub date: Option<NaiveDate>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub content: String,
}

#[derive(Serialize)]
pub struct PostList {
    pub metadata: Vec<PostMetadata>,
}

pub enum PostError {
    InvalidPostError,
}