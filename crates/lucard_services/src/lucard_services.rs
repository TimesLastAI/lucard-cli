use std::sync::Arc;

use lucard_app::{
    AgentRepository, CommandInfra, DirectoryReaderInfra, EnvironmentInfra, FileDirectoryInfra,
    FileInfoInfra, FileReaderInfra, FileRemoverInfra, FileWriterInfra, HttpInfra, KVStore,
    McpServerInfra, Services, StrategyFactory, UserInfra, WalkerInfra,
};
use lucard_domain::{
    AppConfigRepository, ConversationRepository, ProviderRepository, SkillRepository,
    SnapshotRepository,
};

use crate::LucardProviderAuthService;
use crate::agent_registry::LucardAgentRegistryService;
use crate::app_config::LucardAppConfigService;
use crate::attachment::LucardChatRequest;
use crate::auth::LucardAuthService;
use crate::command::CommandLoaderService as LucardCommandLoaderService;
use crate::conversation::LucardConversationService;
use crate::discovery::LucardDiscoveryService;
use crate::env::LucardEnvironmentService;
use crate::instructions::LucardCustomInstructionsService;
use crate::mcp::{LucardMcpManager, LucardMcpService};
use crate::policy::LucardPolicyService;
use crate::provider::LucardProviderService;
use crate::template::LucardTemplateService;
use crate::tool_services::{
    LucardFetch, LucardFollowup, LucardFsCreate, LucardFsPatch, LucardFsRead, LucardFsRemove, LucardFsSearch,
    LucardFsUndo, LucardImageRead, LucardPlanCreate, LucardShell, LucardSkillFetch,
};
use crate::workflow::LucardWorkflowService;

type McpService<F> = LucardMcpService<LucardMcpManager<F>, F, <F as McpServerInfra>::Client>;
type AuthService<F> = LucardAuthService<F>;

/// LucardApp is the main application container that implements the App trait.
/// It provides access to all core services required by the application.
///
/// Type Parameters:
/// - F: The infrastructure implementation that provides core services like
///   environment, file reading, vector indexing, and embedding.
/// - R: The repository implementation that provides data persistence
#[derive(Clone)]
pub struct LucardServices<
    F: HttpInfra
        + EnvironmentInfra
        + McpServerInfra
        + WalkerInfra
        + SnapshotRepository
        + ConversationRepository
        + AppConfigRepository
        + KVStore
        + ProviderRepository
        + AgentRepository
        + SkillRepository,
> {
    chat_service: Arc<LucardProviderService<F>>,
    config_service: Arc<LucardAppConfigService<F>>,
    conversation_service: Arc<LucardConversationService<F>>,
    template_service: Arc<LucardTemplateService<F>>,
    attachment_service: Arc<LucardChatRequest<F>>,
    workflow_service: Arc<LucardWorkflowService<F>>,
    discovery_service: Arc<LucardDiscoveryService<F>>,
    mcp_manager: Arc<LucardMcpManager<F>>,
    file_create_service: Arc<LucardFsCreate<F>>,
    plan_create_service: Arc<LucardPlanCreate<F>>,
    file_read_service: Arc<LucardFsRead<F>>,
    image_read_service: Arc<LucardImageRead<F>>,
    file_search_service: Arc<LucardFsSearch<F>>,
    file_remove_service: Arc<LucardFsRemove<F>>,
    file_patch_service: Arc<LucardFsPatch<F>>,
    file_undo_service: Arc<LucardFsUndo<F>>,
    shell_service: Arc<LucardShell<F>>,
    fetch_service: Arc<LucardFetch>,
    followup_service: Arc<LucardFollowup<F>>,
    mcp_service: Arc<McpService<F>>,
    env_service: Arc<LucardEnvironmentService<F>>,
    custom_instructions_service: Arc<LucardCustomInstructionsService<F>>,
    auth_service: Arc<AuthService<F>>,
    agent_registry_service: Arc<LucardAgentRegistryService<F>>,
    command_loader_service: Arc<LucardCommandLoaderService<F>>,
    policy_service: LucardPolicyService<F>,
    provider_auth_service: LucardProviderAuthService<F>,
    skill_service: Arc<LucardSkillFetch<F>>,
}

