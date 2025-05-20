use std::{collections::HashMap, sync::{Arc, Mutex}};
use crate::model::{Ae, Container};

pub type AeStore = Arc<Mutex<Vec<Ae>>>;
pub type ContainerStore = Arc<Mutex<HashMap<String, Vec<Container>>>>;
