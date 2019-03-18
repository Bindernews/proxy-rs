use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Each backend specifies a server that this proxy can proxy for
    pub backend: HashMap<String, Backend>,
    pub general: General,
    pub limits: Limits,
}

#[derive(Serialize, Deserialize)]
pub struct General {
    /// Port for the server to listen on.
    pub port: u32,
    /// The server name sent to the client, default is to not send anything
    pub server_name: Option<String>,
    /// Base path for the JSON API
    #[serde(default = "const_api")]
    pub api_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct Limits {
    /// Maximum number of connections per user at a time, set to 0 for unlimited
    /// connections (default: 0).
    #[serde(default = "const_0usize")]
    pub connections_per_user: usize,
    /// Maximum download speed per connection (KB/s) (default: 1000).
    /// Set to 0 to disable rate limiting.
    #[serde(default = "const_1000")]
    pub download_speed: usize,
    /// Maximum upload speed per connection (KB/s) (default: 1000).
    /// Set to 0 to disable rate limiting.
    #[serde(default = "const_1000")]
    pub upload_speed: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Backend {
    /// The url endpoint this backend will be accessed from
    pub endpoint: String,
    /// The destination server's URL
    pub server: String,
    /// Can the user put in a URL and have it be automatically proxied (false),
    /// or do they NEED to have a randomly generated URL
    #[serde(default = "const_false")]
    pub force_random_urls: bool,
    /// There is also the option to explicitly add mappings. If a mapping overrides
    /// an actual object on the server, the mapping will take priority.
    #[serde(default = "const_empty_HashMap_string_string")]
    pub mappings: HashMap<String, String>,
}

pub enum ConfigError {
    Syntax((usize, usize)),
    DuplicateEndpoints,
    Other,
}

impl Config {
    pub fn from_string(s: &str) -> Result<Box<Config>, ConfigError> {
        let config: Config = toml::from_str(s)
            .map_err(|e| {
                match e.line_col() {
                    Some((line, col)) => {
                        return ConfigError::Syntax((line, col));
                    },
                    None => {
                        return ConfigError::Other;
                    },
                }
            })?;
        
        if config.has_duplicate_endpoints() {
            return Err(ConfigError::DuplicateEndpoints);
        }
        Ok(Box::new(config))
    }

    fn has_duplicate_endpoints(&self) -> bool {
        false
    }
}

/*
 * These functions return constant values and are used as default values
 * for serde.
 */
fn const_false() -> bool { false }
fn const_api() -> String { String::from("api") }
fn const_1000() -> usize { 1000 }
fn const_0usize() -> usize { 0 }
fn const_empty_HashMap_string_string() -> HashMap<String, String> { HashMap::new() }
