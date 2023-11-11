use bitflags::bitflags;
use std::mem;
use std::sync;

use crate::{pipeline::RasterPipelineInfo, swapchain::SwapchainInfo, command_recorder::*, types::*};

#[repr(i32)]
pub enum DeviceType {
    Other = daxa_sys::daxa_DeviceType_DAXA_DEVICE_TYPE_OTHER,
    IntegratedGpu = daxa_sys::daxa_DeviceType_DAXA_DEVICE_TYPE_INTEGRATED_GPU,
    DiscreteGpu = daxa_sys::daxa_DeviceType_DAXA_DEVICE_TYPE_DISCRETE_GPU,
    VirtualGpu = daxa_sys::daxa_DeviceType_DAXA_DEVICE_TYPE_VIRTUAL_GPU,
    Cpu = daxa_sys::daxa_DeviceType_DAXA_DEVICE_TYPE_CPU,
}

bitflags! {
    #[derive(Default)]
    pub struct DeviceFlags: i32 {
        const BUFFER_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT = daxa_sys::daxa_DeviceFlagBits_DAXA_DEVICE_FLAG_BUFFER_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT;
        const CONSERVATIVE_RASTERIZATION = daxa_sys::daxa_DeviceFlagBits_DAXA_DEVICE_FLAG_CONSERVATIVE_RASTERIZATION;
        const MESH_SHADER_BIT = daxa_sys::daxa_DeviceFlagBits_DAXA_DEVICE_FLAG_MESH_SHADER_BIT;
        const SHADER_ATOMIC64 = daxa_sys::daxa_DeviceFlagBits_DAXA_DEVICE_FLAG_SHADER_ATOMIC64;
        const IMAGE_ATOMIC64 = daxa_sys::daxa_DeviceFlagBits_DAXA_DEVICE_FLAG_IMAGE_ATOMIC64;
        const VK_MEMORY_MODEL = daxa_sys::daxa_DeviceFlagBits_DAXA_DEVICE_FLAG_VK_MEMORY_MODEL;
    }
}

pub type DeviceSelector = unsafe extern "C" fn(*const daxa_sys::daxa_DeviceProperties) -> i32;

pub extern "C" fn default_device_selector(properties: *const VkPhysicalDeviceProperties) -> i32 {
    unsafe { daxa_sys::daxa_default_device_score(properties as *const _) }
}

#[repr(C)]
pub struct DeviceInfo {
    selector: DeviceSelector,
    flags: DeviceFlags,
    max_allowed_images: u32,
    max_allowed_buffers: u32,
    max_allowed_samplers: u32,
    name: SmallString,
}

impl Default for DeviceInfo
{
    // TODO(GABE): this causes a link error...
    // fn default() -> Self {
    //     unsafe{
    //         std::mem::transmute(daxa_sys::DAXA_DEFAULT_DEVICE_INFO)
    //     }
    // }
    fn default() -> Self {
        Self{
            selector: daxa_sys::daxa_default_device_score,
            flags: DeviceFlags::BUFFER_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT,
            max_allowed_images: 10000,
            max_allowed_buffers: 10000,
            max_allowed_samplers: 400,
            name: Default::default(),
        }
    }
}

pub struct Device(daxa_sys::daxa_Device);

macro_rules! device_create_fn {
    ($name:ident, $type:ident) => {
        paste::item! {
            pub fn [< create_ $name >] (&self, info: & [< $type Info >]) -> std::result::Result<$type, crate::types::Result> {
                unsafe {
                    let mut handle = mem::zeroed();

                    let c_result = daxa_sys:: [< daxa_dvc_create_ $name >] (
                        self.0,
                        (info as *const [< $type Info >]).cast::< daxa_sys:: [< daxa_ $type Info >] >(),
                        &mut handle,
                    );

                    match mem::transmute::<daxa_sys::daxa_Result, Result>(c_result) {
                        crate::Result::Success => Ok(std::mem::transmute(handle)),
                        error => Err(error),
                    }
                }
            }
        }
    };
}

impl Device {
    pub fn buffer_memory_requirements(&self, info: &[BufferInfo]) -> MemoryRequirements {
        unsafe {
            mem::transmute::<_, MemoryRequirements>(daxa_sys::daxa_dvc_buffer_memory_requirements(
                self.0,
                info.as_ptr().cast::<daxa_sys::daxa_BufferInfo>(),
            ))
        }
    }

    pub fn image_memory_requirements(&self, info: &[ImageInfo]) -> MemoryRequirements {
        unsafe {
            mem::transmute::<_, MemoryRequirements>(daxa_sys::daxa_dvc_image_memory_requirements(
                self.0,
                info.as_ptr().cast::<daxa_sys::daxa_ImageInfo>(),
            ))
        }
    }

