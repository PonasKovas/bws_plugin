use crate::{prelude::*, register::PluginEntrySignature};
use async_ffi::{FfiContext, FfiPoll};

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
    }
}

pub(crate) static mut VTABLE: BwsVTable = BwsVTable::DEFAULT;

pub fn init(vtable: BwsVTable) {
    unsafe {
        VTABLE = vtable;
    }
}
