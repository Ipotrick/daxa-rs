mod types;
mod instance;
mod device;
pub use types::*;
pub use instance::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        let _instance = crate::Instance::new(&crate::InstanceInfo{flags: crate::InstanceFlags::DEBUG_UTIL}).unwrap();
    }
}
