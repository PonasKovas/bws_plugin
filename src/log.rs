use crate::{prelude::BwsStr, LogLevel};
use std::mem::transmute;

pub fn log(msg: impl AsRef<str>, level: LogLevel) {
    // transmute to make it 'static
    let msg: BwsStr<'static> = unsafe { transmute(BwsStr::from_str(msg.as_ref())) };
    unsafe { (crate::vtable::VTABLE.log)(msg, level) }
}
