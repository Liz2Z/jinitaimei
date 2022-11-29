#![allow(unused)]
#![allow(deprecated)]

use futures::{executor::block_on, join};
use std::fs;
use std::future::Future;
use std::task::{Context, Poll};
use std::thread::{sleep, JoinHandle};
use std::time::{Duration, SystemTime};

fn main() {
    println!("Hello, world!");

    block_on(async_entry());
}

async fn async_entry() {
    print!("async_entry start  \n{:#?}", SystemTime::now());

    join!(async_three(), async_two(), async_one());

    print!("async_entry done  \n{:#?}", SystemTime::now())
}

async fn async_one() {
    print!("async one start  \n{:#?}", SystemTime::now());
    Timer::new(1000).await;
    print!("async one done  \n{:#?}", SystemTime::now())
}

async fn async_two() {
    print!("async two start {:#?}", SystemTime::now());
    Timer::new(5000).await;
    print!("async two done \n{:#?}", SystemTime::now());
}

async fn async_three() {
    print!("async three start {:#?}", SystemTime::now());
    sleep(Duration::from_millis(5000));
    print!("async three done \n{:#?}", SystemTime::now());
    Timer::new(5000).await;
}

fn test() {
    print!("xxx")
}

struct Timer {
    timeout: Duration,
    current: SystemTime,
}

impl Timer {
    fn new(timeout: u64) -> Timer {
        Timer {
            timeout: Duration::from_millis(timeout),
            current: SystemTime::now(),
        }
    }
}

impl Future for Timer {
    type Output = u32;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let end = self.current.checked_add(self.timeout).unwrap();

        if SystemTime::now().gt(&end) {
            Poll::Ready(2)
        } else {
            cx.waker().clone().wake();

            Poll::Pending
        }
    }
}
