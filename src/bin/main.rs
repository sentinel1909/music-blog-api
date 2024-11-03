// src/main.rs

// dependencies
use libsql::Database;
use music_blog_api_lib::service::MusicBlogApiService;
use music_blog_api_lib::{get_subscriber, init_subscriber};
use shuttle_runtime::{CustomError, Error, Secrets, SecretStore};
use shuttle_turso::Turso;
use std::sync::Arc;

// main function; configures tracing, builds the app router, starts the service
#[shuttle_runtime::main]
async fn main(
    #[Turso(
        addr = "libsql://music-blog-db-sentinel1909.turso.io",
        token = "{secrets.DB_TURSO_TOKEN}"
    )]
    client: Database,
    #[Secrets] secrets: SecretStore,
) -> Result<MusicBlogApiService, Error> {
    // initialize tracing
    let subscriber = get_subscriber("music-blog-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // initialize the database client and connection
    let client = Arc::new(client);
    let conn = client.connect().map_err(|err| {
        let error_msg = format!("Could not create the database connection: {}", err);
        CustomError::new(err).context(error_msg)
    })?;

    // run the database migrations
    conn.execute(
        "CREATE TABLE IF NOT EXISTS music_posts ( uid text primary key, band text, album text, thoughts text );",
        (),
    )
    .await
    .map_err(|err| {
        let error_msg = format!("Could not create the database table: {}", err);
        CustomError::new(err).context(error_msg)
    })?;

    // build the router
    let app_router = MusicBlogApiService::build();

    // start the service
    Ok(MusicBlogApiService { app_router })
}

