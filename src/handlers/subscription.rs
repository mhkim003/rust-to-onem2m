use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use crate::{
    model::{M2mSubscription},
    store::{SubStore, AeStore, ContainerStore, CinStore},
};

pub async fn register_subscription(
    Path((ae_id, cnt_id)): Path<(String, String)>,
    State((_ae_store, _container_store, _cin_store, sub_store)): State<(AeStore, ContainerStore, CinStore, SubStore)>,
    Json(payload): Json<M2mSubscription>,
) -> Result<Response, Response> {
    let sub = payload.sub.clone();
    let mut store = sub_store.lock().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("❌ Subscription store lock failed: {e:?}")).into_response())?;
    let entry = store.entry((ae_id.clone(), cnt_id.clone())).or_default();

    if entry.iter().any(|s| s.rn == sub.rn) {
        return Ok((StatusCode::CONFLICT, "❌ Subscription already exists").into_response());
    }
    println!("📩 Subscription 요청 수신: ae_id = {}, cnt_id = {}", ae_id, cnt_id);
    println!("📦 요청 payload: {:?}", payload);
    entry.push(sub.clone());
    Ok((StatusCode::CREATED, format!("🔔 Subscription '{}' registered under {ae_id}/{cnt_id}", sub.rn)).into_response())
}