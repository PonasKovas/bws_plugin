use crate::prelude::*;
use async_ffi::FfiFuture;

/// Initiates the shutdown of the whole server
pub fn shutdown() {
    unsafe {
        (crate::vtable::VTABLE.shutdown)();
    }
}

/// Makes the program wait for the atomic increase on shutdown so you have time
/// to gracefully exit
/// The future finishes when a shutdown is initiated, use `gracefully_exited()`
/// with the pointer to signal that you finished cleaning up
pub fn register_for_graceful_shutdown() -> (FfiFuture<BwsUnit>, SendPtr<()>) {
    let x = unsafe { (crate::vtable::VTABLE.register_for_graceful_shutdown)() };
    (x.0, x.1)
}

pub fn gracefully_exited(atomic: SendPtr<()>) {
    unsafe {
        (crate::vtable::VTABLE.gracefully_exited)(atomic);
    }
}