impl<
    F: McpServerInfra
        + EnvironmentInfra
        + FileWriterInfra
        + FileInfoInfra
        + FileReaderInfra
        + HttpInfra
        + WalkerInfra
        + DirectoryReaderInfra
        + CommandInfra
        + UserInfra
        + SnapshotRepository
        + ConversationRepository
        + AppConfigRepository
        + ProviderRepository
        + KVStore
        + AgentRepository
        + SkillRepository,
> LucardServices<F>
{
    pub fn new(infra: Arc<F>) -> Self {
        let mcp_manager = Arc::new(LucardMcpManager::new(infra.clone()));
        let mcp_service = Arc::new(LucardMcpService::new(mcp_manager.clone(), infra.clone()));
        let template_service = Arc::new(LucardTemplateService::new(infra.clone()));
        let attachment_service = Arc::new(LucardChatRequest::new(infra.clone()));
        let workflow_service = Arc::new(LucardWorkflowService::new(infra.clone()));
        let suggestion_service = Arc::new(LucardDiscoveryService::new(infra.clone()));
        let conversation_service = Arc::new(LucardConversationService::new(infra.clone()));
        let auth_service = Arc::new(LucardAuthService::new(infra.clone()));
        let chat_service = Arc::new(LucardProviderService::new(infra.clone()));
        let config_service = Arc::new(LucardAppConfigService::new(infra.clone()));
        let file_create_service = Arc::new(LucardFsCreate::new(infra.clone()));
        let plan_create_service = Arc::new(LucardPlanCreate::new(infra.clone()));
        let file_read_service = Arc::new(LucardFsRead::new(infra.clone()));
        let image_read_service = Arc::new(LucardImageRead::new(infra.clone()));
        let file_search_service = Arc::new(LucardFsSearch::new(infra.clone()));
        let file_remove_service = Arc::new(LucardFsRemove::new(infra.clone()));
        let file_patch_service = Arc::new(LucardFsPatch::new(infra.clone()));
        let file_undo_service = Arc::new(LucardFsUndo::new(infra.clone()));
        let shell_service = Arc::new(LucardShell::new(infra.clone()));
        let fetch_service = Arc::new(LucardFetch::new());
        let followup_service = Arc::new(LucardFollowup::new(infra.clone()));
        let env_service = Arc::new(LucardEnvironmentService::new(infra.clone()));
        let custom_instructions_service =
            Arc::new(LucardCustomInstructionsService::new(infra.clone()));
        let agent_registry_service = Arc::new(LucardAgentRegistryService::new(infra.clone()));
        let command_loader_service = Arc::new(LucardCommandLoaderService::new(infra.clone()));
        let policy_service = LucardPolicyService::new(infra.clone());
        let provider_auth_service = LucardProviderAuthService::new(infra.clone());
        let skill_service = Arc::new(LucardSkillFetch::new(infra.clone()));

        Self {
            conversation_service,
            attachment_service,
            template_service,
            workflow_service,
            discovery_service: suggestion_service,
            mcp_manager,
            file_create_service,
            plan_create_service,
            file_read_service,
            image_read_service,
            file_search_service,
            file_remove_service,
            file_patch_service,
            file_undo_service,
            shell_service,
            fetch_service,
            followup_service,
            mcp_service,
            env_service,
            custom_instructions_service,
            auth_service,
            chat_service,
            config_service,
            agent_registry_service,
            command_loader_service,
            policy_service,
            provider_auth_service,
            skill_service,
        }
    }
}

impl<
    F: FileReaderInfra
        + FileWriterInfra
        + CommandInfra
        + UserInfra
        + McpServerInfra
        + FileRemoverInfra
        + FileInfoInfra
        + FileDirectoryInfra
        + EnvironmentInfra
        + DirectoryReaderInfra
        + HttpInfra
        + WalkerInfra
        + Clone
        + SnapshotRepository
        + ConversationRepository
        + AppConfigRepository
        + KVStore
        + ProviderRepository
        + AgentRepository
        + SkillRepository
        + StrategyFactory
        + Clone
        + 'static,
