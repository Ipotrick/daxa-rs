#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        unsafe {
            let instance_info = daxa_sys::daxa_InstanceInfo{
                flags: 0,
            };
            let mut instance : daxa_sys::daxa_Instance = std::ptr::null_mut();
            daxa_sys::daxa_create_instance(&instance_info, &mut instance);
            daxa_sys::daxa_destroy_instance(instance);
        }
    }
}
