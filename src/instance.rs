use std::mem;

use crate::types::*;
use bitflags::bitflags;
use daxa_sys;

pub struct Instance {
    instance: daxa_sys::daxa_Instance,
}

bitflags! {
    #[derive(Default)]
    pub struct InstanceFlags: u64 {
        const DEBUG_UTIL = daxa_sys::DAXA_INSTANCE_FLAG_DEBUG_UTIL;
    }
}

#[repr(C)]
#[derive(Default)]
pub struct InstanceInfo {
    pub flags: InstanceFlags,
}

#[derive(Debug)]
pub enum InstanceCreateError {
    MissingExtension,
    Unknown,
}

impl std::fmt::Display for InstanceCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", &self)
    }
}
impl std::error::Error for InstanceCreateError {}

impl Instance {
    pub fn new(info: &InstanceInfo) -> std::result::Result<Self, InstanceCreateError> {
        use crate::types::Result;
        use Result::*;
        unsafe {
            let c_info = info.as_ptr().cast::<daxa_sys::daxa_InstanceInfo>();

            let mut c_instance = std::mem::zeroed();

            let c_result = daxa_sys::daxa_create_instance(c_info, &mut c_instance);

            match mem::transmute::<Result>(c_result) {
                Success => Ok(Instance {
                    instance: c_instance,
                }),
                MissingExtension => Err(InstanceCreateError::MissingExtension),
                _ => Err(InstanceCreateError::Unknown),
            }
        }
    }

    pub fn info(&self) -> &InstanceInfo {
        unsafe {
            daxa_sys::daxa_instance_info(self.instance)
                .cast::<InstanceInfo>()
                .as_ref()
                .unwrap()
        }
    }
}

// impl Drop for Instance {
//     fn drop(&mut self) {
//         unsafe {
//             daxa_sys::daxa_dvc_destroy_instance(self.instance);
//         }
//     }
// }
