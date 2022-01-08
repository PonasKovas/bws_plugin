pub mod register;
pub mod stable_types;
pub mod vtable;

// relics from the past, but not gonna delete in case I need them in the future

// #[repr(transparent)]
// #[derive(Debug, Copy, Clone)]
// pub struct SendMutPtr<T>(pub *mut T);

// unsafe impl<T> Send for SendMutPtr<T> {}
// unsafe impl<T> Sync for SendMutPtr<T> {}

// #[repr(transparent)]
// #[derive(Debug, Copy, Clone)]
// pub struct SendPtr<T>(pub *const T);

// unsafe impl<T> Send for SendPtr<T> {}
// unsafe impl<T> Sync for SendPtr<T> {}

/// A gate for the plugin side
/// Basically a receiver for an events channel
#[repr(C)]
pub struct BwsPluginGate {
    receiver: *const (),
}

pub mod prelude {
    pub use crate::register::Plugin;
    pub use crate::stable_types::{
        option::BwsOption,
        slice::BwsSlice,
        string::{BwsStr, BwsString},
        tuples::{BwsTuple2, BwsTuple3, BwsTuple4, BwsTuple5},
        unit::{unit, BwsUnit},
        vec::BwsVec,
    };
    // pub use crate::{SendMutPtr, SendPtr};
}

pub const BWS_PLUGIN_ABI_VERSION: u16 = 0;
