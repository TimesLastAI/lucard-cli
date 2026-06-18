use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitStatus;
use std::sync::Arc;

use bytes::Bytes;
use lucard_app::{
    CommandInfra, DirectoryReaderInfra, EnvironmentInfra, FileDirectoryInfra, FileInfoInfra,
    FileReaderInfra, FileRemoverInfra, FileWriterInfra, HttpInfra, McpServerInfra, StrategyFactory,
    UserInfra, WalkerInfra,
};
use lucard_domain::{
    AuthMethod, CommandOutput, Environment, FileInfo as FileInfoData, McpServerConfig, ProviderId,
    URLParam,
};
use reqwest::header::HeaderMap;
use reqwest::{Response, Url};
use reqwest_eventsource::EventSource;

use crate::auth::{AnyAuthStrategy, LucardAuthStrategyFactory};
use crate::env::LucardEnvironmentInfra;
use crate::executor::LucardCommandExecutorService;
use crate::fs_create_dirs::LucardCreateDirsService;
use crate::fs_meta::LucardFileMetaService;
use crate::fs_read::LucardFileReadService;
use crate::fs_read_dir::LucardDirectoryReaderService;
use crate::fs_remove::LucardFileRemoveService;
use crate::fs_write::LucardFileWriteService;
use crate::http::LucardHttpInfra;
use crate::inquire::LucardInquire;
use crate::mcp_client::LucardMcpClient;
use crate::mcp_server::LucardMcpServer;
use crate::walker::LucardWalkerService;

#[derive(Clone)]
pub struct LucardInfra {
    // TODO: Drop the "Service" suffix. Use names like LucardFileReader, LucardFileWriter,
    // LucardHttpClient etc.
    file_read_service: Arc<LucardFileReadService>,
    file_write_service: Arc<LucardFileWriteService>,
    file_remove_service: Arc<LucardFileRemoveService>,
    environment_service: Arc<LucardEnvironmentInfra>,
    file_meta_service: Arc<LucardFileMetaService>,
    create_dirs_service: Arc<LucardCreateDirsService>,
    directory_reader_service: Arc<LucardDirectoryReaderService>,
    command_executor_service: Arc<LucardCommandExecutorService>,
    inquire_service: Arc<LucardInquire>,
    mcp_server: LucardMcpServer,
    walker_service: Arc<LucardWalkerService>,
    http_service: Arc<LucardHttpInfra<LucardFileWriteService>>,
    strategy_factory: Arc<LucardAuthStrategyFactory>,
}

impl LucardInfra {
    pub fn new(restricted: bool, cwd: PathBuf) -> Self {
        let environment_service = Arc::new(LucardEnvironmentInfra::new(restricted, cwd));
        let env = environment_service.get_environment();

        let file_write_service = Arc::new(LucardFileWriteService::new());
        let http_service = Arc::new(LucardHttpInfra::new(env.clone(), file_write_service.clone()));
        let file_read_service = Arc::new(LucardFileReadService::new());
        let file_meta_service = Arc::new(LucardFileMetaService);
        let directory_reader_service = Arc::new(LucardDirectoryReaderService);

        Self {
            file_read_service,
            file_write_service,
            file_remove_service: Arc::new(LucardFileRemoveService::new()),
            environment_service,
            file_meta_service,
            create_dirs_service: Arc::new(LucardCreateDirsService),
            directory_reader_service,
            command_executor_service: Arc::new(LucardCommandExecutorService::new(
                restricted,
                env.clone(),
            )),
            inquire_service: Arc::new(LucardInquire::new()),
            mcp_server: LucardMcpServer,
            walker_service: Arc::new(LucardWalkerService::new()),
            strategy_factory: Arc::new(LucardAuthStrategyFactory::new()),
            http_service,
        }
    }
}

impl EnvironmentInfra for LucardInfra {
    fn get_environment(&self) -> Environment {
        self.environment_service.get_environment()
    }

    fn get_env_var(&self, key: &str) -> Option<String> {
        self.environment_service.get_env_var(key)
    }

    fn get_env_vars(&self) -> BTreeMap<String, String> {
        self.environment_service.get_env_vars()
    }

    fn is_restricted(&self) -> bool {
        self.environment_service.is_restricted()
    }
}

#[async_trait::async_trait]
impl FileReaderInfra for LucardInfra {
    async fn read_utf8(&self, path: &Path) -> anyhow::Result<String> {
        self.file_read_service.read_utf8(path).await
    }

    async fn read(&self, path: &Path) -> anyhow::Result<Vec<u8>> {
        self.file_read_service.read(path).await
    }

    async fn range_read_utf8(
        &self,
        path: &Path,
        start_line: u64,
        end_line: u64,
    ) -> anyhow::Result<(String, FileInfoData)> {
        self.file_read_service
            .range_read_utf8(path, start_line, end_line)
            .await
    }
}

#[async_trait::async_trait]
impl FileWriterInfra for LucardInfra {
    async fn write(&self, path: &Path, contents: Bytes) -> anyhow::Result<()> {
        self.file_write_service.write(path, contents).await
    }

