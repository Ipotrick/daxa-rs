use crate::types::{self, Swapchain};

type NativeWindowHandle = daxa_sys::daxa_NativeWindowHandle;

pub type SurfaceFormatSelector = extern "C" fn(types::Format) -> i32;

pub extern "C" fn default_format_selector(format: types::Format) -> i32 {
    unsafe { daxa_sys::daxa_default_format_selector(format as _) }
}

#[repr(i32)]
pub enum NativeWindowPlatform {
    Unknown = daxa_sys::daxa_NativeWindowPlatform_DAXA_NATIVE_WINDOW_PLATFORM_UNKNOWN,
    Win32 = daxa_sys::daxa_NativeWindowPlatform_DAXA_NATIVE_WINDOW_PLATFORM_WIN32_API,
    Xlib = daxa_sys::daxa_NativeWindowPlatform_DAXA_NATIVE_WINDOW_PLATFORM_XLIB_API,
    Wayland = daxa_sys::daxa_NativeWindowPlatform_DAXA_NATIVE_WINDOW_PLATFORM_WAYLAND_API,
}

#[repr(i32)]
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
    pub struct SurfaceTransformFlags: i32 {
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

// macro_rules! swapchain_fn {
//     ($name:ident, $ret:ty) => {
//         paste::item! {
//             pub fn $name (&self) -> $ret {
//                 unsafe {
//                     std::mem::transmute(daxa_sys::[< daxa_swp_ $name >] (self.0))
//                 }
//             }
//         }
//     };
// }

// impl Swapchain {
//     pub fn set_present_mode(&self, present_mode: PresentMode) -> types::Result {
//         unsafe { std::mem::transmute(daxa_sys::daxa_swp_set_present_mode(self.0, std::mem::transmute(present_mode))) }
//     }

//     pub fn acquire_next_image(&self) -> std::result::Result<types::ImageId, types::Result> {
//         unsafe {
//             let mut out_image_id: types::ImageId;
//             let res = std::mem::transmute(daxa_sys::daxa_swp_acquire_next_image(
//                 self.0,
//                 out_image_id.0,
//             ));
//             if res == types::Result::Success {
//                 Ok(out_image_id)
//             } else {
//                 Err(res)
//             }
//         }
//     }

//     swapchain_fn!(get_surface_extent, types::Extent);
//     swapchain_fn!(get_format, types::Format);
//     swapchain_fn!(resize, types::Result);
//     swapchain_fn!(get_acquire_semaphore, types::BinarySemaphore);
//     swapchain_fn!(get_present_semaphore, types::BinarySemaphore);
//     swapchain_fn!(get_gpu_timeline_semaphore, types::TimelineSemaphore);
//     swapchain_fn!(get_cpu_timeline_value, usize);
//     swapchain_fn!(info, SwapchainInfo);
//     swapchain_fn!(get_vk_swapchain, daxa_sys::VkSwapchainKHR);
//     swapchain_fn!(get_vk_surface, daxa_sys::VkSurfaceKHR);
//     swapchain_fn!(inc_refcnt, u64);
//     swapchain_fn!(dec_refcnt, u64);
// }
