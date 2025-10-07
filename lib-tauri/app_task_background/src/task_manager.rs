use std::{
    cmp::PartialEq,
    fmt::{Display, Formatter},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use log::{error, info};
use tauri::AppHandle;
use tokio::{sync::RwLock, task::JoinHandle, time::interval};

use crate::{BackgroundTask, ResultTask, tasks};

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub(crate) enum StatusTask {
    #[default]
    /// Task waiting to be started
    Pending,
    /// Task running
    Running,
    /// Task paused, waiting to be executed
    Paused,
    /// Task stopped, but can be restarted
    Stopped,
    /// Task finished, can be restarted
    Done,
    /// Task successfully completed, waiting to be restarted
    Completed,
    /// Task finished and remove in the background manager
    Canceled,
    /// Task finished with an error, but can be restarted
    Error,
}

impl Display for StatusTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusTask::Pending => f.write_str("Pending"),
            StatusTask::Running => f.write_str("Running"),
            StatusTask::Paused => f.write_str("Paused"),
            StatusTask::Stopped => f.write_str("Stopped"),
            StatusTask::Done => f.write_str("Done"),
            StatusTask::Completed => f.write_str("Completed"),
            StatusTask::Canceled => f.write_str("Canceled"),
            StatusTask::Error => f.write_str("Error"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct TaskInfo {
    name: String,
    interval: Duration,
    status: StatusTask,
}

impl Default for TaskInfo {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            interval: Duration::from_secs(60),
            status: StatusTask::Pending,
        }
    }
}

impl TaskInfo {
    pub fn new(name: String, interval: Option<Duration>) -> Self {
        Self {
            name,
            interval: interval.unwrap_or(Duration::from_secs(60)),
            status: StatusTask::Pending,
        }
    }

    #[inline]
    pub fn name(&self) -> String { self.name.clone() }

    #[inline]
    pub fn interval(&self) -> Duration { self.interval }

    #[inline]
    pub(crate) fn status(&self) -> StatusTask { self.status }

    pub(crate) fn set_status(&mut self, status: StatusTask) { self.status = status; }
}

#[derive(Default)]
pub struct BackgroundManager {
    is_running: Arc<AtomicBool>,
    task_handles: Vec<Arc<JoinHandle<()>>>,
    task_running: Vec<Arc<RwLock<dyn BackgroundTask>>>,
    task_paused: Vec<Arc<RwLock<dyn BackgroundTask>>>,
    task_stopped: Vec<Arc<RwLock<dyn BackgroundTask>>>,
    task_done: Vec<Arc<RwLock<dyn BackgroundTask>>>,
}

