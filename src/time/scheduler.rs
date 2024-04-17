pub struct TachyonTask {
    pub id: u16,
    pub delay: Option<u64>,
    pub repeat_interval: Option<u64>,
    pub sync: bool,
    pub executor: Box<dyn Fn()>
}

impl TachyonTask {
    pub(crate) fn execute(&mut self) {
        if self.sync {
            self.executor();
        } else {
            tokio::spawn(async move {
                self.executor();
            });
        }
    }
}

pub trait TaskScheduler {
    fn run_sync_task<F: Fn() + 'static>(
        &mut self,
        delay: u64,
        interval: u64,
        executor: F
    );

    fn run_async_task<F: Fn() + 'static>(
        &mut self,
        delay: u64,
        interval: u64,
        executor: F
    );

    fn run_task(&mut self, task: TachyonTask);

    fn get_tasks(&self) -> Vec<TachyonTask>;
}

struct DefaultSchedulerManager {
    tasks: Vec<TachyonTask>
}

impl TaskScheduler for DefaultSchedulerManager {
    fn run_sync_task<F: Fn() + 'static>(
        &mut self,
        delay: u64,
        interval: u64,
        executor: F
    ) {
        self.run_task(TachyonTask {
            id: 0,
            delay: Some(delay),
            repeat_interval: Some(interval),
            sync: true,
            executor: Box::new(executor)
        });
    }

    fn run_async_task<F: Fn() + 'static>(
        &mut self,
        delay: u64,
        interval: u64,
        executor: F
    ) {
        self.run_task(TachyonTask {
            id: 0,
            delay: Some(delay),
            repeat_interval: Some(interval),
            sync: false,
            executor: Box::new(executor)
        });
    }

    fn run_task(&mut self, task: TachyonTask) {
        self.tasks.push(task);
    }

    fn get_tasks(&self) -> Vec<TachyonTask> {
        self.tasks.clone()
    }
}