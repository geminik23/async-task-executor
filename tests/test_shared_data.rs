use async_taskex::SharedData;

#[derive(Default)]
struct SimpleSharedData {
    counter: i32,
}

impl SharedData for SimpleSharedData {}

#[async_std::test]
async fn test_shared_data() {
    let shared_data = SimpleSharedData::default();
    assert_eq!(shared_data.counter, 0);
}
