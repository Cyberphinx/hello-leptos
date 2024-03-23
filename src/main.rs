#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use conflux::app_state::AppState;
    use conflux::run;
    use leptos::*;
    use sqlx::postgres::PgPoolOptions;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    dotenvy::dotenv().expect("Error loading .env files");

    let database_url = std::env::var("DATABASE_URL").expect("Missing database url");

    // connect to database via sqlx
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error connecting to the database");

    sqlx::migrate!().run(&pool).await.expect("migration failed");

    // instantiate a http client
    let http_client = match reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36")
        .build() {
            Ok(http_client) => http_client,
            Err(error) => {
                eprintln!("Error building reqwest http client: {:?}", error);
                panic!();
        }
    };

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;

    let app_state = AppState {
        http_client,
        pool,
        leptos_options,
    };

    run(app_state).await;
}
