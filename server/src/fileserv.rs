use app::error_template::AppError;
use app::error_template::ErrorTemplate;
use axum::response::Response as AxumResponse;
use axum::{
    body::{boxed, Body, BoxBody},
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::IntoResponse,
};
use leptos::{Errors, LeptosOptions, view};
use tower::ServiceExt;
use tower_http::services::ServeDir;

// TODO: Handle Files and Errors Better

#[allow(clippy::missing_panics_doc)]
pub async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    request: Request<Body>,
) -> AxumResponse {
    let root = options.site_root.clone();
    let response = get_static_file(uri.clone(), &root).await.unwrap();

    if response.status() == StatusCode::OK {
        response.into_response()
    } else {
        let mut errors = Errors::default();
        errors.insert_with_default_key(AppError::NotFound);
        let handler = leptos_axum::render_app_to_stream(
            #[allow(clippy::implicit_clone)]
            options.to_owned(),
            move |cx| view! {cx, <ErrorTemplate outside_errors=errors.clone()/>},
        );
        handler(request).await.into_response()
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let request = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(request).await {
        Ok(response) => Ok(response.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}
