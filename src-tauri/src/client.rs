use floresta_rpc::jsonrpc_client::{Client, JsonRPCConfig};
use floresta_rpc::rpc::FlorestaRPC;
use floresta_rpc::rpc_types::GetBlockchainInfoRes;
use serde::Serialize;
use std::sync::Mutex;

/// Configuration for connecting to a running florestad instance
#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct FlorestadConfig {
    pub network: String,
    pub rpc_host: String,
    pub rpc_port: u16,
    pub rpc_user: Option<String>,
    pub rpc_password: Option<String>,
}

impl Default for FlorestadConfig {
    fn default() -> Self {
        Self {
            network: "signet".to_string(),
            rpc_host: "127.0.0.1".to_string(),
            rpc_port: 38332,
            rpc_user: None,
            rpc_password: None,
        }
    }
}

impl FlorestadConfig {
    pub fn rpc_url(&self) -> String {
        format!("http://{}:{}", self.rpc_host, self.rpc_port)
    }
}

// Wraps the floresta-rpc client and manages its lifecyclee
pub struct FlorestaClient {
    inner: Mutex<Option<Client>>,
    config: Mutex<Option<FlorestadConfig>>,
}

impl FlorestaClient {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(None),
            config: Mutex::new(None),
        }
    }

    pub fn connect(&self, config: FlorestadConfig) -> Result<(), String> {
        let client = Client::new_with_config(JsonRPCConfig {
            url: config.rpc_url(),
            user: config.rpc_user.clone(),
            pass: config.rpc_password.clone(),
        });

        *self.inner.lock().map_err(|e| e.to_string())? = Some(client);
        *self.config.lock().map_err(|e| e.to_string())? = Some(config);

        Ok(())
    }

    pub fn disconnect(&self) -> Result<(), String> {
        *self.inner.lock().map_err(|e| e.to_string())? = None;
        *self.config.lock().map_err(|e| e.to_string())? = None;
        Ok(())
    }

    pub fn get_config(&self) -> Result<Option<FlorestadConfig>, String> {
        let config = self.config.lock().map_err(|e| e.to_string())?;
        Ok(config.clone())
    }

    fn with_client<F, T>(&self, f: F) -> Result<T, String>
    where
        F: FnOnce(&Client) -> Result<T, floresta_rpc::rpc_types::Error>,
    {
        let guard = self.inner.lock().map_err(|e| e.to_string())?;
        let client = guard
            .as_ref()
            .ok_or_else(|| "Not connected to florestad".to_string())?;

        f(client).map_err(|e| format!("RPC error: {e}"))
    }

    pub fn get_blockchain_info(&self) -> Result<GetBlockchainInfoRes, String> {
        self.with_client(|c| c.get_blockchain_info())
    }

    pub fn get_block_count(&self) -> Result<u32, String> {
        self.with_client(|c| c.get_block_count())
    }

    pub fn get_best_block_hash(&self) -> Result<String, String> {
        self.with_client(|c| c.get_best_block_hash().map(|h| h.to_string()))
    }

    pub fn get_block_hash(&self, height: u32) -> Result<String, String> {
        self.with_client(|c| c.get_block_hash(height).map(|h| h.to_string()))
    }

    pub fn get_block_header(&self, hash: String) -> Result<serde_json::Value, String> {
        self.with_client(|c| {
            let hash = hash
                .parse()
                .map_err(|_| floresta_rpc::rpc_types::Error::EmptyResponse)?;
            c.get_block_header(hash)
        })
        .map(|header| serde_json::to_value(header).unwrap_or_default())
    }
}
