use bitflags::bitflags;
use std::mem;
use std::process::Command;
use std::sync;

use crate::types::*;

pub struct CommandRecorderInfo
{
    name: SmallString,
}

impl Default for CommandRecorderInfo
{
    fn default() -> Self {
        unsafe{
            Self{
                name: Default::default(),
            }
        }
    }
}


pub struct CommandRecorder
{
    recorder: daxa_sys::daxa_CommandRecorder,
}

impl Drop for CommandRecorder
{
    fn drop(&mut self) {
        unsafe{
            daxa_sys::daxa_destroy_command_recorder(self.recorder);
        }
    }
}

macro_rules! define_command_recorder_function_with_result {
    ($rust_name:ident, $c_name:ident, $info_type:ty) => {
        impl CommandRecorder
        {
            pub fn $rust_name(self: &Self, info: &$info_type) -> daxa_sys::daxa_Result
            {
                unsafe{
                    daxa_sys::$c_name(std::mem::transmute(self.recorder), std::mem::transmute(info))
                }
            }
        }
    };
}

macro_rules! define_command_recorder_function {
    ($rust_name:ident, $c_name:ident, $info_type:ty) => {
        impl CommandRecorder
        {
            pub fn $rust_name(self: &Self, info: &$info_type)
            {
                unsafe{
                    daxa_sys::$c_name(std::mem::transmute(self.recorder), std::mem::transmute(info))
                }
            }
        }
    };
}

pub type BufferCopyInfo = daxa_sys::daxa_BufferCopyInfo;
pub type BufferImageCopyInfo = daxa_sys::daxa_BufferImageCopyInfo;
pub type ImageBufferCopyInfo = daxa_sys::daxa_ImageBufferCopyInfo;
pub type ImageCopyInfo = daxa_sys::daxa_ImageCopyInfo;
pub type ImageBlitInfo = daxa_sys::daxa_ImageBlitInfo;
pub type ImageClearInfo = daxa_sys::daxa_ImageClearInfo;    
pub type BufferClearInfo = daxa_sys::daxa_BufferClearInfo;
define_command_recorder_function_with_result!(copy_buffer_to_buffer, daxa_cmd_copy_buffer_to_buffer, BufferInfo);
define_command_recorder_function_with_result!(copy_buffer_to_image, daxa_cmd_copy_buffer_to_image, BufferImageCopyInfo);
define_command_recorder_function_with_result!(copy_image_to_buffer, daxa_cmd_copy_image_to_buffer, ImageBufferCopyInfo);
define_command_recorder_function_with_result!(copy_image_to_image, daxa_cmd_copy_image_to_image, ImageCopyInfo);
define_command_recorder_function_with_result!(blit_image_to_image, daxa_cmd_blit_image_to_image, ImageBlitInfo);
define_command_recorder_function_with_result!(clear_buffer, daxa_cmd_clear_buffer, ImageClearInfo);
define_command_recorder_function_with_result!(clear_image, daxa_cmd_clear_image, BufferClearInfo);


pub type MemoryBarrierInfo = daxa_sys::daxa_MemoryBarrierInfo;
pub type ImageMemoryBarrierInfo = daxa_sys::daxa_ImageMemoryBarrierInfo;
pub type EventSignalInfo = daxa_sys::daxa_EventSignalInfo;
pub type EventWaitInfo = daxa_sys::daxa_EventWaitInfo;
pub type ResetEventInfo = daxa_sys::daxa_ResetEventInfo;
define_command_recorder_function!(pipeline_barrier, daxa_cmd_pipeline_barrier, MemoryBarrierInfo);
define_command_recorder_function_with_result!(pipeline_barrier_image_transition, daxa_cmd_pipeline_barrier_image_transition, ImageMemoryBarrierInfo);
define_command_recorder_function!(signal_event, daxa_cmd_signal_event, EventSignalInfo);
define_command_recorder_function!(wait_event, daxa_cmd_wait_event, EventWaitInfo);
define_command_recorder_function!(reset_event, daxa_cmd_reset_event, ResetEventInfo);
impl CommandRecorder
{
    pub fn wait_events(self: &Self, info: &[EventWaitInfo])
    {
        let ptr_range = info.as_ptr_range();
        let size = (ptr_range.end as usize - ptr_range.start as usize) / std::mem::size_of::<EventWaitInfo>();
        unsafe{
            daxa_sys::daxa_cmd_wait_events(
                std::mem::transmute(self.recorder), 
                std::mem::transmute(ptr_range.start), 
                size);
        }
    }
}

pub type SetUniformBufferInfo = daxa_sys::daxa_SetUniformBufferInfo;
impl CommandRecorder
{
    pub fn push_constants<T>(self: &Self, data: &T)
    {
        unsafe{
            let ptr = std::mem::transmute(data);
            let size = std::mem::size_of::<T>();
            daxa_sys::daxa_cmd_push_constant(
                std::mem::transmute(self.recorder), 
                ptr, 
                size as u32);
        }
    }
}
define_command_recorder_function_with_result!(set_uniform_buffer, daxa_cmd_set_uniform_buffer, SetUniformBufferInfo);

pub type DispatchInfo = daxa_sys::daxa_DispatchInfo;
pub type DispatchIndirectInfo = daxa_sys::daxa_DispatchIndirectInfo;
define_command_recorder_function!(dispatch, daxa_cmd_dispatch, DispatchInfo);
define_command_recorder_function_with_result!(dispatch_indirect, daxa_cmd_dispatch_indirect, DispatchIndirectInfo);
define_command_recorder_function!(set_compute_pipeline, daxa_cmd_set_compute_pipeline, ComputePipeline);
