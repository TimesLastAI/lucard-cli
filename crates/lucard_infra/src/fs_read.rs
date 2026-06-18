use std::path::Path;

use anyhow::Result;
use lucard_app::FileReaderInfra;

pub struct LucardFileReadService;

impl Default for LucardFileReadService {
    fn default() -> Self {
        Self
    }
}

impl LucardFileReadService {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl FileReaderInfra for LucardFileReadService {
    async fn read_utf8(&self, path: &Path) -> Result<String> {
        lucard_common::fs::LucardFS::read_utf8(path).await
    }

    async fn read(&self, path: &Path) -> Result<Vec<u8>> {
        lucard_common::fs::LucardFS::read(path).await
    }

    async fn range_read_utf8(
        &self,
        path: &Path,
        start_line: u64,
        end_line: u64,
    ) -> Result<(String, lucard_domain::FileInfo)> {
        lucard_common::fs::LucardFS::read_range_utf8(path, start_line, end_line).await
    }
}
