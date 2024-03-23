use cfg_if::cfg_if;

pub mod app;
pub mod app_state;
pub mod error_template;
pub mod fileserv;
pub mod models;
pub mod router;
pub mod routes;
pub mod utilities;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(App);
    }
}}

cfg_if! {
if #[cfg(feature = "ssr")] {
    pub async fn run(app_state: app_state::AppState) {
        use std::{env, net::SocketAddr};
        use crate::router::create_router;

        let address: SocketAddr = if env::var("ENVIRONMENT").unwrap().eq("production") {
            SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 8080))
        } else {
            app_state.leptos_options.site_addr
        };

        let app = create_router(app_state);

        // run with hyper`axum::Server` is a re-export of `hyper::Server`
        log::info!("listening on http://{}", &address);
        axum::Server::bind(&address)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}}
