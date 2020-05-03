use std::future::Future;
use std::thread;

use crossbeam::channel::{unbounded, Sender};
use futures::executor;
use once_cell::sync::Lazy;

static QUEUE: Lazy<Sender<async_task::Task<()>>> = Lazy::new(|| {
    let (sender, receiver) = unbounded::<async_task::Task<()>>();
    for _ in 0..4 {
        let recv = receiver.clone();

        thread::spawn(|| {
            for task in recv {
                task.run(); 
            }
        });
    }

    sender
});

fn spawn<F, R>(future: F) -> async_task::JoinHandle<R, ()>
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    let schedule = |task| QUEUE.send(task).unwrap();
    let (task, handle) = async_task::spawn(future, schedule, ());

    task.schedule();

    handle
}

fn main() {
    let handles: Vec<_> = (0..10).map(|i| {
        spawn(async move {
            println!("Hello from task {}", i);
        })
    }).collect();

    // Wait for the tasks to finish.
    for handle in handles {
        executor::block_on(handle);
    }
}
