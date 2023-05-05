use std::sync::Arc;

use async_taskex::{Response, SharedData, TaskHandler, TaskMessage};
use async_trait::async_trait;

#[derive(Default)]
struct SimpleData;

impl SharedData for SimpleData {}

struct TestHandler;

#[async_trait]
impl TaskHandler<SimpleData> for TestHandler {
    async fn handle(
        &self,
        task_message: TaskMessage,
        _shared_data: Arc<SimpleData>,
    ) -> Result<Response, String> {
        Ok(Response {
            success: true,
            result: Some(task_message.payload),
            error: None,
        })
    }
}

#[async_std::test]
async fn test_handler() {
    let data = Arc::new(SimpleData {});
    let handler = TestHandler;
    let message = TaskMessage {
        task_name: "test_type".into(),
        payload: "Test message".into(),
    };

    let response = handler.handle(message, data.clone()).await.unwrap();
    assert!(response.success);
    assert_eq!(response.result.unwrap(), "Test message");
}
