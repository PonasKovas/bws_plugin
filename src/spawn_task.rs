use crate::prelude::*;
use async_ffi::FfiFuture;

pub fn spawn_task(future: FfiFuture<BwsUnit>) {
    unsafe { (crate::vtable::VTABLE.spawn_task)(future) }
}
