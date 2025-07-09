/// Generates static getter methods for each config field defined in a struct.
///
/// This macro is intended to be used internally by `config_generator!`.
/// Each generated getter method returns a reference to the corresponding field.
///
/// # Example Output
/// For input:
/// ```rust
/// __config_ref_getters! {
///     (log_level, &'static LogLevel),
///     (buffer_capacity, &'static usize),
/// }
/// ```
///
/// The following methods will be generated:
/// ```rust
/// pub fn log_level() -> &'static LogLevel { ... }
/// pub fn buffer_capacity() -> &'static usize { ... }
/// ```
///
/// The following methods will be generated:
/// ```rust
/// pub fn log_level() -> &'static LogLevel {
///     let config = Self::get();
///     &config.log_level
/// }
/// pub fn buffer_capacity() -> &'static usize {
///     let config = Self::get();
///     &config.buffer_capacity
/// }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __config_ref_getters {
    ($(($field:ident, $return_type:ty)),* $(,)?) => {
        $(
            pub fn $field() -> $return_type {
                let config = Self::get();
                &config.$field
            }
        )*
    };
}

/// Extracts a typed field from a `HashMap<String, String>` with a fallback default.
///
/// Used internally by `config_generator!` to parse config values from strings.
/// Supports both generic types and `String` specifically.
///
/// # Match Arms
/// - For `String` types, it clones the value.
/// - For other types, it tries to `parse()` them from the string value.
///
/// # Example
/// ```rust
/// let my_string: String = __config_field!(map, some_key, "default".to_string(), String);
/// let my_number: usize = __config_field!(map, size, 100, usize);
/// let my_enum: SomeEnumType = __config_field!(map, enum_key, SomeEnumType::default(), SomeEnumType);
/// ```
/// The fllowing code will be generated:
/// ```rust
/// let my_string: String = map
///     .get("some_key")
///     .cloned()
///     .unwrap_or("default".to_string());
/// let my_number: usize = map
///     .get("size")
///     .and_then(|s| s.parse::<usize>().ok())
///     .unwrap_or(100);
/// let my_enum: SomeEnumType = map
///     .get("enum_key")
///     .and_then(|s| s.parse::<SomeEnumType>().ok())
///     .unwrap_or(SomeEnumType::default());
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __config_field {
    ($hash:ident, $key:ident, $default:expr, String) => {
        $hash.get(stringify!($key)).cloned().unwrap_or($default)
    };

    ($hash:ident, $key:ident, $default:expr, $type:ty) => {
        $hash
            .get(stringify!($key))
            .and_then(|s| s.parse::<$type>().ok())
            .unwrap_or($default)
    };
}

/// Generates a reusable static configuration struct with default values and typed getters.
///
/// This macro allows you to define a global, lazily-initialized config object
/// that is accessible across your codebase without explicit parameter passing.
///
/// It creates:
/// - A `struct` with public fields
/// - A `static OnceLock` to store the configuration
/// - A `.from_hashmap()` initializer to populate it from environment/config maps
/// - Typed `&'static` getter methods for each field
///
/// # Parameters
/// - `$object`: The name of the struct
/// - `$static_name`: The name of the static `OnceLock`
/// - A list of tuples: `(field_name, field_type, default_value)`
///
/// **Note:** Getter methods return `&'static` references. Use `*value` or `.as_str()`
/// when interacting with them.
///
/// # Example
/// ```rust
/// use std::collections::HashMap;
///
/// config_generator!(
///     AppConfig,
///     CONFIG,
///     [
///         (log_level, LogLevel, LogLevel::Info),
///         (buffer_capacity, usize, 1024),
///         (environment, String, "production".to_string())
///     ]
/// );
///
/// let mut map = HashMap::new();
/// map.insert("environment".to_string(), "development".to_string());
/// AppConfig::from_hashmap(Some(map));
///
/// // Accessing values
/// let level = AppConfig::log_level();           // &'static LogLevel => LogLevel::Info
/// let capacity = AppConfig::buffer_capacity();  // &'static usize => 1024
/// let env = AppConfig::environment();           // &'static String => "development"
///
/// // Interacting with values
/// let double = *capacity * 2;                   // 2048
/// let is_prod = env.as_str() == "production";   // false
/// ```
#[macro_export]
macro_rules! config_generator {
  ($object:ident, $static_name:ident, [$(($field:ident, $type:ty, $default:expr)), * $(,)?]) => {

    static $static_name: std::sync::OnceLock<$object> = std::sync::OnceLock::new();

    #[derive(Debug, Clone)]
    pub struct $object {
        $(pub $field: $type),*
    }

    impl $object {
        fn get() -> &'static $object {
            $static_name.get().expect("Config not initialized. Did you forget to call from_hashmap()?")
        }

        fn new($( $field: $type ),*) -> &'static $object {
            let tmp_config = Self { $( $field ),* };
            $static_name.set(tmp_config).expect("Config already initialized.");
            Self::get()
        }

        pub fn from_hashmap(hash: Option<std::collections::HashMap<String, String>>) -> &'static $object {
            let hash = hash.unwrap_or_default();
            $(
                let $field: $type = $crate::__config_field!(hash, $field, $default, $type);
            )*

            Self::new($( $field ),*)
        }

        $crate::__config_ref_getters! {
            $(($field, &'static $type)),*
        }
    }

    impl Default for &'static $object {
        fn default() -> Self {
            $object::new($( $default ),*)
        }
    }
  };
}
