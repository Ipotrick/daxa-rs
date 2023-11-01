use crate::types;

type NativeWindowHandle = daxa_sys::daxa_NativeWindowHandle;

pub type SurfaceFormatSelector = extern "C" fn(types::Format) -> i32;

pub extern "C" fn default_format_selector(format: types::Format) -> i32 {
    unsafe { daxa_sys::daxa_default_format_selector(format as _) }
}

#[repr(u32)]
pub enum NativeWindowPlatform {
    Unknown = daxa_sys::daxa_NativeWindowPlatform_DAXA_NATIVE_WINDOW_PLATFORM_UNKNOWN,
    Win32 = daxa_sys::daxa_NativeWindowPlatform_DAXA_NATIVE_WINDOW_PLATFORM_WIN32_API,
    Xlib = daxa_sys::daxa_NativeWindowPlatform_DAXA_NATIVE_WINDOW_PLATFORM_XLIB_API,
    Wayland = daxa_sys::daxa_NativeWindowPlatform_DAXA_NATIVE_WINDOW_PLATFORM_WAYLAND_API,
}

#[repr(u32)]
pub enum PresentMode {
    Immediate = daxa_sys::VkPresentModeKHR_VK_PRESENT_MODE_IMMEDIATE_KHR,
    Mailbox = daxa_sys::VkPresentModeKHR_VK_PRESENT_MODE_MAILBOX_KHR,
    Fifo = daxa_sys::VkPresentModeKHR_VK_PRESENT_MODE_FIFO_KHR,
    FifoRelaxed = daxa_sys::VkPresentModeKHR_VK_PRESENT_MODE_FIFO_RELAXED_KHR,
    SharedDemandRefresh = daxa_sys::VkPresentModeKHR_VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR,
    SharedContinuousRefresh =
        daxa_sys::VkPresentModeKHR_VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR,
}

bitflags::bitflags! {
    #[derive(Default)]
    pub struct SurfaceTransformFlags: u32 {
        const IDENTITY = daxa_sys::VkSurfaceTransformFlagBitsKHR_VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR;
        const ROTATE_90 = daxa_sys::VkSurfaceTransformFlagBitsKHR_VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR;
        const ROTATE_180 = daxa_sys::VkSurfaceTransformFlagBitsKHR_VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR;
        const ROTATE_270 = daxa_sys::VkSurfaceTransformFlagBitsKHR_VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR;
        const HORIZONTAL_MIRROR = daxa_sys::VkSurfaceTransformFlagBitsKHR_VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR;
        const HORIZONTAL_MIRROR_ROTATE_90 = daxa_sys::VkSurfaceTransformFlagBitsKHR_VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR;
        const HORIZONTAL_MIRROR_ROTATE_180 = daxa_sys::VkSurfaceTransformFlagBitsKHR_VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR;
        const HORIZONTAL_MIRROR_ROTATE_270 = daxa_sys::VkSurfaceTransformFlagBitsKHR_VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR;
        const INHERIT = daxa_sys::VkSurfaceTransformFlagBitsKHR_VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR;
    }
}

#[repr(C)]
pub struct SwapchainInfo {
    native_window: NativeWindowHandle,
    native_window_platform: NativeWindowPlatform,
    surface_format_selector: SurfaceFormatSelector,
    present_mode: PresentMode,
    present_operation: SurfaceTransformFlags,
    image_usage: types::ImageUsageFlags,
    max_allowed_frames_in_flight: usize,
    name: types::SmallString,
}
