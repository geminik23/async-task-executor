use async_std::sync::{Arc, Mutex};
use async_trait::async_trait;
use std::collections::HashMap;

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
    async fn handle(
        &self,
        task_message: TaskMessage,
        shared_data: Arc<S>,
    ) -> Result<Response, String>;

    async fn authorize(&self, _shared_data: &Arc<S>) -> bool {
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

    pub async fn register(&self, task_name: String, handler: Arc<dyn TaskHandler<S>>) {
        self.task_handlers.lock().await.insert(task_name, handler);
    }

    pub async fn execute(&self, task_message: TaskMessage) -> Result<Response, String> {
        let task_name = &task_message.task_name;
        let handlers = self.task_handlers.lock().await;

        let handler = match handlers.get(task_name) {
            Some(handler) => {
                if handler.authorize(&self.shared_data).await {
                    handler.clone()
                } else {
                    return Err("Unauthorized".to_string());
                }
            }
            None => return Err("task not found".to_string()),
        };

        // explicitly drop the MutexGuard.
        drop(handlers);

        let shared_data = self.shared_data.clone();
        let handle_futere = async move { handler.handle(task_message, shared_data).await };
        async_std::task::spawn(handle_futere).await
    }
}
