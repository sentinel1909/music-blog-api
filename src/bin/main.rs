// src/main.rs

// dependencies
use shuttle_runtime::Error;
use music_blog_api_lib::{build, get_subscriber, init_subscriber};
use music_blog_api_lib::service::ShuttleTemplateAxum;

// main function; configures tracing, builds the app router, starts the service
#[shuttle_runtime::main]
async fn main() -> Result<ShuttleTemplateAxum, Error> {
    // initialize tracing
    let subscriber = get_subscriber(
        "music-blog-api".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    // build the router
    let app_router = build();

    // start the service
    Ok(ShuttleTemplateAxum { app_router })
}
