use std::sync::Arc;

use async_taskex::{SharedData, TaskHandler};
use async_trait::async_trait;

#[derive(Default)]
struct SimpleData;

impl SharedData for SimpleData {}

struct TestHandler;

#[async_trait]
impl TaskHandler<SimpleData> for TestHandler {
    async fn handle(
        &self,
        task_json: String,
        _shared_data: Arc<SimpleData>,
    ) -> Result<String, String> {
        Ok(task_json)
    }
}

#[async_std::test]
async fn test_handler() {
    let data = Arc::new(SimpleData {});
    let handler = TestHandler;
    let message = "Test message".into();

    let result = handler.handle(message, data.clone()).await.unwrap();
    assert_eq!(result, "Test message");
}
