use std::{env, net::SocketAddr};

use askama::Template;
use axum::{
    Router,
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

#[derive(Template)]
#[template(path = "not_found.html")]
struct NotFoundTemplate;

#[tokio::main]
async fn main() {
    init_tracing();

    let address: SocketAddr = env::var("MFA_SITE_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3000".into())
        .parse()
        .expect("MFA_SITE_ADDR must be a valid socket address");

    let listener = TcpListener::bind(address)
        .await
        .expect("failed to bind marketing site listener");

    info!(%address, "mailforagents.com listening");
    axum::serve(listener, app())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("marketing site server failed");
}

fn app() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/healthz", get(health))
        .route("/robots.txt", get(robots))
        .route("/sitemap.xml", get(sitemap))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(not_found)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
}

async fn home() -> Response {
    render(StatusCode::OK, &HomeTemplate)
}

async fn not_found() -> Response {
    render(StatusCode::NOT_FOUND, &NotFoundTemplate)
}

async fn health() -> &'static str {
    "ok"
}

async fn robots() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "User-agent: *\nAllow: /\nSitemap: https://mailforagents.com/sitemap.xml\n",
    )
}

async fn sitemap() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\"><url><loc>https://mailforagents.com/</loc></url></urlset>\n",
    )
}

fn render(status: StatusCode, template: &impl Template) -> Response {
    match template.render() {
        Ok(body) => (status, Html(body)).into_response(),
        Err(error) => {
            tracing::error!(%error, "template rendering failed");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("mailforagents_site=info,tower_http=info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl-C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install termination handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn homepage_template_contains_core_promise() {
        let html = HomeTemplate.render().expect("homepage should render");
        assert!(html.contains("Email automation built for agents"));
        assert!(html.contains("AWS SES"));
    }

    #[test]
    fn not_found_template_is_useful() {
        let html = NotFoundTemplate.render().expect("404 should render");
        assert!(html.contains("Return to the signal"));
    }
}
