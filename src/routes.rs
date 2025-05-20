use axum::{Router, routing::{post}};
use crate::handlers::{
    ae::{register_ae, discover_ae},
    container::{register_container, get_containers},
};
use crate::store::{AeStore, ContainerStore};

pub fn create_router(
    ae_store: AeStore,
    container_store: ContainerStore,
) -> Router {
    let state = (ae_store.clone(), container_store.clone());

    Router::new()
        .route("/csebase", post(register_ae).get(discover_ae))
        .with_state(state.clone())
        .nest(
            "/csebase",
            Router::new()
                .route("/:ae", post(register_container).get(get_containers))
                .with_state(state),
        )
}
