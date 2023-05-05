use std::error::Error;

use async_std::sync::Arc;
use async_taskex::{Response, SharedData, TaskExecutor, TaskHandler, TaskMessage};
use async_trait::async_trait;

pub struct ExampleData {}

impl SharedData for ExampleData {}

struct ExampleTaskHandler;

#[async_trait]
impl TaskHandler<ExampleData> for ExampleTaskHandler {
    async fn handle(
        &self,
        task_message: TaskMessage,
        _shared_data: Arc<ExampleData>,
    ) -> Result<Response, String> {
        println!(
            "Received - task name({}), message({})",
            task_message.task_name, task_message.payload
        );

        let mut result = task_message.payload.clone();
        result.push_str(" World!");

        Ok(Response {
            success: true,
            result: Some(result),
            error: None,
        })
    }
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let shared_data = Arc::new(ExampleData {});
    let task_executor: TaskExecutor<ExampleData> = TaskExecutor::new(shared_data);

    let task_handler = ExampleTaskHandler {};

    task_executor
        .register("task name".to_string(), Arc::new(task_handler))
        .await;

    let response = task_executor
        .execute(TaskMessage {
            task_name: "task name".into(),
            payload: "Hello".into(),
        })
        .await?;

    if response.success {
        println!("Response - {}", response.result.unwrap());
    }

    Ok(())
}
