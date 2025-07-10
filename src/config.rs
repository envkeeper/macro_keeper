/// Generates static getter methods for each config field defined in a struct.
///
/// This macro is used internally by [`config_generator!`] to expose typed access
/// to configuration fields.
///
/// Each generated method returns a `&'static` reference to its corresponding field,
/// so you'll typically want to use `.clone()`, `.as_str()`, or `*value` when needed.
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
/// This macro is used internally by [`config_generator!`] during the `.from_hashmap()`
/// process to parse and assign values to each field based on its type.
///
/// # Behavior
/// - For `String` types, it clones the value from the map.
/// - For other types, it attempts to parse the value using `.parse()`.
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
/// This macro creates a global, lazily-initialized config object stored inside a `OnceLock`,
/// backed by a struct with typed fields and convenient static accessors.
///
/// # Parameters
/// - `$object`: Name of the struct (e.g., `AppConfig`)
/// - `$static_name`: Name of the `OnceLock` holding the config (e.g., `CONFIG`)
/// - Field list: A list of `(field_name, field_type, default_value)` tuples
///
/// # Features
/// - Strong typing for each config key
/// - Global access via static getters
/// - One-time initialization from `HashMap<String, String>`
/// - Safe fallback to defaults if a key is missing
///
/// # Notes
/// - Only the **first** call to `.from_hashmap()` initializes the config.
/// - Getter methods return `&'static` references. Use `.clone()`, `.as_str()`, or deref `*value` when needed.
///
/// # Example
/// ```rust
/// use std::collections::HashMap;
/// use macro_keeper::config_generator;
///
/// #[derive(PartialEq, Clone, Debug)]
/// enum LogLevel {
///     Debug,
///     Info,
/// }
///
/// impl std::str::FromStr for LogLevel {
///     type Err = String;
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///         match s {
///             "Debug" => Ok(LogLevel::Debug),
///             "Info" => Ok(LogLevel::Info),
///             _ => Err("Invalid log level".to_string()),
///         }
///     }
/// }
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
/// let level = AppConfig::log_level();           // &'static LogLevel
/// let capacity = AppConfig::buffer_capacity();  // &'static usize
/// let env = AppConfig::environment();           // &'static String
///
/// // Interacting with values
/// let double = *capacity * 2;                   // 2048
/// let is_prod = env.as_str() == "production";   // false
/// ```
#[macro_export]
macro_rules! config_generator {
    ($object:ident, $static_name:ident, [$(($field:ident, $type:ty, $default:expr)),* $(,)?]) => {
        static $static_name: std::sync::OnceLock<$object> = std::sync::OnceLock::new();

        #[derive(Debug, Clone, PartialEq)]
        pub struct $object {
            $(pub $field: $type),*
        }

        impl $object {
            fn get() -> &'static $object {
                $static_name.get().expect("Config not initialized. Did you forget to call from_hashmap()?")
            }

            #[allow(clippy::too_many_arguments)]
            fn new($( $field: $type ),*) -> &'static $object {
                let tmp_config = Self { $( $field ),* };
                $static_name.set(tmp_config).expect("Config already initialized.");
                Self::get()
            }

            /// Initializes the global config with optional overrides.
            ///
            /// This method can only be called once. Any further calls will return
            /// the already-initialized config.
            ///
            /// # Arguments
            /// - `hash`: An optional `HashMap<String, String>` with override values.
            ///
            /// # Returns
            /// - A reference to the global config
            pub fn from_hashmap(hash: Option<std::collections::HashMap<String, String>>) -> &'static $object {
                let hash = hash.unwrap_or_default();
                $(
                    let $field: $type = $crate::__config_field!(hash, $field, $default, $type);
                )*

                if $static_name.get().is_some() {
                    return Self::get()
                }

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
