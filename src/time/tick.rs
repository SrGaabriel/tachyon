use std::time::Duration;
use tokio::time::Instant;
use crate::server::TachyonServer;
use crate::time::scheduler::TaskScheduler;

pub(crate) struct GameTimeManager {
    pub(crate) start_time: Option<Instant>
}

impl GameTimeManager {
    pub(crate) fn new() -> Self {
        GameTimeManager {
            start_time: None
        }
    }

    pub(crate) fn start(&mut self, scheduler: &TaskScheduler) {
        self.start_time = Some(Instant::now());
        tokio::spawn(async move {
            let mut tick = 0u64;
            loop {
                scheduler.get_tasks().iter_mut().for_each(|task| {
                    if task.repeat_interval.is_some() {
                        let is_interval = tick % task.repeat_interval.unwrap() == 0;
                        if is_interval {
                            task.execute();
                        }
                    }
                });
                tokio::time::sleep(Duration::from_secs(1/20)).await;
                tick += 1;
            }
        });
    }
}

