use crate::prelude::*;
use crate::vtable::BwsVTable;
use async_ffi::FfiFuture;

#[derive(Clone)]
pub struct Plugin {
    pub name: String,
    pub version: (u64, u64, u64),
    pub dependencies: Vec<(String, String)>,
    pub entry: PluginEntrySignature,
}

pub type PluginEntrySignature = unsafe extern "C" fn(
    // name of the plugin
    BwsString,
    BwsVTable,
    // A pointer to the receiver of the events channel for the plugin side
    SendPtr<()>,
) -> FfiFuture<BwsUnit>;

/// used to pass plugin information to the host
#[repr(C)]
pub struct RegPluginStruct {
    pub name: BwsString,
    pub version: BwsTuple3<u64, u64, u64>,
    pub dependencies: BwsVec<BwsTuple2<BwsString, BwsString>>,
    pub entry: PluginEntrySignature,
}

/// Builder for registering a plugin conveniently
impl Plugin {
    pub fn new(
        name: impl AsRef<str>,
        version: (u64, u64, u64),
        entry: PluginEntrySignature,
    ) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            version,
            dependencies: Vec::new(),
            entry,
        }
    }
    pub fn add_dep(
        mut self,
        dependency_name: impl AsRef<str>,
        dependency_version_req: impl AsRef<str>,
    ) -> Self {
        self.dependencies.push((
            dependency_name.as_ref().to_owned(),
            dependency_version_req.as_ref().to_owned(),
        ));

        self
    }
    pub fn register(self, reg_fn: unsafe extern "C" fn(RegPluginStruct)) {
        unsafe {
            reg_fn(RegPluginStruct {
                name: BwsString::from_string(self.name),
                version: BwsTuple3(self.version.0, self.version.1, self.version.2),
                dependencies: BwsVec::from_vec(
                    self.dependencies
                        .into_iter()
                        .map(|(name, version_req)| {
                            BwsTuple2(
                                BwsString::from_string(name),
                                BwsString::from_string(version_req),
                            )
                        })
                        .collect::<Vec<_>>(),
                ),
                entry: self.entry,
            })
        }
    }
}
