//! Module [http] is an inbound/driving adapter that exposes
//! a REST-ful API to interact with the core domain over HTTP.

use crate::domain::booking::ports::BookingService;
use anyhow::Context;
use axum::extract::Request;
use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[derive(Debug, Clone)]
pub struct HttpConfig<'cfg> {
    pub port: &'cfg str,
}

#[derive(Debug, Clone)]
pub struct AppState<BS: BookingService> {
    pub bookings: Arc<BS>,
}

pub struct HttpServer {
    router: Router,
    listener: TcpListener,
}

impl HttpServer {
    pub async fn new<BS: BookingService>(
        config: HttpConfig<'_>,
        bookings: BS,
    ) -> anyhow::Result<Self> {
        let trace_layer = TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            let uri = request.uri().to_string();
            tracing::info_span!("http_request", method = ?request.method(), uri)
        });

        let app_state = AppState {
            bookings: Arc::new(bookings),
        };

        let router = Router::new()
            .nest("/api", Router::new())
            .layer(trace_layer)
            .with_state(app_state);

        let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("failed to bind to {}", config.port))?;

        Ok(Self { router, listener })
    }

    pub async fn serve(self) -> anyhow::Result<()> {
        tracing::debug!("listening on {}", self.listener.local_addr()?);

        axum::serve(self.listener, self.router)
            .await
            .context("Error starting server")?;

        Ok(())
    }
}