    async fn write_temp(&self, prefix: &str, ext: &str, content: &str) -> anyhow::Result<PathBuf> {
        self.file_write_service
            .write_temp(prefix, ext, content)
            .await
    }
}

#[async_trait::async_trait]
impl FileInfoInfra for LucardInfra {
    async fn is_binary(&self, path: &Path) -> anyhow::Result<bool> {
        self.file_meta_service.is_binary(path).await
    }

    async fn is_file(&self, path: &Path) -> anyhow::Result<bool> {
        self.file_meta_service.is_file(path).await
    }

    async fn exists(&self, path: &Path) -> anyhow::Result<bool> {
        self.file_meta_service.exists(path).await
    }

    async fn file_size(&self, path: &Path) -> anyhow::Result<u64> {
        self.file_meta_service.file_size(path).await
    }
}
#[async_trait::async_trait]
impl FileRemoverInfra for LucardInfra {
    async fn remove(&self, path: &Path) -> anyhow::Result<()> {
        self.file_remove_service.remove(path).await
    }
}

#[async_trait::async_trait]
impl FileDirectoryInfra for LucardInfra {
    async fn create_dirs(&self, path: &Path) -> anyhow::Result<()> {
        self.create_dirs_service.create_dirs(path).await
    }
}

#[async_trait::async_trait]
impl CommandInfra for LucardInfra {
    async fn execute_command(
        &self,
        command: String,
        working_dir: PathBuf,
        silent: bool,
        env_vars: Option<Vec<String>>,
    ) -> anyhow::Result<CommandOutput> {
        self.command_executor_service
            .execute_command(command, working_dir, silent, env_vars)
            .await
    }

    async fn execute_command_raw(
        &self,
        command: &str,
        working_dir: PathBuf,
        env_vars: Option<Vec<String>>,
    ) -> anyhow::Result<ExitStatus> {
        self.command_executor_service
            .execute_command_raw(command, working_dir, env_vars)
            .await
    }
}

#[async_trait::async_trait]
impl UserInfra for LucardInfra {
    async fn prompt_question(&self, question: &str) -> anyhow::Result<Option<String>> {
        self.inquire_service.prompt_question(question).await
    }

    async fn select_one<T: std::fmt::Display + Send + 'static>(
        &self,
        message: &str,
        options: Vec<T>,
    ) -> anyhow::Result<Option<T>> {
        self.inquire_service.select_one(message, options).await
    }

    async fn select_many<T: std::fmt::Display + Clone + Send + 'static>(
        &self,
        message: &str,
        options: Vec<T>,
    ) -> anyhow::Result<Option<Vec<T>>> {
        self.inquire_service.select_many(message, options).await
    }
}

#[async_trait::async_trait]
impl McpServerInfra for LucardInfra {
    type Client = LucardMcpClient;

    async fn connect(
        &self,
        config: McpServerConfig,
        env_vars: &BTreeMap<String, String>,
    ) -> anyhow::Result<Self::Client> {
        self.mcp_server.connect(config, env_vars).await
    }
}

#[async_trait::async_trait]
impl WalkerInfra for LucardInfra {
    async fn walk(&self, config: lucard_app::Walker) -> anyhow::Result<Vec<lucard_app::WalkedFile>> {
        self.walker_service.walk(config).await
    }
}

#[async_trait::async_trait]
impl HttpInfra for LucardInfra {
    async fn http_get(&self, url: &Url, headers: Option<HeaderMap>) -> anyhow::Result<Response> {
        self.http_service.http_get(url, headers).await
    }

    async fn http_post(&self, url: &Url, body: Bytes) -> anyhow::Result<Response> {
        self.http_service.http_post(url, body).await
    }

    async fn http_delete(&self, url: &Url) -> anyhow::Result<Response> {
        self.http_service.http_delete(url).await
    }
    async fn http_eventsource(
        &self,
        url: &Url,
        headers: Option<HeaderMap>,
        body: Bytes,
    ) -> anyhow::Result<EventSource> {
        self.http_service.http_eventsource(url, headers, body).await
    }
}
#[async_trait::async_trait]
impl DirectoryReaderInfra for LucardInfra {
    async fn list_directory_entries(
        &self,
        directory: &Path,
    ) -> anyhow::Result<Vec<(PathBuf, bool)>> {
        self.directory_reader_service
            .list_directory_entries(directory)
            .await
    }

    async fn read_directory_files(
        &self,
        directory: &Path,
        pattern: Option<&str>,
    ) -> anyhow::Result<Vec<(PathBuf, String)>> {
        self.directory_reader_service
            .read_directory_files(directory, pattern)
            .await
    }
}

impl StrategyFactory for LucardInfra {
    type Strategy = AnyAuthStrategy;
    fn create_auth_strategy(
        &self,
        provider_id: ProviderId,
        method: AuthMethod,
        required_params: Vec<URLParam>,
    ) -> anyhow::Result<Self::Strategy> {
        self.strategy_factory
            .create_auth_strategy(provider_id, method, required_params)
    }
}