    //TODO: Patrick review these functions and make sure their signatures are correct. They seem to be correct (they all follow the same path/code), so using the same macro should be sound.
    // device_create_fn!(memory, MemoryBlock);
    // device_create_fn!(image, Image);
    // device_create_fn!(image_view, ImageView);
    // device_create_fn!(sampler, Sampler);
    // device_create_fn!(buffer, Buffer);
    // device_create_fn!(raster_pipeline, RasterPipeline);
    // device_create_fn!(compute_pipeline, ComputePipeline);
    // device_create_fn!(swapchain, Swapchain);
    // device_create_fn!(binary_semaphore, BinarySemaphore);
    // device_create_fn!(timeline_semaphore, TimelineSemaphore);
    // device_create_fn!(event, Event);
    // device_create_fn!(timeline_query_pool, TimelineQueryPool);
    device_create_fn!(command_recorder, CommandRecorder);

    pub fn is_buffer_valid(&self, buffer: BufferId) -> bool {
        unsafe { daxa_sys::daxa_dvc_is_buffer_valid(self.0, buffer) != 0 }
    }

    pub fn is_image_valid(&self, image: ImageId) -> bool {
        unsafe { daxa_sys::daxa_dvc_is_image_valid(self.0, image) != 0 }
    }

    pub fn is_image_view_valid(&self, image_view: ImageViewId) -> bool {
        unsafe { daxa_sys::daxa_dvc_is_image_view_valid(self.0, image_view) != 0 }
    }

    pub fn is_sampler_valid(&self, sampler: SamplerId) -> bool {
        unsafe { daxa_sys::daxa_dvc_is_sampler_valid(self.0, sampler) != 0 }
    }

    // pub fn create_raster_pipeline(
    //     &self,
    //     info: &crate::pipeline::RasterPipelineInfo,
    // ) -> std::result::Result<crate::pipeline::RasterPipeline, crate::types::Result> {
    //     use crate::types::Result;
    //     use Result::Success;
    //     unsafe {
    //         let c_info = info.as_ptr().cast::<daxa_sys::daxa_RasterPipelineInfo>();

    //         let mut raster_pipeline = std::mem::zeroed();

    //         let c_result = daxa_sys::daxa_dvc_create_raster_pipeline(
    //             self.0,
    //             c_info,
    //             &mut raster_pipeline,
    //         );

    //         match mem::transmute::<daxa_sys::daxa_Result, Result>(c_result) {
    //             Success => Ok(raster_pipeline),
    //             error => Err(error),
    //         }
    //     }
    // }

    // //compute
    // pub fn create_compute_pipeline(
    //     &self,
    //     info: &crate::pipeline::ComputePipelineInfo,
    // ) -> std::result::Result<crate::pipeline::ComputePipeline, crate::types::Result> {
    //     use crate::types::Result;
    //     use Result::Success;
    //     unsafe {
    //         let c_info = info.as_ptr().cast::<daxa_sys::daxa_ComputePipelineInfo>();

    //         let mut compute_pipeline = std::mem::zeroed();

    //         let c_result = daxa_sys::daxa_dvc_create_compute_pipeline(
    //             self.0,
    //             c_info,
    //             &mut compute_pipeline,
    //         );

    //         match mem::transmute::<daxa_sys::daxa_Result, Result>(c_result) {
    //             Success => Ok(compute_pipeline),
    //             error => Err(error),
    //         }
    //     }
    // }

    // //swapchain
    // pub fn create_swapchain(
    //     &self,
    //     info: &crate::pipeline::SwapchainInfo,
    // ) -> std::result::Result<crate::pipeline::Swapchain, crate::types::Result> {
    //     use crate::types::Result;
    //     use Result::Success;
    //     unsafe {
    //         let c_info = info.as_ptr().cast::<daxa_sys::daxa_SwapchainInfo>();

    //         let mut swapchain = std::mem::zeroed();

    //         let c_result = daxa_sys::daxa_dvc_create_swapchain(self.0, c_info, &mut swapchain);

    //         match mem::transmute::<daxa_sys::daxa_Result, Result>(c_result) {
    //             Success => Ok(swapchain),
    //             error => Err(error),
    //         }
    //     }
    // }

    // //command recorder
    // pub fn create_command_recorder(
    //     &self,
    //     info: &crate::pipeline::CommandRecorderInfo,
    // ) -> std::result::Result<crate::pipeline::CommandRecorder, crate::types::Result> {
    //     use crate::types::Result;
    //     use Result::Success;
    //     unsafe {
    //         let c_info = info.as_ptr().cast::<daxa_sys::daxa_CommandRecorderInfo>();

    //         let mut command_recorder = std::mem::zeroed();

    //         let c_result = daxa_sys::daxa_dvc_create_command_recorder(
    //             self.0,
    //             c_info,
    //             &mut command_recorder,
    //         );

    //         match mem::transmute::<daxa_sys::daxa_Result, Result>(c_result) {
    //             Success => Ok(command_recorder),
    //             error => Err(error),
    //         }
    //     }
    // }

