pub mod model;
pub mod web;
pub mod blog;

use std::net::SocketAddr;

use axum::Router;
use axum::routing::get;
use handlebars::Handlebars;
use axum_template::engine::Engine;
use model::AppState;
use tower_http::services::ServeDir;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    
    // Set up the Handlebars engine with the same route paths as the Axum router
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("index", "static/templates/index_template.hbs")
        .unwrap();
    handlebars
        .register_template_file("post", "static/templates/post_template.hbs")
        .unwrap();
    // handlebars.register_helpxer("format_date", Box::new(format_date));
    // handlebars.register_escape_fn(handlebars::no_escape);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(web::index))
        .route("/post/:pid", get(web::posts))
        .with_state(AppState {
            engine: Engine::from(handlebars),
        })
        .nest_service("/static", ServeDir::new("static"));

    // run it with hyper on localhost:3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}



