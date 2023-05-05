# Async Task Executor

This project is currently in **active development** and is not yet ready for production use.

A lightweight Rust library for building extensible asynchronous task executors, designed for handling large-scale multitasking in server environments. 


# TODO

**Base Task Executor**
- [x] Trait SharedData to access the sharable data in different thread.
- [x] TaskHandler : takes a message(String type) and return the result (String) - ?use DeserializeOwned?
- [x] Implement the TaskMessage and Response
- [x] Implement the TaskExecutor with TaskHandler
- [ ] more generic approach using `serde::Serialize` and `serde::de::DeserializeOwned` traits


## Future work

- Add a communication layers to allow clients to send task and receive responses : RESTFul API, a gRPC, a WebSocket server
- Implement a communication layer: Develop a communication layer to allow clients to send task messages to the server and receive responses. 
	- This could include creating a RESTful API, a gRPC service, or a WebSocket server to facilitate communication between clients and the multitask executor.
- Improve the error handling and logging.
- Enhance the 'authorize' method.



# License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
