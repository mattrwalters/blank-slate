use app::*;
use axum::{routing::post, Router};
use fileserv::file_and_error_handler;
use leptos::{tracing::instrument, *};
use leptos_axum::{generate_route_list, LeptosRoutes};
use log::info;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter};

pub mod fileserv;

#[tokio::main]
async fn main() {
    init_tracing();
    info!("Tracing INIT!");
    run().await;
}

#[instrument(name = "fn_run")]
async fn run() {
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;
    let middleware = ServiceBuilder::new()
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, |cx| view! { cx, <App/> })
        .fallback(file_and_error_handler)
        .layer(middleware)
        .with_state(leptos_options);

    info!("listening on {}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn init_tracing() {
    LogTracer::init().expect("Failed to set logger");
    let fmt_layer = fmt::layer()
        .with_writer(std::io::stdout.with_max_level(Level::DEBUG))
        .pretty()
        .with_thread_names(true)
        .with_file(false)
        .with_line_number(true);
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        "server=debug,tower_http=debug,axum::rejection=trace,app=debug,leptos=debug".into()
    });
    let reg = registry().with(filter_layer).with(fmt_layer);
    tracing::subscriber::set_global_default(reg).expect("Unable to set a global collector");
}
