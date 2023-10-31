use bitflags::bitflags;
use std::mem;
use std::sync;

use crate::types::*;

#[repr(u32)]
pub enum DeviceType {
    Other = daxa_sys::daxa_DeviceType_DAXA_DEVICE_TYPE_OTHER,
    IntegratedGpu = daxa_sys::daxa_DeviceType_DAXA_DEVICE_TYPE_INTEGRATED_GPU,
    DiscreteGpu = daxa_sys::daxa_DeviceType_DAXA_DEVICE_TYPE_DISCRETE_GPU,
    VirtualGpu = daxa_sys::daxa_DeviceType_DAXA_DEVICE_TYPE_VIRTUAL_GPU,
    Cpu = daxa_sys::daxa_DeviceType_DAXA_DEVICE_TYPE_CPU,
}

bitflags! {
    #[derive(Default)]
    pub struct DeviceFlags: u64 {
        const BUFFER_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT = daxa_sys::daxa_DeviceFlagBits_DAXA_DEVICE_FLAG_BUFFER_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT;
        const CONSERVATIVE_RASTERIZATION = daxa_sys::daxa_DeviceFlagBits_DAXA_DEVICE_FLAG_CONSERVATIVE_RASTERIZATION;
        const MESH_SHADER_BIT = daxa_sys::daxa_DeviceFlagBits_DAXA_DEVICE_FLAG_MESH_SHADER_BIT;
        const SHADER_ATOMIC64 = daxa_sys::daxa_DeviceFlagBits_DAXA_DEVICE_FLAG_SHADER_ATOMIC64;
        const IMAGE_ATOMIC64 = daxa_sys::daxa_DeviceFlagBits_DAXA_DEVICE_FLAG_IMAGE_ATOMIC64;
        const VK_MEMORY_MODEL = daxa_sys::daxa_DeviceFlagBits_DAXA_DEVICE_FLAG_VK_MEMORY_MODEL;
    }
}

pub type DeviceSelector = extern "C" fn(*const VkPhysicalDeviceProperties) -> i32;

pub extern "C" fn default_device_selector(properties: *const VkPhysicalDeviceProperties) -> i32 {
    unsafe { daxa_sys::daxa_default_device_score(properties) }
}

pub struct DeviceInfo<'a> {
    selector: DeviceSelector,
    flags: DeviceFlags,
    max_allowed_images: u32,
    max_allowed_buffers: u32,
    max_allowed_samplers: u32,
    name: String,
}

pub struct Device(daxa_sys::daxa_Device);

macro_rules! device_create_fn {
    (name: ident, from: ty, daxa_from: ty, to: ty) => {
        pub fn create_$name(
            &self,
            info: &$from,
        ) -> std::result::Result<$to, crate::types::Result> {
            unsafe {
                let mut handle = mem::zeroed();
    
                let c_result = daxa_sys::daxa_dvc_create_$name(
                    self.handle,
                    info.as_ptr().cast::<$daxa_from>(),
                    &mut handle,
                );
    
                match mem::transmute::<Result>(c_result) {
                    crate::Result::Success => Ok($to(handle)),
                    error => Err(error),
                }
            }
        }
    };
}

impl Device {
    pub fn buffer_memory_requirements(&self, info: &[BufferInfo]) -> MemoryRequirements {
        unsafe {
            mem::transmute::<MemoryRequirements>(daxa_sys::daxa_dvc_buffer_memory_requirements(
                self.handle,
                info.as_ptr().cast::<daxa_sys::daxa_BufferInfo>(),
            ))
        }
    }

    pub fn image_memory_requirements(&self, info: &[ImageInfo]) -> MemoryRequirements {
        unsafe {
            mem::transmute::<MemoryRequirements>(daxa_sys::daxa_dvc_image_memory_requirements(
                self.handle,
                info.as_ptr().cast::<daxa_sys::daxa_ImageInfo>(),
            ))
        }
    }

