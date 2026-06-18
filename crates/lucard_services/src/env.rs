use std::sync::Arc;

use lucard_app::domain::Environment;
use lucard_app::{EnvironmentInfra, EnvironmentService};

pub struct LucardEnvironmentService<F>(Arc<F>);

impl<F> LucardEnvironmentService<F> {
    pub fn new(infra: Arc<F>) -> Self {
        Self(infra)
    }
}

impl<F: EnvironmentInfra> EnvironmentService for LucardEnvironmentService<F> {
    fn get_environment(&self) -> Environment {
        self.0.get_environment()
    }

    fn is_restricted(&self) -> bool {
        self.0.is_restricted()
    }
}
