use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use leptos_axum::*;
    use leptos::*;
    use axum::{
        Router,
        routing::get,
        extract::{State, Path, RawQuery},
        response::{IntoResponse, Response}, body::Body,
        http::{Request, header::HeaderMap}
    };

    use crate::{app::App, app_state::AppState, fileserv::file_and_error_handler};

    async fn server_fn_handler(
        State(app_state): State<AppState>,
        path: Path<String>,
        headers: HeaderMap,
        raw_query: RawQuery,
        request: Request<Body>
    ) -> impl IntoResponse{
        handle_server_fns_with_context(path, headers, raw_query, move || {
            provide_context(app_state.pool.clone());
            provide_context(app_state.http_client.clone());
        }, request).await
    }

    async fn leptos_routes_handler(State(app_state): State<AppState>, request: Request<Body>) -> Response {
        let handler = render_app_to_stream_with_context(
            app_state.leptos_options.clone(),
            move || {
                provide_context(app_state.pool.clone());
                provide_context(app_state.http_client.clone());
            },
            || view! { <App /> });
        handler(request).await.into_response()
    }

    // build the router
    pub fn create_router(app_state: AppState) -> Router {
        let routes = generate_route_list(|| view! { <App /> });

        // build our application with a route
        Router::new()
            .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
            .leptos_routes_with_handler(routes, leptos_routes_handler)
            .fallback(file_and_error_handler)
            .with_state(app_state)
    }
}}
