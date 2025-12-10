use crate::noop_waker::noop_waker;
use std::pin::Pin;
use std::task::{Context, Poll};

struct BusyExecutor {
    futures: Vec<Pin<Box<dyn Future<Output = ()> + Sync + 'static>>>
}

impl BusyExecutor {
    const fn new() -> Self {
        Self { futures: Vec::new() }
    }

    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Sync + 'static,
    {
        self.futures.push(Box::pin(future));
    }

    fn run(&mut self) {
        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);

        while !self.futures.is_empty() {
            self.futures.retain_mut(|future| {
                match future.as_mut().poll(&mut context) {
                    Poll::Ready(_) => false,
                    Poll::Pending => true
                }
            });
        }

        println!("Executor finished");
    }
}

static mut EXECUTOR: BusyExecutor = BusyExecutor::new();

pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + Sync + 'static,
{
    unsafe {
        let executor = std::ptr::addr_of_mut!(EXECUTOR);
        (*executor).spawn(future);
    }
}

pub fn run() {
    unsafe {
        let executor = std::ptr::addr_of_mut!(EXECUTOR);
        (*executor).run();
    }
}
