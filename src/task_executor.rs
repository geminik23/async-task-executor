use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

pub trait SharedData: Send + Sync + 'static {}

#[derive(Serialize, Deserialize)]
pub struct TaskMessage {
    pub task_name: String,
    pub payload: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub success: bool,
    pub result: Option<String>,
    pub error: Option<String>,
}

// definition of the interface for handling tasks.
#[async_trait]
pub trait TaskHandler<S: SharedData>: Send + Sync {
    async fn handle(&self, task_json: String, shared_data: Arc<S>) -> Result<String, String>;
    async fn authorize(&self, _shared_data: Arc<S>) -> bool {
        true
    }
}

// The TaskExecutor struct manages a collection of TaskHandler implementations.
pub struct TaskExecutor<S: SharedData> {
    task_handlers: Arc<Mutex<HashMap<String, Arc<dyn TaskHandler<S>>>>>,
    shared_data: Arc<S>,
}

impl<S: SharedData> TaskExecutor<S> {
    pub fn new(shared_data: Arc<S>) -> Self {
        TaskExecutor {
            task_handlers: Arc::new(Mutex::new(HashMap::new())),
            shared_data,
        }
    }
    pub async fn register(&self, task_name: String, handler: Arc<dyn TaskHandler<S>>) {}

    pub async fn execute(&self, task_name: &str, task_json: String) -> Result<String, String> {
        unimplemented!("")
    }
}
