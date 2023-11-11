mod device;
mod instance;
mod pipeline;
mod types;
mod swapchain;
mod command_recorder;
pub use instance::*;
pub use types::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let _instance = crate::Instance::new(&crate::InstanceInfo {
            flags: crate::InstanceFlags::DEBUG_UTIL,
        })
        .unwrap();

        let device = _instance.create_device(&Default::default()).unwrap();

        let recorder = device.create_command_recorder(&Default::default()).unwrap();
    }
}
