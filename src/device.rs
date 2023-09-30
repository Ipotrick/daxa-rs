use bitflags::bitflags;

#[repr(i32)]
pub enum DeviceType {
    Other = daxa_sys::daxa_Result_DAXA_DEVICE_TYPE_OTHER,
    IntegratedGpu = daxa_sys::daxa_Result_DAXA_DEVICE_TYPE_INTEGRATED_GPU,
    DiscreteGpu = daxa_sys::daxa_Result_DAXA_DEVICE_TYPE_DISCRETE_GPU,
    VirtualGpu = daxa_sys::daxa_Result_DAXA_DEVICE_TYPE_VIRTUAL_GPU,
    Cpu = daxa_sys::daxa_Result_DAXA_DEVICE_TYPE_CPU,
}

bitflags! {
    #[derive(Default)]
    pub struct DeviceFlags: u64 {
        const BUFFER_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT = daxa_sys::DAXA_DEVICE_FLAG_BUFFER_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT;
        const CONSERVATIVE_RASTERIZATION = daxa_sys::DAXA_DEVICE_FLAG_CONSERVATIVE_RASTERIZATION;
        const MESH_SHADER_BIT = daxa_sys::DAXA_DEVICE_FLAG_MESH_SHADER_BIT;
        const SHADER_ATOMIC64 = daxa_sys::DAXA_DEVICE_FLAG_SHADER_ATOMIC64;
        const IMAGE_ATOMIC64 = daxa_sys::DAXA_DEVICE_FLAG_IMAGE_ATOMIC64;
        const VK_MEMORY_MODEL = daxa_sys::DAXA_DEVICE_FLAG_VK_MEMORY_MODEL;
    }
}

pub type DeviceSelector = extern "C" fn(*const VkPhysicalDeviceProperties) -> i32;

pub extern "C" fn default_device_selector(properties: *const VkPhysicalDeviceProperties) -> i32 {
    unsafe {
        daxa_sys::daxa_default_device_score(properties)
    }
}

pub const VkMaxPhysicalDeviceNameSize: usize = 256;

pub struct VkPhysicalDeviceProperties {
    api_version: u32,
    driver_version: u32,
    vendor_id: u32,
    device_id: u32,
    device_type: DeviceType,
    
}