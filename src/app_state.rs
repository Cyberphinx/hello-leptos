use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
    use axum::extract::FromRef;
    use leptos::LeptosOptions;
    use sqlx::{Pool, Postgres};

    /// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
    /// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
    #[derive(Clone, FromRef)]
    pub struct AppState {
        pub http_client: reqwest::Client,
        pub pool: Pool<Postgres>,
        pub leptos_options: LeptosOptions,
    }
}}
