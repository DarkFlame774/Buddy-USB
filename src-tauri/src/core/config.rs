use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConfig {
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
    #[serde(default = "default_response_reserve")]
    pub response_reserve: usize,
    #[serde(default = "default_identity_budget")]
    pub identity_budget: usize,
    #[serde(default = "default_profile_budget")]
    pub profile_budget: usize,
    #[serde(default = "default_memory_budget")]
    pub memory_budget: usize,
    #[serde(default = "default_system_budget")]
    pub system_budget: usize,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            max_tokens: default_max_tokens(),
            response_reserve: default_response_reserve(),
            identity_budget: default_identity_budget(),
            profile_budget: default_profile_budget(),
            memory_budget: default_memory_budget(),
            system_budget: default_system_budget(),
        }
    }
}

fn default_max_tokens() -> usize { 32768 }
fn default_response_reserve() -> usize { 2048 }
fn default_identity_budget() -> usize { 500 }
fn default_profile_budget() -> usize { 300 }
fn default_memory_budget() -> usize { 4000 }
fn default_system_budget() -> usize { 200 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    #[serde(default = "default_provider")]
    pub provider: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_base_url")]
    pub base_url: String,
    #[serde(default)]
    pub context: ContextConfig,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: default_provider(),
            model: default_model(),
            base_url: default_base_url(),
            context: ContextConfig::default(),
        }
    }
}

fn default_provider() -> String { "ollama".to_string() }
fn default_model() -> String { "qwen2.5:7b".to_string() }
fn default_base_url() -> String { "http://localhost:11434".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    #[serde(default = "default_extraction_enabled")]
    pub extraction_enabled: bool,
    #[serde(default = "default_max_retrieval_results")]
    pub max_retrieval_results: usize,
    #[serde(default = "default_similarity_threshold")]
    pub similarity_threshold: f32,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            extraction_enabled: default_extraction_enabled(),
            max_retrieval_results: default_max_retrieval_results(),
            similarity_threshold: default_similarity_threshold(),
        }
    }
}

fn default_extraction_enabled() -> bool { true }
fn default_max_retrieval_results() -> usize { 10 }
fn default_similarity_threshold() -> f32 { 0.5 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    #[serde(default = "default_embedding_model")]
    pub model: String,
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            model: default_embedding_model(),
        }
    }
}

fn default_embedding_model() -> String { "AllMiniLML6V2".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    #[serde(default = "default_database_path")]
    pub database_path: String,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            database_path: default_database_path(),
        }
    }
}

fn default_database_path() -> String { "buddy-data/buddy.db".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityConfig {
    #[serde(default = "default_buddy_name")]
    pub name: String,
}

impl Default for IdentityConfig {
    fn default() -> Self {
        Self {
            name: default_buddy_name(),
        }
    }
}

fn default_buddy_name() -> String { "Buddy".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BuddyConfig {
    #[serde(default)]
    pub llm: LlmConfig,
    #[serde(default)]
    pub memory: MemoryConfig,
    #[serde(default)]
    pub embedding: EmbeddingConfig,
    #[serde(default)]
    pub storage: StorageConfig,
    #[serde(default)]
    pub identity: IdentityConfig,
}

impl BuddyConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        if !path.exists() {
            println!("Config file not found at {:?}, using defaults.", path);
            let default_config = Self::default();
            // Try to generate default config if directory exists or can be created
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
                let _ = default_config.save(path);
            }
            return default_config;
        }

        match fs::read_to_string(path) {
            Ok(contents) => match toml::from_str(&contents) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Failed to parse config at {:?}: {}", path, e);
                    Self::default()
                }
            },
            Err(e) => {
                eprintln!("Failed to read config at {:?}: {}", path, e);
                Self::default()
            }
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let toml_string = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        fs::write(path, toml_string)
    }
}
