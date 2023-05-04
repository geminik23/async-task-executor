use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub trait SharedData: Send + Sync + 'static {}

// definition of the interface for handling tasks.
#[async_trait]
pub trait TaskHandler<S: SharedData>: Send + Sync {
    async fn handle(&self, task_json: String, shared_data: Arc<S>) -> Result<String, String>;
    async fn authorize(&self, _shared_data: Arc<S>) -> bool {
        true
    }
}
