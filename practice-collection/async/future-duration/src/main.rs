use std::time::{ Duration,Instant };
use std::{
    pin::Pin, 
    task::{ Context,Poll},
    thread::{ spawn,sleep,JoinHandle}
};
use futures::{
    prelude::*,
};

pub struct Delay {
    dur: Duration,
    handle: Option<JoinHandle<()>>,
}

impl Delay {
    pub fn new(dur: Duration) -> Self {
        Delay {
            dur,
            handle: None,
        }
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.handle {
            None => {
                let waker = cx.waker().clone();
                let dur = self.dur.clone();

                let handle = spawn(move || {
                    sleep(dur);
                    waker.wake();
                });
                self.handle = Some(handle);

                Poll::Pending
            }

            Some(_) => {
                Poll::Ready(())
            }
        }
    }
}

fn main(){
    let mut list_of_delay = vec![];
    for _ in 0..1000 {
        list_of_delay.push(Delay::new(Duration::from_secs(5)));
    }
    // for i in 0..10 {
    //     list_of_delay.push(Delay::new(Duration::from_secs(i)));
    // }

    let all = futures_util::future::join_all(list_of_delay);

    let now = Instant::now();
    futures_executor::block_on(all);
    println!("time elapsed: {:?}", now.elapsed());
}