use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use lucard_domain::{Environment, Snapshot, SnapshotRepository};

pub struct LucardFileSnapshotService {
    inner: Arc<lucard_services::snaps::SnapshotService>,
}

impl LucardFileSnapshotService {
    pub fn new(env: Environment) -> Self {
        Self {
            inner: Arc::new(lucard_services::snaps::SnapshotService::new(
                env.snapshot_path(),
            )),
        }
    }
}

#[async_trait::async_trait]
impl SnapshotRepository for LucardFileSnapshotService {
    // Creation
    async fn insert_snapshot(&self, file_path: &Path) -> Result<Snapshot> {
        self.inner.create_snapshot(file_path.to_path_buf()).await
    }

    // Undo
    async fn undo_snapshot(&self, file_path: &Path) -> Result<()> {
        self.inner.undo_snapshot(file_path.to_path_buf()).await
    }
}
