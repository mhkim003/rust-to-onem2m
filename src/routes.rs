use axum::{Router, routing::{post, get}};
use crate::handlers::{
    ae::{discover_ae, register_ae},
    container::{get_containers, register_container},
    content_instance::{register_cin, get_latest_cin},
    subscription::register_subscription,
};
use crate::store::{AeStore, ContainerStore, CinStore, SubStore};

pub fn create_router(
    ae_store: AeStore,
    container_store: ContainerStore,
    cin_store: CinStore,
    sub_store: SubStore,
) -> Router {
    let state = (ae_store.clone(), container_store.clone(), cin_store.clone(), sub_store.clone());

    Router::new()
        .route("/csebase", post(register_ae).get(discover_ae))
        .with_state(state.clone())
        .nest(
            "/csebase",
            Router::new()
                .route("/:ae", post(register_container).get(get_containers))
                .route("/:ae/:cnt", post(register_cin))
                .route("/:ae/:cnt/la", get(get_latest_cin))
                .route("/:ae/:cnt/sub", post(register_subscription))
                .with_state(state),
        )
}
