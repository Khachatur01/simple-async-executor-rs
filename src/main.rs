mod noop_waker;
mod sleep_future;
mod busy_executor;

use std::pin::Pin;
use std::task::{Context, Poll};
use crate::sleep_future::SleepFuture;
use std::time::Duration;
use crate::busy_executor::{run, spawn};


async fn async_future_sugared() {
    println!("Starting async future sugared");

    SleepFuture::new(Duration::from_millis(2000)).await;
    println!("Hello, world! after 2 seconds");
    SleepFuture::new(Duration::from_millis(5000)).await;
    println!("Hello, world! after 5 seconds");
}

fn async_future_desugared() -> impl Future<Output = ()> {
    enum AsyncFutureState {
        Started,
        Waiting1(SleepFuture),
        Waiting2(SleepFuture),
        Finished
    }

    struct AsyncFuture {
        state: AsyncFutureState
    }
    impl AsyncFuture {
        pub fn new() -> Self {
            Self { state: AsyncFutureState::Started }
        }
    }

    impl Future for AsyncFuture {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            /* loop until sub-future is pending or all stages finished */
            loop {
                match &mut self.state {
                    AsyncFutureState::Started => {
                        println!("Starting async future desugared");
                        self.state = AsyncFutureState::Waiting1(SleepFuture::new(Duration::from_millis(2000)));
                    },
                    AsyncFutureState::Waiting1(future) => {
                        match std::pin::pin!(future).poll(cx) {
                            Poll::Pending => return Poll::Pending,
                            Poll::Ready(_) => {
                                println!("Hello, world! after 2 seconds");
                                self.state = AsyncFutureState::Waiting2(SleepFuture::new(Duration::from_millis(5000)));
                            }
                        }
                    }
                    AsyncFutureState::Waiting2(future) => {
                        match std::pin::pin!(future).poll(cx) {
                            Poll::Pending => return Poll::Pending,
                            Poll::Ready(_) => {
                                println!("Hello, world! after 5 seconds");
                                self.state = AsyncFutureState::Finished;
                            }
                        }
                    }
                    AsyncFutureState::Finished => return Poll::Ready(()),
                }
            }
        }
    }

    AsyncFuture::new()
}

fn main() {
    spawn(async_future_sugared());
    spawn(async_future_desugared());

    run();
}
