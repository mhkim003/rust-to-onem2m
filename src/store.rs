use std::{collections::HashMap, sync::{Arc, Mutex}};
use crate::model::{Ae, Container, ContentInstance, Subscription};

pub type AeStore = Arc<Mutex<Vec<Ae>>>;
pub type ContainerStore = Arc<Mutex<HashMap<String, Vec<Container>>>>;
pub type CinStore = Arc<Mutex<HashMap<(String, String), Vec<ContentInstance>>>>;
pub type SubStore = Arc<Mutex<HashMap<(String, String), Vec<Subscription>>>>;

pub type AppState = (
    AeStore,
    ContainerStore,
    CinStore,
    SubStore,
);

pub fn init_store() -> AppState {
    (
        Arc::new(Mutex::new(vec![])),
        Arc::new(Mutex::new(HashMap::new())),
        Arc::new(Mutex::new(HashMap::new())),
        Arc::new(Mutex::new(HashMap::new())),
    )
}