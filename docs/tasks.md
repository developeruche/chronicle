# Chronicle-Task
-------------------------

This implements a rules on how concurrent tasks should be executed in the Chronicle.

Once the Trait `Task` is implemented, the task executor logic in the main binary will be able to execute the tasks concurrently. The `Task` trait posses a `run` function would does not have a default implementation, meaning it must be implemented by all tasks. This run function is then used to create and async handle.

```rust
pub async fn spawn_tasks<T, R, E>(tasks: impl IntoIterator<Item = Box<dyn Task>>, signal: T)
where
    T: Future<Output = Result<R, E>> + Send + 'static,
    E: std::fmt::Debug;

```


The `Task` trait is implemented by the following tasks:
1. `IndexerTask`
```rust
#[derive(Debug)]
pub struct IndexerTask {
    pub config: IndexerConfig,
}
```
2. `ServerTask`
```rust
#[derive(Debug)]
pub struct ServerTask {
    pub config: ServerConfig,
}
```
