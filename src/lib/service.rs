// src/lib/service.rs

// dependencies
use crate::routes::health_check;
use crate::telemetry::MakeRequestUuid;
use axum::{http::HeaderName, routing::get, Router};
use shuttle_runtime::{Error, Service};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::layer::Layer;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    normalize_path::NormalizePathLayer,
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

// struct type to represent the server service
pub struct MusicBlogApiService {
    pub app_router: Router,
}

// methods for the MusicBlogApiService type
impl MusicBlogApiService {
    // function to build and return a configured application that serves out the Zola built public assets
    pub fn build() -> Router {
        // define the tracing layer
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(
                DefaultMakeSpan::new()
                    .include_headers(true)
                    .level(Level::INFO),
            )
            .on_response(DefaultOnResponse::new().include_headers(true));

        // define a layer to handle CORS (Cross Origin Resource Sharing)
        let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

        // build the router and wrap it with CORS and the telemetry layers
        let x_request_id = HeaderName::from_static("x-request-id");
        let api_routes = Router::new()
            .route("/health_check", get(health_check))
            .layer(cors)
            .layer(
                ServiceBuilder::new()
                    .layer(SetRequestIdLayer::new(
                        x_request_id.clone(),
                        MakeRequestUuid,
                    ))
                    .layer(trace_layer)
                    .layer(PropagateRequestIdLayer::new(x_request_id)),
            );

        // wrap the API routes with Normalize Path Layer
        let api_router = NormalizePathLayer::trim_trailing_slash().layer(api_routes);

        // combine the api and asset routes to make the complete router
        Router::new()
            .nest_service("/api", api_router)
    }

    pub async fn run_until_stopped(self, addr: SocketAddr) {
        let listener = TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, self.app_router).await.unwrap();
    }
}

// implement the Shuttle Service trait ont he NasaImageryViewerService type
#[shuttle_runtime::async_trait]
impl Service for MusicBlogApiService {
    async fn bind(self, addr: SocketAddr) -> Result<(), Error> {
        let router = self.app_router;

        axum::serve(TcpListener::bind(addr).await?, router).await?;

        Ok(())
    }
}
