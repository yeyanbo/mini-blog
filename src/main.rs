pub mod model;
pub mod web;
pub mod blog;
mod setting;

use std::net::SocketAddr;
use std::str::FromStr;
use axum::Router;
use axum::routing::get;
use handlebars::Handlebars;
use axum_template::engine::Engine;
use model::AppState;
use setting::{Author, Settings};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    //加载配置文件
    //如果配置文件内容错误，或者配置文件不存在，则应用默认配置项
    let settings = Settings::new()
        .unwrap_or(
            Settings {
                title: "Mini Blog".to_string(),
                organization: "版权所属 2023-2024".to_string(),
                record_number: "国家网站备案号".to_string(),
                app_port: 3000,
                log_level: "info".to_string(),
                author: Author { title: "About Me".into(), description: vec![] }
            }
        );

    //启动日志
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::from_str(&settings.log_level).unwrap())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("日志模块启动失败！");
    
    // 模板引擎加载模板文件
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("index", "static/templates/index_template.hbs")
        .unwrap();
    handlebars
        .register_template_file("post", "static/templates/post_template.hbs")
        .unwrap();
    handlebars
        .register_template_file("category", "static/templates/category_template.hbs")
        .unwrap();
    handlebars
        .register_template_file("tag", "static/templates/tag_template.hbs")
        .unwrap();

    handlebars.register_partial("title", settings.title).unwrap();

    let description = settings.author.description
        .iter()
        .map(|s| format!("<br><span>{}</span>", s))
        .collect::<Vec<String>>()
        .join("\n");

    handlebars.register_partial("author", format!(r###"
            <div class="author">
				<p>
                    <img src="../static/avator.png"/>
                    <strong>{}</strong>
                    {}
                </p>
			</div>
        "###, settings.author.title, description)).unwrap();

    handlebars.register_partial("footer", format!(r###"
            <footer>
                <div class="line"></div>
                <br>
                <center class="footer">
                    <span>@{}; Licensed under <a href="https://creativecommons.org/licenses/by-sa/4.0/">CC BY-SA</a>; Powered by <a href="http://app.yeyanbo.cn/mini-blog">mini-blog</a>, rev 0.4.1</span>
                    <span>{}</span>
                </center>
                <br>
            </footer>
    "###, settings.organization, settings.record_number)).unwrap();

    // build our application with a single route
    let state = AppState::new(Engine::from(handlebars));
    let app = Router::new()
        .route("/", get(web::index))
        .route("/post/:pid", get(web::posts))
        .route("/category/:cid", get(web::category))
        .route("/tag/:tid", get(web::tag))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    // run it with hyper on localhost:3000
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.app_port));
    tracing::info!("监听器开始监听端口[{}]...", &addr);

    let listener = TcpListener::bind(addr)
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}



