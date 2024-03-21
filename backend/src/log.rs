use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};
use tracing::debug;
use uuid::Uuid;

use crate::web::{
    context::Context,
    error::{ClientError, Error, Result},
    mw_req_stamp::ReqStamp,
};

pub async fn log_request(
    http_method: Method,
    uri: Uri,
    req_stamp: ReqStamp,
    ctx: Option<Context>,
    web_error: Option<&Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    // -- Prep error
    let error_type = web_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(web_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    // -- Prep Req Information
    let ReqStamp { uuid, time_in } = req_stamp;
    let now = OffsetDateTime::now_utc();
    let duration: Duration = now - time_in;
    // duration_ms in milliseconds with microseconds precision.
    let duration_ms = (duration.as_seconds_f64() * 1_000_000.).floor() / 1_000.;

    // Create the RequestLogLine
    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: now.format(&Rfc3339).unwrap(), // LogLine timestamp ("time_out")
        time_in: now.format(&Rfc3339).unwrap(),
        duration_ms,

        http_path: uri.to_string(),
        http_method: http_method.to_string(),

        user_id: ctx.map(|c| c.get_session().clone().id),

        client_error_type: client_error.map(|e| e.as_ref().to_string()),

        error_type,
        error_data,
    };

    debug!("REQUEST LOG LINE:\n{}", json!(log_line));

    // TODO - Send to cloud-watch and/or have a `pack_and_send` logic as well (newline json and/or parquet file)

    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,      // uuid string formatted
    timestamp: String, // (Rfc3339)
    time_in: String,   // (Rfc3339)
    duration_ms: f64,

    // -- User and context attributes.
    user_id: Option<Uuid>,

    // -- http request attributes.
    http_path: String,
    http_method: String,

    // -- Errors attributes.
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
