use std::path::Path;

use anyhow::Result;
use lucard_app::FileInfoInfra;

pub struct LucardFileMetaService;
#[async_trait::async_trait]
impl FileInfoInfra for LucardFileMetaService {
    async fn is_file(&self, path: &Path) -> Result<bool> {
        Ok(lucard_common::fs::LucardFS::is_file(path))
    }

    async fn is_binary(&self, path: &Path) -> Result<bool> {
        lucard_common::fs::LucardFS::is_binary_file(path).await
    }

    async fn exists(&self, path: &Path) -> Result<bool> {
        Ok(lucard_common::fs::LucardFS::exists(path))
    }

    async fn file_size(&self, path: &Path) -> Result<u64> {
        lucard_common::fs::LucardFS::file_size(path).await
    }
}
