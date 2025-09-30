mod tasks;
use app_utils::app_error::ApplicationError;
use async_trait::async_trait;
use log::{error, info};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::{interval, timeout};

type ResultTask<T = ()> = Result<T, ApplicationError>;

#[async_trait]
pub trait BackgroundTask {
    const INTERVAL_SEC: u64;
    const TASK_NAME: &'static str;

    async fn run(&mut self) -> ResultTask;
    fn name(&self) -> &'static str {
        Self::TASK_NAME
    }
    fn interval(&self) -> Duration {
        Duration::from_secs(Self::INTERVAL_SEC)
    }
}

#[derive(Default)]
pub struct BackgroundManager {
    is_running: Arc<AtomicBool>,
    task_handles: Vec<JoinHandle<()>>,
}

impl BackgroundManager {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            task_handles: Vec::new(),
        }
    }

    pub fn start(&mut self, app_handle: Arc<tauri::AppHandle>) -> ResultTask {
        if self
            .is_running
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return Ok(()); // Just started
        }
        info!("Starting Background Manager");

        self.spawn_task(tasks::IdStanzeProcessor::new(app_handle));

        info!(
            "Starting Background Manager with {} tasks",
            self.task_handles.len()
        );
        Ok(())
    }

    fn spawn_task<T>(&mut self, task: T)
    where
        T: BackgroundTask + Send + Sync + 'static,
    {
        let is_running_clone = self.is_running.clone();
        let handle = tokio::spawn(async move {
            Self::run_for_loop(task, is_running_clone).await;
        });

        self.task_handles.push(handle);
    }

    async fn run_for_loop<T>(mut task: T, is_running: Arc<AtomicBool>)
    where
        T: BackgroundTask + Send + Sync + 'static,
    {
        let mut interval_timer = interval(task.interval());
        loop {
            interval_timer.tick().await;

            if !is_running.load(Ordering::SeqCst) {
                info!("Task stopped: {}", task.name());
                break;
            }

            if let Err(e) = task.run().await {
                error!("Error in task {}: {}", task.name(), e);
            }
        }
    }

    pub async fn stop(&mut self) {
        if !self.is_running.load(Ordering::SeqCst) {
            return;
        }

        self.is_running.store(false, Ordering::SeqCst);
        info!("Stopping Background Manager ...");

        let handles = std::mem::take(&mut self.task_handles);
        for (i, handle) in handles.into_iter().enumerate() {
            match timeout(Duration::from_secs(5), handle).await {
                Ok(Ok(_)) => {
                    info!("Task {} stopped successfully", i);
                }
                Ok(Err(e)) => {
                    error!("Task {} stopped with an error: {}", i, e);
                }
                Err(_) => {
                    error!("Timeout in waiting for task {} stop", i);
                }
            }
        }

        info!("Stopped Background Manager");
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    pub fn task_count(&self) -> usize {
        self.task_handles.len()
    }
}
