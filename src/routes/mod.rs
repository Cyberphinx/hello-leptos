use cfg_if::cfg_if;

pub mod hello_world;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use sqlx::{Pool,Postgres};
    use leptos::*;

    pub fn pool() -> Result<Pool<Postgres>, ServerFnError> {
        use_context::<Pool<Postgres>>().ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()) )
    }

    pub fn http_client() -> Result<reqwest::Client, ServerFnError> {
        use_context::<reqwest::Client>().ok_or_else(|| ServerFnError::ServerError("Reqwest Http Client missing".into()) )
    }
}}
