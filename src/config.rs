use anyhow::Result;

/// Configuration for optional filesystem tools
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub enable_filesystem_tools: bool,
    pub enable_list_directory: bool,
    pub enable_directory_tree: bool,
    pub enable_create_directory: bool,
    pub enable_move_file: bool,
    pub enable_write_file: bool,
    pub enable_read_multiple_files: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            enable_filesystem_tools: false,
            enable_list_directory: false,
            enable_directory_tree: false,
            enable_create_directory: false,
            enable_move_file: false,
            enable_write_file: false,
            enable_read_multiple_files: false,
        }
    }
}

impl ServerConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let enable_all = env_bool("ENABLE_FILESYSTEM_TOOLS");
        
        Self {
            enable_filesystem_tools: enable_all,
            enable_list_directory: enable_all || env_bool("ENABLE_LIST_DIRECTORY"),
            enable_directory_tree: enable_all || env_bool("ENABLE_DIRECTORY_TREE"),
            enable_create_directory: enable_all || env_bool("ENABLE_CREATE_DIRECTORY"),
            enable_move_file: enable_all || env_bool("ENABLE_MOVE_FILE"),
            enable_write_file: enable_all || env_bool("ENABLE_WRITE_FILE"),
            enable_read_multiple_files: enable_all || env_bool("ENABLE_READ_MULTIPLE_FILES"),
        }
    }
}

/// Helper to parse boolean from environment variable
fn env_bool(key: &str) -> bool {
    std::env::var(key)
        .unwrap_or_default()
        .to_lowercase()
        .parse()
        .unwrap_or(false)
}
