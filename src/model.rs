use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Ae {
    pub rn: String,
    pub api: String,
    pub rr: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct M2mAe {
    #[serde(rename = "m2m:ae")]
    pub ae: Ae,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Container {
    pub rn: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct M2mContainer {
    #[serde(rename = "m2m:cnt")]
    pub cnt: Container,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ContentInstance {
    pub con: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct M2mContentInstance {
    #[serde(rename = "m2m:cin")]
    pub cin: ContentInstance,
}

#[derive(Debug, Clone, Deserialize)]
pub struct M2mSubscription {
    #[serde(rename = "m2m:sub")]
    pub sub: Subscription,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Subscription {
    pub rn: String,
    pub nu: Vec<String>,
}