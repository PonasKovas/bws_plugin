use crate::{prelude::BwsStr, LogLevel};
use std::mem::transmute;

pub fn log(plugin_name: impl AsRef<str>, msg: impl AsRef<str>, level: LogLevel) {
    // transmute to make them 'static
    let plugin_name: BwsStr<'static> = unsafe { transmute(BwsStr::from_str(plugin_name.as_ref())) };
    let msg: BwsStr<'static> = unsafe { transmute(BwsStr::from_str(msg.as_ref())) };
    unsafe { (crate::vtable::VTABLE.log)(plugin_name, msg, level) }
}
