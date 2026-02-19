use std::sync::Arc;
use anyhow::Result;

pub struct BrawlersUseCase<T>
where
    T: Send + Sync,
{
    brawler_repository: Arc<T>,
}

impl<T> BrawlersUseCase<T>
where
    T: Send + Sync,
{
    pub fn new(brawler_repository: Arc<T>) -> Self {
        Self { brawler_repository }
    }
}
