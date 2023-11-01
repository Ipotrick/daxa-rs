mod device;
mod instance;
mod pipeline;
mod types;
mod swapchain;
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
    }
}