    // //binary semaphore
    // pub fn create_binary_semaphore(
    //     &self,
    //     info: &crate::pipeline::BinarySemaphoreInfo,
    // ) -> std::result::Result<crate::pipeline::BinarySemaphore, crate::types::Result> {
    //     use crate::types::Result;
    //     use Result::Success;
    //     unsafe {
    //         let c_info = info.as_ptr().cast::<daxa_sys::daxa_BinarySemaphoreInfo>();

    //         let mut binary_semaphore = std::mem::zeroed();

    //         let c_result = daxa_sys::daxa_dvc_create_binary_semaphore(
    //             self.0,
    //             c_info,
    //             &mut binary_semaphore,
    //         );

    //         match mem::transmute::<daxa_sys::daxa_Result, Result>(c_result) {
    //             Success => Ok(binary_semaphore),
    //             error => Err(error),
    //         }
    //     }
    // }

    // //timeline semaphore
    // pub fn create_timeline_semaphore(
    //     &self,
    //     info: &crate::pipeline::TimelineSemaphoreInfo,
    // ) -> std::result::Result<crate::pipeline::TimelineSemaphore, crate::types::Result> {
    //     use crate::types::Result;
    //     use Result::Success;
    //     unsafe {
    //         let c_info = info.as_ptr().cast::<daxa_sys::daxa_TimelineSemaphoreInfo>();

    //         let mut timeline_semaphore = std::mem::zeroed();

    //         let c_result = daxa_sys::daxa_dvc_create_timeline_semaphore(
    //             self.0,
    //             c_info,
    //             &mut timeline_semaphore,
    //         );

    //         match mem::transmute::<daxa_sys::daxa_Result, Result>(c_result) {
    //             Success => Ok(timeline_semaphore),
    //             error => Err(error),
    //         }
    //     }
    // }

    // //event
    // pub fn create_event(
    //     &self,
    //     info: &crate::pipeline::EventInfo,
    // ) -> std::result::Result<crate::pipeline::Event, crate::types::Result> {
    //     use crate::types::Result;
    //     use Result::Success;
    //     unsafe {
    //         let c_info = info.as_ptr().cast::<daxa_sys::daxa_EventInfo>();

    //         let mut event = std::mem::zeroed();

    //         let c_result = daxa_sys::daxa_dvc_create_event(self.0, c_info, &mut event);

    //         match mem::transmute::<daxa_sys::daxa_Result, Result>(c_result) {
    //             Success => Ok(event),
    //             error => Err(error),
    //         }
    //     }
    // }

    // //timeline query_pool
    // pub fn create_timeline_query_pool(
    //     &self,
    //     info: &crate::pipeline::TimelineQueryInfoInfo,
    // ) -> std::result::Result<crate::pipeline::TimelineQueryInfo, crate::types::Result> {
    //     use crate::types::Result;
    //     use Result::Success;
    //     unsafe {
    //         let c_info = info.as_ptr().cast::<daxa_sys::daxa_TimelineQueryInfoInfo>();

    //         let mut timeline_query_pool = std::mem::zeroed();

    //         let c_result = daxa_sys::daxa_dvc_create_timeline_query_pool(
    //             self.0,
    //             c_info,
    //             &mut timeline_query_pool,
    //         );

    //         match mem::transmute::<daxa_sys::daxa_Result, Result>(c_result) {
    //             Success => Ok(timeline_query_pool),
    //             error => Err(error),
    //         }
    //     }
    // }

    pub fn buffer_device_address(&self, buffer: BufferId) -> BufferDeviceAddress {
        unsafe {
            let mut address = mem::zeroed();
            daxa_sys::daxa_dvc_buffer_device_address(self.0, buffer, &mut address);
            address
        }
    }

    pub fn buffer_host_address(&self, buffer: BufferId) -> *mut () {
        unsafe {
            let mut address = mem::zeroed();
            daxa_sys::daxa_dvc_buffer_host_address(self.0, buffer, &mut address);
            address as *mut _
        }
    }

    pub fn info(&self) -> &DeviceInfo {
        unsafe { mem::transmute::<_, _>(daxa_sys::daxa_dvc_info(self.0).as_ref().unwrap()) }
    }

    //TODO submit
    //TODO present

    pub fn wait_idle(&self) -> std::result::Result<(), crate::types::Result> {
        unsafe {
            match mem::transmute::<daxa_sys::daxa_Result, Result>(daxa_sys::daxa_dvc_wait_idle(
                self.0,
            )) {
                crate::Result::Success => Ok(()),
                error => Err(error),
            }
        }
    }

    pub fn collect_garbage(&self) -> std::result::Result<(), crate::types::Result> {
        unsafe {
            match mem::transmute::<daxa_sys::daxa_Result, Result>(
                daxa_sys::daxa_dvc_collect_garbage(self.0),
            ) {
                crate::Result::Success => Ok(()),
                error => Err(error),
            }
        }
    }
}

// impl Drop for Device {
//     fn drop(&mut self) {
//         unsafe {
//             daxa_sys::daxa_dvc_destroy_device(self.0);
//         }
//     }
// }