    device_create_fn!(memory, MemoryBlockInfo, daxa_MemoryBlock, MemoryBlock);
    device_create_fn!(image, ImageInfo, daxa_ImageInfo, Image);
    device_create_fn!(image_view, ImageViewInfo, daxa_ImageViewInfo, ImageView);
    device_create_fn!(sampler, SamplerInfo, daxa_SamplerInfo, Sampler);
    //TODO: Patrick review these functions and make sure their signatures are correct. They seem to be correct (they all follow the same path/code), so using the same macro should be sound.
    device_create_fn!(raster_pipeline, RasterPipelineInfo, daxa_RasterPipelineInfo, RasterPipeline);
    device_create_fn!(compute_pipeline, ComputePipelineInfo, daxa_ComputePipelineInfo, ComputePipeline);
    device_create_fn!(swapchain_pipeline, SwapchainInfo, daxa_SwapchainInfo, Swapchain);
    device_create_fn!(command_recorder, CommandRecorderInfo, daxa_CommandRecorderInfo, CommandRecorder);
    device_create_fn!(binary_semaphore, BinarySemaphoreInfo, daxa_BinarySemaphoreInfo, BinarySemaphore);
    device_create_fn!(timeline_semaphore, TimelineSemaphoreInfo, daxa_TimelineSemaphoreInfo, TimelineSemaphore);
    device_create_fn!(event, EventInfo, daxa_EventInfo, Event);
    device_create_fn!(timeline_query_pool, TimelineQueryPoolInfo, daxa_TimelineQueryPoolInfo, TimelineQueryPool);

    pub fn is_buffer_valid(&self, buffer: BufferId) -> bool {
        unsafe { daxa_sys::daxa_dvc_is_buffer_valid(self.handle, buffer) }
    }

    pub fn is_image_valid(&self, image: ImageId) -> bool {
        unsafe { daxa_sys::daxa_dvc_is_image_valid(self.handle, image) }
    }

    pub fn is_image_view_valid(&self, image_view: ImageViewId) -> bool {
        unsafe { daxa_sys::daxa_dvc_is_image_view_valid(self.handle, image_view) }
    }

    pub fn is_sampler_valid(&self, sampler: SamplerId) -> bool {
        unsafe { daxa_sys::daxa_dvc_is_sampler_valid(self.handle, sampler) }
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
    //             self.handle,
    //             c_info,
    //             &mut raster_pipeline,
    //         );

    //         match mem::transmute::<Result>(c_result) {
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
    //             self.handle,
    //             c_info,
    //             &mut compute_pipeline,
    //         );

    //         match mem::transmute::<Result>(c_result) {
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

    //         let c_result = daxa_sys::daxa_dvc_create_swapchain(self.handle, c_info, &mut swapchain);

    //         match mem::transmute::<Result>(c_result) {
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
    //             self.handle,
    //             c_info,
    //             &mut command_recorder,
    //         );

    //         match mem::transmute::<Result>(c_result) {
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
    //             self.handle,
    //             c_info,
    //             &mut binary_semaphore,
    //         );

    //         match mem::transmute::<Result>(c_result) {
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
    //             self.handle,
    //             c_info,
    //             &mut timeline_semaphore,
    //         );

    //         match mem::transmute::<Result>(c_result) {
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

    //         let c_result = daxa_sys::daxa_dvc_create_event(self.handle, c_info, &mut event);

    //         match mem::transmute::<Result>(c_result) {
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
    //             self.handle,
    //             c_info,
    //             &mut timeline_query_pool,
    //         );

    //         match mem::transmute::<Result>(c_result) {
    //             Success => Ok(timeline_query_pool),
    //             error => Err(error),
    //         }
    //     }
    // }

    pub fn buffer_device_address(&self, buffer: BufferId) -> BufferDeviceAddress {
        unsafe {
            let mut address = mem::zeroed();
            daxa_sys::daxa_dvc_buffer_device_address(self.handle, buffer, &mut address);
            address
        }
    }

    pub fn buffer_host_address(&self, buffer: BufferId) -> *mut () {
        unsafe {
            let mut address = mem::zeroed();
            daxa_sys::daxa_dvc_host_device_address(self.handle, buffer, &mut address);
            address
        }
    }

    pub fn info(&self) -> &DeviceInfo {
        unsafe { daxa_sys::daxa_dvc_info(self.handle).as_ref().unwrap() }
    }

    //TODO submit
    //TODO present

    pub fn wait_idle(&self) -> std::result::Result<(), crate::types::Result> {
        unsafe {
            match mem::transmute::<Result>(daxa_sys::daxa_dvc_wait_idle(self.handle)) {
                crate::Result::Success => Ok(()),
                error => Err(error),
            }
        }
    }

    pub fn collect_garbage(&self) -> std::result::Result<(), crate::types::Result> {
        unsafe {
            match mem::transmute::<Result>(daxa_sys::daxa_dvc_collect_garbage(self.handle)) {
                crate::Result::Success => Ok(()),
                error => Err(error),
            }
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            daxa_sys::daxa_destroy_device(self.handle);
        }
    }
}
