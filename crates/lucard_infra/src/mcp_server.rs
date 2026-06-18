use std::collections::BTreeMap;

use lucard_app::McpServerInfra;
use lucard_domain::McpServerConfig;

use crate::mcp_client::LucardMcpClient;

#[derive(Clone)]
pub struct LucardMcpServer;

#[async_trait::async_trait]
impl McpServerInfra for LucardMcpServer {
    type Client = LucardMcpClient;

    async fn connect(
        &self,
        config: McpServerConfig,
        env_vars: &BTreeMap<String, String>,
    ) -> anyhow::Result<Self::Client> {
        Ok(LucardMcpClient::new(config, env_vars))
    }
}
