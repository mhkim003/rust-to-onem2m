use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use crate::{
    model::{M2mContentInstance},
    store::{AeStore, ContainerStore, CinStore, SubStore},
};
use crate::handlers::notify::notify_subscribers;

fn internal_error<E>(msg: &'static str, _err: E) -> Response
where
    E: std::fmt::Debug,
{
    (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()    
}

pub async fn register_cin(
    Path((ae_id, cnt_id)): Path<(String, String)>,
    State((ae_store, _container_store, cin_store, sub_store)): State<(AeStore, ContainerStore, CinStore, SubStore)>,
    Json(payload): Json<M2mContentInstance>,
) -> Result<Response, Response> {
    let db = ae_store.lock().map_err(|e| internal_error("AE store lock failed", e))?;
    if !db.iter().any(|ae| ae.rn == ae_id) {
        return Ok((StatusCode::NOT_FOUND, format!("AE '{}' not found", ae_id)).into_response());
    }
    drop(db);

    let mut cin_map = cin_store.lock().map_err(|e| internal_error("CIN store lock failed", e))?;
    let entry = cin_map.entry((ae_id.clone(), cnt_id.clone())).or_default();
    entry.push(payload.cin.clone());

    let subs = sub_store.lock().map_err(|e| internal_error("🔒 Sub store lock failed", e))?;
    if let Some(sub_list) = subs.get(&(ae_id.clone(), cnt_id.clone())) {
        let endpoints: Vec<String> = sub_list.iter().flat_map(|s| s.nu.clone()).collect();
        let ae_id_cloned = ae_id.clone();
        let cnt_id_cloned = cnt_id.clone();
        let cin_cloned = payload.cin.clone();
        let endpoints_cloned = endpoints.clone();

        tokio::spawn(async move {
            notify_subscribers(
                &ae_id_cloned,
                &cnt_id_cloned,
                &cin_cloned,
                &endpoints_cloned,
            ).await;
        });
    }

    Ok((StatusCode::CREATED, format!("✅ ContentInstance registered under /{}/{}", ae_id, cnt_id)).into_response())
}

pub async fn get_latest_cin(
    Path((ae_id, cnt_id)): Path<(String, String)>,
    State((_ae_store, _container_store, cin_store, _sub_store)): State<(AeStore, ContainerStore, CinStore, SubStore)>,
) -> Result<Response, Response> {
    let cin_map = cin_store.lock().map_err(|e| internal_error("CIN store lock failed", e))?;
    if let Some(list) = cin_map.get(&(ae_id.clone(), cnt_id.clone())) {
        if let Some(latest) = list.last() {
            let json = serde_json::to_string(latest)
                .map_err(|e| internal_error("JSON serialization error", e))?;
            return Ok((StatusCode::OK, json).into_response());
        }
    }

    Ok((StatusCode::NOT_FOUND, "❌ No ContentInstance found").into_response())
}