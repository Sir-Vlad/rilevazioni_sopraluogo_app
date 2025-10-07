mod task_manager;
mod tasks;

use std::sync::{Arc, OnceLock};

use app_services::service::ApplicationError;
use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::task_manager::{BackgroundManager as InnerBackgroundManager, StatusTask, TaskInfo};

pub type BackgroundManager = Arc<Mutex<InnerBackgroundManager>>;
type ResultTask<T = ()> = Result<T, ApplicationError>;

static BACKGROUND_MANAGER: OnceLock<BackgroundManager> = OnceLock::new();

/// Retrieves the singleton instance of the `BackgroundManager`.
///
/// This function ensures that there is only one instance of the
/// `BackgroundManager` throughout the application's lifetime. If the
/// `BackgroundManager` instance has not been initialized yet, it will create
/// and initialize a new instance of the inner `BackgroundManager` using a
/// thread-safe `Arc<Mutex<InnerBackgroundManager>>`.
///
/// The `BackgroundManager` instance is then returned as a clone of the existing
/// or newly initialized instance.
///
/// # Returns
///
/// * `BackgroundManager` - A thread-safe, shared wrapper of the singleton
///   `BackgroundManager`.
///
/// # Example
///
/// ```
/// let manager = get_background_manager();
/// // Use the manager for background tasks
/// ```
pub fn get_background_manager() -> BackgroundManager {
    BACKGROUND_MANAGER
        .get_or_init(|| Arc::new(Mutex::new(InnerBackgroundManager::new())))
        .clone()
}

#[async_trait]
pub(crate) trait BackgroundTask: Send + Sync + 'static {
    async fn run(&mut self) -> ResultTask<StatusTask>;
    fn info(&self) -> &TaskInfo;
    fn info_mut(&mut self) -> &mut TaskInfo;
}
