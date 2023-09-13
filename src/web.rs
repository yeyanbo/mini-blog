use axum::extract::Path;
use axum::response::IntoResponse;
use axum_template::RenderHtml;
use crate::model::AppEngine;
use crate::blog;

pub async fn index(engine: AppEngine) -> impl IntoResponse {
    // Html("<h1>Hello!</h1><p>Welcome to FHIR Connectathon Platform!</p>")
    let key = "index";

    let post_list = blog::workthrough().unwrap();

    // let data = PostMetadata {
    //     name: "123".to_string(),
    //     title: "Test".to_string(),
    //     date: chrono::NaiveDate::from_ymd_opt(2020, 10, 12),
    //     categories: vec!["test".to_string()],
    //     tags:  vec!["test".to_string()],
    // };

    // let data2 = PostMetadata {
    //     name: "123".to_string(),
    //     title: "Test".to_string(),
    //     date: chrono::NaiveDate::from_ymd_opt(2020, 10, 12),
    //     categories: vec!["test".to_string()],
    //     tags:  vec!["test".to_string()],
    // };

    // let data3 = PostMetadata {
    //     name: "123".to_string(),
    //     title: "Test".to_string(),
    //     date: chrono::NaiveDate::from_ymd_opt(2020, 10, 12),
    //     categories: vec!["test".to_string()],
    //     tags:  vec!["test".to_string()],
    // };

    // let list = PostList {
    //     metadata: vec![data, data2, data3],
    // };

    RenderHtml(key, engine, post_list)
}

pub async fn posts(engine: AppEngine, Path(pid): Path<String>) -> impl IntoResponse {
    // Html("<h1>Hello!</h1><p>Welcome to FHIR Connectathon Platform!</p>")
    let key = "post";

    let post = blog::open_markdown_file(pid).unwrap();

    RenderHtml(key, engine, post)
}