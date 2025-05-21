use axum::{extract::{Path, State}, Json, http::StatusCode, response::{IntoResponse, Response}};
use crate::{model::{M2mContainer}, store::{AeStore, ContainerStore, CinStore}};

fn internal_error<E>(msg: &'static str, _err: E) -> Response
where
    E: std::fmt::Debug,
{
    (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
}

pub async fn register_container(
    Path(ae_id): Path<String>,
    State((ae_store, container_store, _)): State<(AeStore, ContainerStore, CinStore)>,
    Json(payload): Json<M2mContainer>,
) -> Result<Response, Response> {
    let db = ae_store.lock().map_err(|e| internal_error("🔒 Failed to lock AE store", e))?;
    if !db.iter().any(|ae| ae.rn == ae_id) {
        return Ok((StatusCode::NOT_FOUND, format!("❌ AE '{}' not found", ae_id)).into_response());
    }
    drop(db);

    let mut containers = container_store.lock().map_err(|e| internal_error("🔒 Failed to lock Container store", e))?;
    let entry = containers.entry(ae_id.clone()).or_default();

    if entry.iter().any(|c| c.rn == payload.cnt.rn) {
        return Ok((StatusCode::CONFLICT, "❌ Container already exists").into_response());
    }

    entry.push(payload.cnt.clone());
    Ok((StatusCode::CREATED, format!("📦 Container '{}' registered under '{}'", payload.cnt.rn, ae_id)).into_response())
}

pub async fn get_containers(
    Path(ae_id): Path<String>,
    State((ae_store, container_store, _)): State<(AeStore, ContainerStore, CinStore)>,
) -> Result<Response, Response> {
    let db = ae_store.lock().map_err(|e| internal_error("🔒 Failed to lock AE store", e))?;
    if !db.iter().any(|ae| ae.rn == ae_id) {
        return Ok((StatusCode::NOT_FOUND, format!("❌ AE '{}' not found", ae_id)).into_response());
    }
    drop(db);

    let containers = container_store.lock().map_err(|e| internal_error("🔒 Failed to lock Container store", e))?;
    let result = containers.get(&ae_id).cloned().unwrap_or_default();

    let json = serde_json::to_string(&result).map_err(|e| internal_error("❌ Failed to serialize container list", e))?;
    Ok((StatusCode::OK, json).into_response())
}
