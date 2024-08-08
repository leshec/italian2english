mod words;
use axum::routing::get;
use axum::Router;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use lazy_static::lazy_static;
use tera::Tera;
use tower_http::services::ServeDir;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "templates/**/*";
        let tera = Tera::new(source).unwrap();
        tera
    };
}

async fn index() -> Html<String> {
    let mut context = tera::Context::new();
    context.insert("message_from_rust", "hello from rust");
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    Html(page_content)
}

async fn info() -> Html<String> {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("info.html", &context).unwrap();
    Html(page_content)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

async fn words_endpoint() -> Html<String> {
    let (word1, word2) = words::get_random_word_pair();
    let mut context = tera::Context::new();
    context.insert("word1", &word1);
    context.insert("word2", &word2);
    let page_content = TEMPLATES.render("words.html", &context).unwrap();
    Html(page_content)
}

async fn word_pair_endpoint() -> Html<String> {
    let (word1, word2) = words::get_random_word_pair();
    let mut context = tera::Context::new();
    context.insert("word1", &word1);
    context.insert("word2", &word2);
    let page_content = TEMPLATES.render("word_pair.html", &context).unwrap();
    Html(page_content)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // build our application with a route

    let router = Router::new()
        .route("/", get(index))
        .route("/info", get(info))
        .route("/word-pair", get(word_pair_endpoint))
        .route("/words", get(words_endpoint))
        .nest_service("/favicon.ico", ServeDir::new("image/favicon.ico"));

    Ok(router.into())
}
