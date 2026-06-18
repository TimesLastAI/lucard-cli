use std::path::Path;

use lucard_app::FileRemoverInfra;

/// Low-level file remove service
///
/// Provides primitive file deletion operations without snapshot coordination.
/// Snapshot management should be handled at the service layer.
#[derive(Default)]
pub struct LucardFileRemoveService;

impl LucardFileRemoveService {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl FileRemoverInfra for LucardFileRemoveService {
    async fn remove(&self, path: &Path) -> anyhow::Result<()> {
        Ok(lucard_common::fs::LucardFS::remove_file(path).await?)
    }
}
