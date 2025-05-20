use axum::{extract::{Query, State}, Json, http::StatusCode, response::{IntoResponse, Response}};
use crate::{model::{Ae, M2mAe}, store::{AeStore, ContainerStore}};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DiscoveryFilter {
    pub filter: Option<String>,
}

fn internal_error<E>(msg: &'static str, _err: E) -> Response
where
    E: std::fmt::Debug,
{
    (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
}

pub async fn register_ae(
    State((ae_store, _)): State<(AeStore, ContainerStore)>,
    Json(payload): Json<M2mAe>,
) -> Result<Response, Response> {
    let mut db = ae_store.lock().map_err(|e| internal_error("🔒 Failed to lock AE store", e))?;
    db.push(payload.ae.clone());

    Ok((StatusCode::CREATED, format!("✅ AE registered: {:?}", payload.ae)).into_response())
}

pub async fn discover_ae(
    State((ae_store, _)): State<(AeStore, ContainerStore)>,
    Query(query): Query<DiscoveryFilter>,
) -> Result<Response, Response> {
    let db = ae_store.lock().map_err(|e| internal_error("🔒 Failed to lock AE store", e))?;

    let filtered: Vec<Ae> = if let Some(filter) = query.filter {
        let parts: Vec<&str> = filter.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Ok((StatusCode::BAD_REQUEST, "❌ Invalid filter format. Use key:value").into_response());
        }
        let (key, value) = (parts[0], parts[1]);
        db.iter()
            .cloned()
            .filter(|ae| match key {
                "rn" => ae.rn == value,
                "api" => ae.api == value,
                "rr" => ae.rr.to_string() == value,
                _ => false,
            })
            .collect()
    } else {
        db.clone()
    };

    let json = serde_json::to_string(&filtered).map_err(|e| internal_error("❌ Failed to serialize filtered list", e))?;
    Ok((StatusCode::OK, json).into_response())
}
