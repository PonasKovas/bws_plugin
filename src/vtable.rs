use crate::{prelude::*, register::PluginEntrySignature, LogLevel};
use async_ffi::{FfiContext, FfiFuture, FfiPoll};

// bigass macro, but I hope to have saved time in the long run 😩
// basically creates a "default" VTable with all functions just panicking
// with a message that the VTable hasn't been initialized yet.
//
// would be a lot of repetition to do it manually
macro_rules! vtable {
    {
        $(#[$struct_meta:meta])*
        $type_vis:vis struct $type:ident {
            $(
                $(#[$function_meta:meta])*
                $field_vis:vis $func:ident: unsafe extern "C" fn( $($arg:ty),* $(,)? ) $(-> $ret:ty)?
            ),* $(,)?
        }
    } => {
        $(#[$struct_meta])*
        $type_vis struct $type {
            $(
                $(#[$function_meta])*
                $field_vis $func: unsafe extern "C" fn($($arg),*) $(-> $ret)?,
            )*
        }

        impl $type {
            pub const DEFAULT: Self = Self {
                $($func: {
                    unsafe extern "C" fn not_set( $( _: $arg ),* ) $(-> $ret)? {
                        panic!("VTable not set. Hint: make sure to bws_plugin::vtable::init() before using any methods from bws_plugin.");
                    }
                    not_set
                }),*
            };
        }
    };
}

vtable! {
    #[repr(C)]
    #[derive(Clone)]
    pub struct BwsVTable {
        /// Takes:
        /// 1. A pointer to the receiver
        /// 2. FfiContext reference
        ///
        /// Returns:
        /// `None` if the channel is dead and no more events can be received.
        /// A plugin event id, data pointer and a pointer to the oneshot channel for signaling end of event handling.
        pub poll_recv_plugin_event:
            unsafe extern "C" fn(
                SendPtr<()>,
                &mut FfiContext,
            ) -> FfiPoll<BwsOption<BwsTuple3<u32, SendPtr<()>, SendPtr<()>>>>,
        /// Takes:
        /// 1. A pointer to the oneshot sender
        ///
        /// Returns:
        /// True if oneshot fires successfully, otherwise false.
        pub fire_oneshot_plugin_event: unsafe extern "C" fn(SendPtr<()>) -> bool,
        /// Takes:
        /// 1. The name of the plugin that's calling
        /// 2. A string to log
        /// 3. Log level
        pub log: unsafe extern "C" fn(BwsStr<'static>, BwsStr<'static>, LogLevel),
        /// Takes:
        /// 1. An FfiFuture for the task to run
        pub spawn_task: unsafe extern "C" fn(FfiFuture<BwsUnit>),
    }
}

pub(crate) static mut VTABLE: BwsVTable = BwsVTable::DEFAULT;

pub fn init(vtable: BwsVTable) {
    unsafe {
        VTABLE = vtable;
    }
}
