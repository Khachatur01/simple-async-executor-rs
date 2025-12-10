// --- 1. The Infrastructure: A Dummy Waker ---

use std::task::{RawWaker, RawWakerVTable, Waker};

// We need a Waker that does nothing, just to pass into poll().
// Boilerplate to create a RawWaker vtable (virtual function table).
fn noop_clone(_: *const ()) -> RawWaker {
    noop_raw_waker()
}
fn noop(_: *const ()) {} // Reset and wake do nothing

const VTABLE: RawWakerVTable = RawWakerVTable::new(
    noop_clone, // clone
    noop,       // wake
    noop,       // wake_by_ref
    noop,       // drop
);

fn noop_raw_waker() -> RawWaker {
    RawWaker::new(std::ptr::null(), &VTABLE)
}

pub fn noop_waker() -> Waker {
    // UNSAFE: We promise the VTABLE contract is upheld (it is, casually).
    unsafe { Waker::from_raw(noop_raw_waker()) }
}
