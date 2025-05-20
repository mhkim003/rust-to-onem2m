mod routes;
mod store;
mod model;
mod handlers;
use routes::create_router;
use tokio::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use store::{AeStore, ContainerStore};

#[tokio::main]
async fn main() {
    let ae_store: AeStore  = Arc::new(Mutex::new(vec![]));
    let container_store: ContainerStore = Arc::new(Mutex::new(HashMap::new()));
    let app = create_router(ae_store, container_store);

    let listener = TcpListener::bind("127.0.0.1:7579").await.unwrap();
    println!("✅ Server listening on http://127.0.0.1:7579");
    axum::serve(listener, app).await.unwrap();
}
