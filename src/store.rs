use std::{collections::HashMap, sync::{Arc, Mutex}};
use crate::model::{Ae, Container, ContentInstance};

pub type AeStore = Arc<Mutex<Vec<Ae>>>;
pub type ContainerStore = Arc<Mutex<HashMap<String, Vec<Container>>>>;

pub type CinStore = Arc<Mutex<HashMap<(String, String), Vec<ContentInstance>>>>;

pub type AppState = (
    AeStore,
    ContainerStore,
    CinStore,
);

pub fn init_store() -> AppState {
    (
        Arc::new(Mutex::new(vec![])),
        Arc::new(Mutex::new(HashMap::new())),
        Arc::new(Mutex::new(HashMap::new())),
    )
}