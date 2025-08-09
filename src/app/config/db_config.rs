use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DatabaseConfig {
    /// Database connection URL
    #[serde(alias = "database_url")]
    pub url: String,
    
    /// Maximum number of connections in the pool
    #[serde(default = "default_pool_size")]
    pub pool_size: u32,
    
    /// Minimum number of idle connections to maintain
    pub min_idle: Option<u32>,
    
    /// Number of seconds to wait for unacknowledged TCP packets before treating the connection as
    /// broken. This value will determine how long the service stays unavailable in case of full
    /// packet loss between the application and the database.
    #[serde(default = "default_tcp_timeout")]
    pub tcp_timeout: u64,
    
    /// Time to wait for a connection to become available from the connection
    /// pool before returning an error (in milliseconds).
    #[serde(default = "default_connection_timeout")]
    pub connection_timeout: u64,
    
    /// Time to wait for a query response before canceling the query and
    /// returning an error (in milliseconds).
    #[serde(default = "default_statement_timeout")]
    pub statement_timeout: u64,
    
    /// Number of threads to use for asynchronous operations such as connection
    /// creation.
    #[serde(default = "default_helper_threads")]
    pub helper_threads: usize,
    
    /// Whether to enforce that all the database connections are encrypted with TLS.
    #[serde(default = "default_false")]
    pub enforce_tls: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RedisConfig {
    /// Redis connection URL
    pub url: String,
    
    /// Maximum number of connections in the pool
    #[serde(default = "default_redis_pool_size")]
    pub pool_size: u32,
    
    /// Connection timeout in milliseconds
    #[serde(default = "default_redis_timeout")]
    pub timeout: u64,
}

// Default functions
fn default_false() -> bool {
    false
}

fn default_pool_size() -> u32 {
    10
}

fn default_tcp_timeout() -> u64 {
    10000
}

fn default_connection_timeout() -> u64 {
    30000
}

fn default_statement_timeout() -> u64 {
    30000
}

fn default_helper_threads() -> usize {
    10
}

fn default_redis_pool_size() -> u32 {
    10
}

fn default_redis_timeout() -> u64 {
    5000
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "mysql://root:password@localhost:3306/webserver_template".to_string(),
            pool_size: default_pool_size(),
            min_idle: None,
            tcp_timeout: default_tcp_timeout(),
            connection_timeout: default_connection_timeout(),
            statement_timeout: default_statement_timeout(),
            helper_threads: default_helper_threads(),
            enforce_tls: default_false(),
        }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379/".to_string(),
            pool_size: default_redis_pool_size(),
            timeout: default_redis_timeout(),
        }
    }
}
