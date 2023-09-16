use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_template::RenderHtml;
use crate::model::{AppState, PostList, PostMetadataView, PostView};
use crate::blog;

pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let key = "index";

    let mut cache = state.cache.lock().expect("mutex was poisoned");

    // 判断是否需要重新加载Markdown文件夹
    if cache.lasted_update.is_none() || blog::need_reload(cache.lasted_update.unwrap()) {
        blog::work_through(&mut cache);
    }

    let mut metadata = cache.metadata.iter()
        .map(|entry| PostMetadataView::from(entry.1))
        .collect::<Vec<PostMetadataView>>();

    metadata.sort_by(|x,y| {
        let date_a = x.date.unwrap();
        let date_b = y.date.unwrap();

        date_b.cmp(&date_a)
    });

    cache.category_cloud.remove("Uncategorized");

    RenderHtml(key, state.engine.clone(), PostList {
        filter: None,
        category_cloud: cache.category_cloud.clone(),
        tag_cloud: cache.tag_cloud.clone(),
        metadata,
    })
}

pub async fn posts(State(state): State<AppState>, Path(pid): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    let key = "post";

    let mut cache = state.cache.lock().expect("mutex was poisoned");

    // 判断是否需要重新加载Markdown文件夹
    if cache.lasted_update.is_none() || blog::need_reload(cache.lasted_update.unwrap()) {
        blog::work_through(&mut cache);
    }

    if let Some(meta) = cache.metadata.get(&pid) {
        let post = blog::open_markdown_file( pid, meta.clone()).unwrap();

        cache.category_cloud.remove("Uncategorized");
        return Ok(RenderHtml(key,
                             state.engine,
                             PostView {
                                    category_cloud: cache.category_cloud.clone(),
                                    post,
                                }
        ))
    }

    Err(StatusCode::NOT_FOUND)
}

pub async fn category(State(state): State<AppState>, Path(cid): Path<String>) -> impl IntoResponse {
    let key = "category";

    let mut cache = state.cache.lock().expect("mutex was poisoned");

    // 判断是否需要重新加载Markdown文件夹
    if cache.lasted_update.is_none() || blog::need_reload(cache.lasted_update.unwrap()) {
        blog::work_through(&mut cache);
    }

    let mut metadata = cache.metadata.iter()
        .filter(|entry| entry.1.categories.contains(&cid))
        .map(|entry| PostMetadataView::from(entry.1))
        .collect::<Vec<PostMetadataView>>();

    // 将博客按照时间进行排序
    metadata.sort_by(|x,y| {
        let date_a = x.date.unwrap();
        let date_b = y.date.unwrap();

        date_b.cmp(&date_a)
    });

    cache.category_cloud.remove("Uncategorized");

    RenderHtml(key, state.engine.clone(), PostList {
        filter: Some(cid.clone()),
        category_cloud: cache.category_cloud.clone(),
        tag_cloud: cache.tag_cloud.clone(),
        metadata,
    })
}

pub async fn tag(State(state): State<AppState>, Path(tid): Path<String>) -> impl IntoResponse {
    let key = "tag";

    let mut cache = state.cache.lock().expect("mutex was poisoned");

    // 判断是否需要重新加载Markdown文件夹
    if cache.lasted_update.is_none() || blog::need_reload(cache.lasted_update.unwrap()) {
        blog::work_through(&mut cache);
    }

    let mut metadata = cache.metadata.iter()
        .filter(|entry| entry.1.tags.contains(&tid))
        .map(|entry| PostMetadataView::from(entry.1))
        .collect::<Vec<PostMetadataView>>();

    // 将博客按照时间进行排序
    metadata.sort_by(|x,y| {
        let date_a = x.date.unwrap();
        let date_b = y.date.unwrap();

        date_b.cmp(&date_a)
    });

    cache.category_cloud.remove("Uncategorized");

    RenderHtml(key, state.engine.clone(), PostList {
        filter: Some(tid.clone()),
        category_cloud: cache.category_cloud.clone(),
        tag_cloud: cache.tag_cloud.clone(),
        metadata,
    })
}