> Services for LucardServices<F>
{
    type ProviderService = LucardProviderService<F>;
    type AppConfigService = LucardAppConfigService<F>;
    type ConversationService = LucardConversationService<F>;
    type TemplateService = LucardTemplateService<F>;
    type ProviderAuthService = LucardProviderAuthService<F>;

    fn provider_auth_service(&self) -> &Self::ProviderAuthService {
        &self.provider_auth_service
    }
    type AttachmentService = LucardChatRequest<F>;
    type EnvironmentService = LucardEnvironmentService<F>;
    type CustomInstructionsService = LucardCustomInstructionsService<F>;
    type WorkflowService = LucardWorkflowService<F>;
    type FileDiscoveryService = LucardDiscoveryService<F>;
    type McpConfigManager = LucardMcpManager<F>;
    type FsCreateService = LucardFsCreate<F>;
    type PlanCreateService = LucardPlanCreate<F>;
    type FsPatchService = LucardFsPatch<F>;
    type FsReadService = LucardFsRead<F>;
    type ImageReadService = LucardImageRead<F>;
    type FsRemoveService = LucardFsRemove<F>;
    type FsSearchService = LucardFsSearch<F>;
    type FollowUpService = LucardFollowup<F>;
    type FsUndoService = LucardFsUndo<F>;
    type NetFetchService = LucardFetch;
    type ShellService = LucardShell<F>;
    type McpService = McpService<F>;
    type AuthService = AuthService<F>;
    type AgentRegistry = LucardAgentRegistryService<F>;
    type CommandLoaderService = LucardCommandLoaderService<F>;
    type PolicyService = LucardPolicyService<F>;
    type SkillFetchService = LucardSkillFetch<F>;

    fn provider_service(&self) -> &Self::ProviderService {
        &self.chat_service
    }

    fn config_service(&self) -> &Self::AppConfigService {
        &self.config_service
    }

    fn conversation_service(&self) -> &Self::ConversationService {
        &self.conversation_service
    }

    fn template_service(&self) -> &Self::TemplateService {
        &self.template_service
    }

    fn attachment_service(&self) -> &Self::AttachmentService {
        &self.attachment_service
    }

    fn environment_service(&self) -> &Self::EnvironmentService {
        &self.env_service
    }
    fn custom_instructions_service(&self) -> &Self::CustomInstructionsService {
        &self.custom_instructions_service
    }

    fn workflow_service(&self) -> &Self::WorkflowService {
        self.workflow_service.as_ref()
    }

    fn file_discovery_service(&self) -> &Self::FileDiscoveryService {
        self.discovery_service.as_ref()
    }

    fn mcp_config_manager(&self) -> &Self::McpConfigManager {
        self.mcp_manager.as_ref()
    }

    fn fs_create_service(&self) -> &Self::FsCreateService {
        &self.file_create_service
    }

    fn plan_create_service(&self) -> &Self::PlanCreateService {
        &self.plan_create_service
    }

    fn fs_patch_service(&self) -> &Self::FsPatchService {
        &self.file_patch_service
    }

    fn fs_read_service(&self) -> &Self::FsReadService {
        &self.file_read_service
    }

    fn image_read_service(&self) -> &Self::ImageReadService {
        &self.image_read_service
    }

    fn fs_remove_service(&self) -> &Self::FsRemoveService {
        &self.file_remove_service
    }

    fn fs_search_service(&self) -> &Self::FsSearchService {
        &self.file_search_service
    }

    fn follow_up_service(&self) -> &Self::FollowUpService {
        &self.followup_service
    }

    fn fs_undo_service(&self) -> &Self::FsUndoService {
        &self.file_undo_service
    }

    fn net_fetch_service(&self) -> &Self::NetFetchService {
        &self.fetch_service
    }

    fn shell_service(&self) -> &Self::ShellService {
        &self.shell_service
    }

    fn mcp_service(&self) -> &Self::McpService {
        &self.mcp_service
    }

    fn auth_service(&self) -> &Self::AuthService {
        self.auth_service.as_ref()
    }

    fn agent_registry(&self) -> &Self::AgentRegistry {
        &self.agent_registry_service
    }

    fn command_loader_service(&self) -> &Self::CommandLoaderService {
        &self.command_loader_service
    }

    fn policy_service(&self) -> &Self::PolicyService {
        &self.policy_service
    }

    fn skill_fetch_service(&self) -> &Self::SkillFetchService {
        &self.skill_service
    }
}