impl BackgroundManager {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            ..Self::default()
        }
    }

    pub async fn start(&mut self, app_handle: Arc<AppHandle>) -> ResultTask {
        if self
            .is_running
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return Ok(()); // Just started
        }
        info!("Starting Background Manager");

        let task = Arc::new(RwLock::new(tasks::IdStanzeProcessor::new(app_handle)));
        self.task_stopped.push(task.clone());

        self.start_task(task).await;

        info!(
            "Starting Background Manager with {} tasks",
            self.task_handles.len()
        );
        Ok(())
    }

    async fn start_task(&mut self, task: Arc<RwLock<dyn BackgroundTask>>) {
        // Rimuovi il task dalle altre liste se presente
        self.remove_task_from_lists(&task);

        // Aggiungi alla lista running
        {
            let mut task_write = task.write().await;
            task_write.info_mut().set_status(StatusTask::Running);
        }
        self.task_running.push(task.clone());

        // Crea l'handle
        let is_running_clone = self.is_running.clone();
        let task_clone = task.clone();
        let mut manager_clone = self.clone();

        let handle = tokio::spawn(async move {
            manager_clone
                .run_task_loop(task_clone, is_running_clone)
                .await;
        });

        self.task_handles.push(Arc::new(handle));
    }

    async fn run_task_loop(
        &mut self,
        task: Arc<RwLock<dyn BackgroundTask>>,
        is_running: Arc<AtomicBool>,
    ) {
        if task.read().await.info().status() != StatusTask::Running {
            return;
        }

        let mut interval_timer = interval(task.read().await.info().interval());
        // skip first tick
        interval_timer.tick().await;

        loop {
            let task_name = task.read().await.info().name();
            interval_timer.tick().await;

            if !is_running.load(Ordering::SeqCst) {
                info!("Task stopped due to manager shutdown: {}", task_name);
                break;
            }

            let result_task = {
                let mut guard = task.write().await;
                guard.run().await
            };

            match result_task {
                Ok(StatusTask::Completed) => {
                    task.write()
                        .await
                        .info_mut()
                        .set_status(StatusTask::Completed);
                    info!("Task completed: {}", task_name);
                }
                Ok(StatusTask::Done) => {
                    self.done_task(&task_name).await;
                    break;
                }
                Ok(StatusTask::Paused) => {
                    self.pause_task(&task_name).await;
                    break;
                }
                Ok(StatusTask::Stopped) => {
                    self.stop_task(&task_name).await;
                    break;
                }
                Err(e) => {
                    task.write().await.info_mut().set_status(StatusTask::Error);
                    error!("Error in task {}: {}", task_name, e);
                    break;
                }
                _ => {
                    unreachable!()
                }
            }
        }
    }

    fn remove_task_from_lists(&mut self, target_task: &Arc<RwLock<dyn BackgroundTask>>) {
        self.task_running
            .retain(|task| !Arc::ptr_eq(task, target_task));
        self.task_paused
            .retain(|task| !Arc::ptr_eq(task, target_task));
        self.task_stopped
            .retain(|task| !Arc::ptr_eq(task, target_task));
        self.task_done
            .retain(|task| !Arc::ptr_eq(task, target_task));
    }

    async fn pause_task(&mut self, task_name: &str) {
        if let Some(task) = self.find_task_in_running(task_name).await {
            self.remove_task_from_lists(&task);
            task.write().await.info_mut().set_status(StatusTask::Paused);
            self.task_paused.push(task);
            info!("Task paused: {}", task_name);
        }
    }

    async fn resume_task(&mut self, task_name: &str) {
        if let Some(task) = self.find_task_in_paused(task_name).await {
            self.start_task(task).await;
            info!("Task resumed: {}", task_name);
        }
    }

    async fn stop_task(&mut self, task_name: &str) {
        if let Some(task) = self.find_task_in_running(task_name).await {
            self.remove_task_from_lists(&task);
            task.write()
                .await
                .info_mut()
                .set_status(StatusTask::Stopped);
            self.task_stopped.push(task);
            info!("Task stopped: {}", task_name);
        }
    }

    async fn done_task(&mut self, task_name: &str) {
        if let Some(task) = self.find_task_in_running(task_name).await {
            self.remove_task_from_lists(&task);
            task.write().await.info_mut().set_status(StatusTask::Done);
            self.task_done.push(task);
            info!("Task done: {}", task_name);
        }
    }

    async fn restart_task(&mut self, task_name: &str) {
        // Cerca nelle liste stopped e done
        let task = {
            let mut task = self.find_task_in_stopped(task_name).await;
            if task.is_none() {
                task = self.find_task_in_done(task_name).await;
            }
            task
        };

        if let Some(task) = task {
            self.start_task(task).await;
            info!("Task restarted: {}", task_name);
        }
    }

    async fn find_task_in_running(
        &self,
        task_name: &str,
    ) -> Option<Arc<RwLock<dyn BackgroundTask>>> {
        for task in &self.task_running {
            if task.read().await.info().name() == task_name {
                return Some(task.clone());
            }
        }
        None
    }

    async fn find_task_in_paused(
        &self,
        task_name: &str,
    ) -> Option<Arc<RwLock<dyn BackgroundTask>>> {
        for task in &self.task_paused {
            if task.read().await.info().name() == task_name {
                return Some(task.clone());
            }
        }
        None
    }

    async fn find_task_in_stopped(
        &self,
        task_name: &str,
    ) -> Option<Arc<RwLock<dyn BackgroundTask>>> {
        for task in &self.task_stopped {
            if task.read().await.info().name() == task_name {
                return Some(task.clone());
            }
        }
        None
    }

    async fn find_task_in_done(&self, task_name: &str) -> Option<Arc<RwLock<dyn BackgroundTask>>> {
        for task in &self.task_done {
            if task.read().await.info().name() == task_name {
                return Some(task.clone());
            }
        }
        None
    }

    pub async fn stop(&mut self) {
        if !self.is_running.load(Ordering::SeqCst) {
            return;
        }

        self.is_running.store(false, Ordering::SeqCst);
        info!("Stopping Background Manager ...");

        let graceful_timeout = Duration::from_secs(5);
        let start_time = std::time::Instant::now();

        while !self.task_handles.is_empty() && start_time.elapsed() < graceful_timeout {
            self.task_handles.retain(|handle| !handle.is_finished());
            if !self.task_handles.is_empty() {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }

        if !self.task_handles.is_empty() {
            info!("Force aborting {} remaining tasks", self.task_handles.len());

            for (i, handle_arc) in self.task_handles.iter().enumerate() {
                handle_arc.abort();
                info!("Task {} aborted", i);
            }

            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        self.task_handles.clear();
        self.task_running.clear();

        info!("Stopped Background Manager");
    }
}

impl Clone for BackgroundManager {
    fn clone(&self) -> Self {
        Self {
            is_running: self.is_running.clone(),
            task_handles: self.task_handles.clone(),
            task_running: self.task_running.clone(),
            task_paused: self.task_paused.clone(),
            task_stopped: self.task_stopped.clone(),
            task_done: self.task_done.clone(),
        }
    }
}
