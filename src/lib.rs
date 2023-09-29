#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        unsafe {
            let instance_info = daxa_sys::daxa_InstanceInfo { flags: 0 };
            let mut instance = std::ptr::null_mut();
            let res = daxa_sys::daxa_create_instance(&instance_info, &mut instance);
            if res != daxa_sys::daxa_Result_DAXA_RESULT_SUCCESS {
                panic!("Failed to create instance.");
            }
            daxa_sys::daxa_destroy_instance(instance);
        }
    }
}
