# macro_keeper

> A lightweight macro toolkit for generating statically typed configs and utilities.

**Warning**: This crate is still in early development and only includes one macro at the moment. More are coming soon!

## Features

- `config_generator!` – A macro to define static, thread-safe config structs with global access.

## Example

```rust
config_generator!(
    AppConfig,
    CONFIG,
    [
        (log_level, LogLevel, LogLevel::Info),
        (environment, String, "production".to_string())
    ]
);
```
