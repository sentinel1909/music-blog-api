// src/main.rs

// dependencies
use libsql::Database;
use music_blog_api_lib::service::MusicBlogApiService;
use music_blog_api_lib::{build, get_subscriber, init_subscriber};
use shuttle_runtime::Error;
use std::sync::Arc;

// main function; configures tracing, builds the app router, starts the service
#[shuttle_runtime::main]
async fn main(
    #[shuttle_turso::Turso(
        addr = "libsql://music-blog-api-sentinel1909.turso.io",
        token = "{secrets.DB_TURSO_TOKEN}"
    )]
    client: Database,
) -> Result<MusicBlogApiService, Error> {
    // initialize tracing
    let subscriber = get_subscriber("music-blog-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // initialize the database client and connection
    let client = Arc::new(client);
    let conn = client.connect().unwrap();

    // run the database migrations
    conn.execute(
        "CREATE TABLE IF NOT EXISTS music_posts ( uid text primary key, post text );", ()
    )
    .await
    .unwrap();

    // build the router
    let app_router = build();

    // start the service
    Ok(MusicBlogApiService { app_router })
}
