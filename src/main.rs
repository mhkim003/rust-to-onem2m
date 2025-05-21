mod routes;
mod store;
mod model;
mod handlers;
use routes::create_router;
use tokio::net::TcpListener;
use crate::store::init_store;

#[tokio::main]
async fn main() {
    let (ae_store, container_store, cin_store) = init_store();
    let app = create_router(ae_store, container_store, cin_store);

    let listener = TcpListener::bind("127.0.0.1:7579").await.unwrap();
    println!("✅ Server listening on http://127.0.0.1:7579");
    axum::serve(listener, app).await.unwrap();
}
