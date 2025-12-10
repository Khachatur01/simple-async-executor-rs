use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

pub struct SleepFuture {
    start: Instant,
    duration: Duration,
}

impl SleepFuture {
    pub fn new(duration: Duration) -> Self {
        Self {
            start: Instant::now(),
            duration
        }
    }
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.start + self.duration {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
