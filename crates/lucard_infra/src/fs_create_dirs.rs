use std::path::Path;

use lucard_app::FileDirectoryInfra;

#[derive(Default)]
pub struct LucardCreateDirsService;

#[async_trait::async_trait]
impl FileDirectoryInfra for LucardCreateDirsService {
    async fn create_dirs(&self, path: &Path) -> anyhow::Result<()> {
        Ok(lucard_common::fs::LucardFS::create_dir_all(path).await?)
    }
}
