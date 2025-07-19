use std::{
    future::{self, Future},
    pin::Pin,
    task::{Context, Poll},
};

use async_runtime::{
    executor::{self, Executor},
    waker,
};

pub struct CountingFuture {
    pub count: i32,
}

impl Future for CountingFuture {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;

        if self.count == 4 {
            println!("Counting future is done!");
            Poll::Ready(self.count)
        } else {
            cx.waker().wake_by_ref();
            println!("Counting future is not done yet! {}", self.count);
            Poll::Pending
        }
    }
}

fn main() {
    let counter = CountingFuture { count: -10 };
    let counter_two = CountingFuture { count: -5 };

    let mut executor = Executor::new();

    let handle = executor.spawn(counter);
    let handle_two = executor.spawn(counter_two);

    std::thread::spawn(move || {
        loop {
            executor.poll();
        }
    });

    let result = handle.block_on().unwrap();
    let result_2 = handle_two.block_on().unwrap();

    println!("Results: {} - {}", result, result_2);
}
