use std::sync::Arc;
use tokio::time::Instant;

use poem::{Endpoint, IntoResponse, Middleware, Request, Response, Result};
use prometheus::{
    histogram_opts, opts, register_histogram_vec_with_registry,
    register_int_counter_vec_with_registry, HistogramVec, IntCounterVec, Registry,
};
/// A middleware that extracts token from HTTP headers.
pub struct MetricsMiddleware {
    request_counter: Arc<IntCounterVec>,
    request_duration: Arc<HistogramVec>,
}

impl MetricsMiddleware {
    pub fn new(registry: &Registry) -> Self {
        let request_counter = register_int_counter_vec_with_registry!(
            opts!("prb_http_requests_total", "Number of HTTP requests made."),
            &["method", "path", "status"],
            registry,
        )
        .expect("Could not create IntCounterVec for request_counter.");
        let request_duration = register_histogram_vec_with_registry!(
            histogram_opts!(
                "prb_http_request_duration_seconds",
                "HTTP request latencies in seconds."
            ),
            &["method", "path", "status"],
            registry,
        )
        .expect("Could not create HistogramVec for request_duration.");
        Self {
            request_counter: Arc::new(request_counter),
            request_duration: Arc::new(request_duration),
        }
    }
}

impl<E: Endpoint> Middleware<E> for MetricsMiddleware {
    type Output = MetricsMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        MetricsMiddlewareImpl {
            ep,
            request_counter: self.request_counter.clone(),
            request_duration: self.request_duration.clone(),
        }
    }
}

/// The new endpoint type generated by the TokenMiddleware.
pub struct MetricsMiddlewareImpl<E> {
    ep: E,
    request_counter: Arc<IntCounterVec>,
    request_duration: Arc<HistogramVec>,
}

impl<E: Endpoint> Endpoint for MetricsMiddlewareImpl<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        // Record start time with tokio::time::Instant
        let start = Instant::now();
        // Extract method and path labels
        let method_label = req.method().to_string();
        let path_label = req.uri().path().to_string();
        // Forward request to next endpoint
        let result = self.ep.call(req).await;
        let duration_sec = start.elapsed().as_secs_f64();
        // Determine status code
        let (status, resp) = match result {
            Ok(resp) => {
                let resp = resp.into_response();
                (resp.status(), Ok(resp))
            }
            Err(err) => (err.status(), Err(err)),
        };
        let status_label = status.as_str().to_string();
        // Update prometheus metrics (request counter & duration)
        self.request_counter
            .with_label_values(&[&method_label, &path_label, &status_label])
            .inc();

        self.request_duration
            .with_label_values(&[&method_label, &path_label, &status_label])
            .observe(duration_sec);

        // Return the response
        resp
    }
}