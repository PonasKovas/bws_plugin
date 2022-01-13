use async_ffi::ContextExt;
use std::future::poll_fn;

use crate::{prelude::BwsTuple3, SendPtr};

/// Given a receiver asynchronously receives an event
///
/// VTable must be initialized
pub async unsafe fn receive_event(
    receiver: SendPtr<()>,
) -> Option<(u32, SendPtr<()>, SendPtr<()>)> {
    poll_fn(|ctx| {
        ctx.with_ffi_context(|ffi_ctx| unsafe {
            (crate::vtable::VTABLE.poll_recv_plugin_event)(receiver, ffi_ctx)
                .try_into_poll()
                .expect("receiving a plugin event panicked")
        })
    })
    .await
    .into_option()
    .map(|t| (t.0, t.1, t.2))
}

pub unsafe fn finish_event_handling(oneshot_sender: SendPtr<()>) -> bool {
    unsafe { (crate::vtable::VTABLE.fire_oneshot_plugin_event)(oneshot_sender) }
}
