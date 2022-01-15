#![feature(future_poll_fn)]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod log;
pub mod receive_event;
pub mod register;
pub mod shutdown;
pub mod spawn_task;
pub mod stable_types;
pub mod vtable;

pub use log::log;
pub use receive_event::{finish_event_handling, receive_event};
pub use spawn_task::spawn_task;

#[repr(C)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SendMutPtr<T>(pub *mut T);

unsafe impl<T> Send for SendMutPtr<T> {}
unsafe impl<T> Sync for SendMutPtr<T> {}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SendPtr<T>(pub *const T);

unsafe impl<T> Send for SendPtr<T> {}
unsafe impl<T> Sync for SendPtr<T> {}

pub mod prelude {
    pub use crate::stable_types::{
        option::BwsOption,
        slice::BwsSlice,
        string::{BwsStr, BwsString},
        tuples::{BwsTuple2, BwsTuple3, BwsTuple4, BwsTuple5},
        unit::{unit, BwsUnit},
        vec::BwsVec,
    };
    pub use crate::{SendMutPtr, SendPtr};
}

pub const ABI_VERSION: u16 = 0;
