use std::fmt::{Debug, Pointer};

pub struct TachyonTask<'a> {
    pub id: u16,
    pub delay: Option<u64>,
    pub repeat_interval: Option<u64>,
    pub sync: bool,
    pub executor: Box<dyn Fn() + 'a>
}

impl TachyonTask<'_> {
    pub(crate) fn execute(&mut self) {
        if self.sync {
            (self.executor)();
        } else {
            tokio::spawn(async move {
                (self.executor)();
            });
        }
    }
}

// pub trait TaskScheduler<'a> {
//     fn run_sync_task(
//         &mut self,
//         delay: u64,
//         interval: u64,
//         executor: impl Fn() + 'a
//     );
//
//     fn run_async_task(
//         &mut self,
//         delay: u64,
//         interval: u64,
//         executor: impl Fn() + 'a
//     );
//
//     fn run_task(&mut self, task: TachyonTask);
//
//     fn get_tasks(&self) -> Vec<TachyonTask>;
// }

pub struct TaskScheduler<'a> {
    tasks: Vec<TachyonTask<'a>>
}

impl<'a> TaskScheduler<'a> {
    pub(crate) fn new() -> Self {
        TaskScheduler {
            tasks: Vec::new()
        }
    }

    pub fn run_sync_task(
        &mut self,
        delay: u64,
        interval: u64,
        executor: impl Fn() + 'a
    ) {
        self.run_task(TachyonTask {
            id: 0,
            delay: Some(delay),
            repeat_interval: Some(interval),
            sync: true,
            executor: Box::new(executor)
        });
    }

    pub fn run_async_task(
        &mut self,
        delay: u64,
        interval: u64,
        executor: impl Fn() + 'a
    ) {
        self.run_task(TachyonTask {
            id: 0,
            delay: Some(delay),
            repeat_interval: Some(interval),
            sync: false,
            executor: Box::new(executor)
        });
    }

    pub fn run_task(&mut self, task: TachyonTask) {
        self.tasks.push(task);
    }

    pub(crate) fn get_tasks(&self) -> Vec<TachyonTask> {
        self.tasks.clone()
    }
}