use serde::{Deserialize, Serialize};

use crate::objects::KubeObject;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum WatchEvent {
    Put(PutEvent),
    Delete,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PutEvent {
    pub key: String,
    pub object: KubeObject,
}

impl WatchEvent {
    pub fn new_put(key: String, object: KubeObject) -> Self {
        WatchEvent::Put(PutEvent {
            key,
            object,
        })
    }

    pub fn new_delete(key: String) -> Self {
        WatchEvent::Delete
    }
}