use macro_keeper::config_generator;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Clone)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
}

impl FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Debug" => Ok(LogLevel::Debug),
            "Info" => Ok(LogLevel::Info),
            "Warn" => Ok(LogLevel::Warn),
            "Error" => Ok(LogLevel::Error),
            _ => Err(format!("Invalid log level: {}", s)),
        }
    }
}

config_generator!(
    AppConfig,
    CONFIG,
    [
        (log_level, LogLevel, LogLevel::Info),
        (buffer_capacity, usize, 1024),
        (environment, String, "production".to_string())
    ]
);

#[test]
fn test_config_from_hashmap() {
    config_generator!(
        AppConfigTest1,
        CONFIG_TEST_1,
        [
            (log_level, LogLevel, LogLevel::Info),
            (buffer_capacity, usize, 1024),
            (environment, String, "production".to_string())
        ]
    );

    let mut map = HashMap::new();
    map.insert("log_level".to_string(), "Debug".to_string());
    map.insert("buffer_capacity".to_string(), "2048".to_string());
    map.insert("environment".to_string(), "development".to_string());
    AppConfigTest1::from_hashmap(Some(map));
    assert_eq!(*AppConfigTest1::buffer_capacity(), 2048);
    assert_eq!(AppConfigTest1::environment(), "development");
    assert_eq!(*AppConfigTest1::log_level(), LogLevel::Debug);
}

#[test]
fn test_config_default_values() {
    config_generator!(
        AppConfigTest2,
        CONFIG_TEST_2,
        [
            (log_level, LogLevel, LogLevel::Info),
            (buffer_capacity, usize, 1024),
            (environment, String, "production".to_string())
        ]
    );

    AppConfigTest2::from_hashmap(None);
    assert_eq!(*AppConfigTest2::buffer_capacity(), 1024);
    assert_eq!(AppConfigTest2::environment(), "production");
    assert_eq!(*AppConfigTest2::log_level(), LogLevel::Info);
}

#[test]
fn test_config_default_values_from_empty_hashmap() {
    config_generator!(
        AppConfigTest2,
        CONFIG_TEST_2,
        [
            (log_level, LogLevel, LogLevel::Info),
            (buffer_capacity, usize, 1024),
            (environment, String, "production".to_string())
        ]
    );

    AppConfigTest2::from_hashmap(Some(HashMap::new()));
    assert_eq!(*AppConfigTest2::buffer_capacity(), 1024);
    assert_eq!(AppConfigTest2::environment(), "production");
    assert_eq!(*AppConfigTest2::log_level(), LogLevel::Info);
}

#[test]
fn test_config_with_extra_keys() {
    config_generator!(
        AppConfigTest6,
        CONFIG_TEST_6,
        [
            (log_level, LogLevel, LogLevel::Info),
            (buffer_capacity, usize, 1024),
            (environment, String, "production".to_string())
        ]
    );

    let mut map = HashMap::new();
    map.insert("log_level".to_string(), "Warn".to_string());
    map.insert("buffer_capacity".to_string(), "4096".to_string());
    map.insert("environment".to_string(), "staging".to_string());
    map.insert("extra_key".to_string(), "should_be_ignored".to_string());

    AppConfigTest6::from_hashmap(Some(map));
    assert_eq!(*AppConfigTest6::buffer_capacity(), 4096);
    assert_eq!(AppConfigTest6::environment(), "staging");
    assert_eq!(*AppConfigTest6::log_level(), LogLevel::Warn);
}

#[test]
fn test_config_with_invalid_log_level() {
    config_generator!(
        AppConfigTest3,
        CONFIG_TEST_3,
        [
            (log_level, LogLevel, LogLevel::Info),
            (buffer_capacity, usize, 1024),
            (environment, String, "production".to_string())
        ]
    );

    let mut map = HashMap::new();
    map.insert("log_level".to_string(), "InvalidLevel".to_string());
    AppConfigTest3::from_hashmap(Some(map));
    assert_eq!(*AppConfigTest3::buffer_capacity(), 1024);
    assert_eq!(AppConfigTest3::environment(), "production");
    assert_eq!(*AppConfigTest3::log_level(), LogLevel::Info);
}

#[test]
fn test_panic_on_double_initialization() {
    config_generator!(
        AppConfigTest4,
        CONFIG_TEST_4,
        [
            (log_level, LogLevel, LogLevel::Info),
            (buffer_capacity, usize, 1024),
            (environment, String, "production".to_string())
        ]
    );

    let mut map = HashMap::new();
    map.insert("log_level".to_string(), "Debug".to_string());
    AppConfigTest4::from_hashmap(Some(map.clone()));

    // This should panic because the config is already initialized
    let result = std::panic::catch_unwind(|| {
        AppConfigTest4::from_hashmap(Some(map));
    });

    assert!(result.is_err());
}

#[test]
fn test_panic_on_calling_before_initialization() {
    config_generator!(
        AppConfigTest5,
        CONFIG_TEST_5,
        [
            (log_level, LogLevel, LogLevel::Info),
            (buffer_capacity, usize, 1024),
            (environment, String, "production".to_string())
        ]
    );

    // This should panic because the config is not initialized
    let result = std::panic::catch_unwind(|| {
        AppConfigTest5::log_level();
    });

    assert!(result.is_err());
}
