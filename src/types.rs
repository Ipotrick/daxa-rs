use bitflags::bitflags;
use std::{marker::PhantomData, mem, os};

use crate::device::{Device, DeviceType};

#[repr(i32)]
pub enum Result {
    Success = daxa_sys::daxa_Result_DAXA_RESULT_SUCCESS,
    MissingExtension = daxa_sys::daxa_Result_DAXA_RESULT_MISSING_EXTENSION,
}

#[repr(u32)]
pub enum ImageLayout {
    Undefined = daxa_sys::daxa_ImageLayout_DAXA_IMAGE_LAYOUT_UNDEFINED,
    General = daxa_sys::daxa_ImageLayout_DAXA_IMAGE_LAYOUT_GENERAL,
    TransferSrcOptimal = daxa_sys::daxa_ImageLayout_DAXA_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
    TransferDstOptimal = daxa_sys::daxa_ImageLayout_DAXA_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
    ReadOnlyOptimal = daxa_sys::daxa_ImageLayout_DAXA_IMAGE_LAYOUT_READ_ONLY_OPTIMAL,
    AttachmentOptimal = daxa_sys::daxa_ImageLayout_DAXA_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL,
    PresentSrc = daxa_sys::daxa_ImageLayout_DAXA_IMAGE_LAYOUT_PRESENT_SRC,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec2<T> {
    x: T,
    y: T,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec4<T> {
    x: T,
    y: T,
    z: T,
    w: T,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ImageMipArraySlice {
    base_mip_level: u32,
    level_count: u32,
    base_array_layer: u32,
    layer_count: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ImageArraySlice {
    mip_level: u32,
    base_array_layer: u32,
    layer_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ImageSlice {
    mip_level: u32,
    array_layer: u32,
}

bitflags! {
    #[derive(Default)]
    pub struct PipelineStageFlags: u64 {
        const TOP_OF_PIPE_BIT = daxa_sys::VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT;
        const DRAW_INDIRECT_BIT = daxa_sys::VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT;
        const VERTEX_INPUT_BIT = daxa_sys::VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT;
        const VERTEX_SHADER_BIT = daxa_sys::VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT;
        const TESSELLATION_CONTROL_SHADER_BIT = daxa_sys::VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT;
        const TESSELLATION_EVALUATION_SHADER_BIT = daxa_sys::VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT;
        const GEOMETRY_SHADER_BIT = daxa_sys::VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT;
        const FRAGMENT_SHADER_BIT = daxa_sys::VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT;
        const EARLY_FRAGMENT_TESTS_BIT = daxa_sys::VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT;
        const LATE_FRAGMENT_TESTS_BIT = daxa_sys::VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT;
        const COLOR_ATTACHMENT_OUTPUT_BIT = daxa_sys::VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT;
        const COMPUTE_SHADER_BIT = daxa_sys::VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT;
        const TRANSFER_BIT = daxa_sys::VK_PIPELINE_STAGE_2_TRANSFER_BIT;
        const BOTTOM_OF_PIPE_BIT = daxa_sys::VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT;
        const HOST_BIT = daxa_sys::VK_PIPELINE_STAGE_2_HOST_BIT;
        const ALL_GRAPHICS_BIT = daxa_sys::VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT;
        const ALL_COMMANDS_BIT = daxa_sys::VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT;
        const NONE = daxa_sys::VK_PIPELINE_STAGE_2_NONE;
        const TRANSFORM_FEEDBACK_BIT_EXT = daxa_sys::VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT;
        const CONDITIONAL_RENDERING_BIT_EXT = daxa_sys::VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT;
        const ACCELERATION_STRUCTURE_BUILD_BIT_KHR = daxa_sys::VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR;
        const RAY_TRACING_SHADER_BIT_KHR = daxa_sys::VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR;
        const FRAGMENT_DENSITY_PROCESS_BIT_EXT = daxa_sys::VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = daxa_sys::VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR;
        const COMMAND_PREPROCESS_BIT_NV = daxa_sys::VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV;
        const TASK_SHADER_BIT_EXT = daxa_sys::VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_EXT;
        const MESH_SHADER_BIT_EXT = daxa_sys::VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_EXT;
        const SHADING_RATE_IMAGE_BIT_NV = daxa_sys::VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR;
        const RAY_TRACING_SHADER_BIT_NV = daxa_sys::VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR;
        const ACCELERATION_STRUCTURE_BUILD_BIT_NV = daxa_sys::VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR;
        const TASK_SHADER_BIT_NV = daxa_sys::VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_EXT;
        const MESH_SHADER_BIT_NV = daxa_sys::VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_EXT;
        const NONE_KHR = daxa_sys::VK_PIPELINE_STAGE_2_NONE;
    }
}

bitflags! {
    pub struct ImageViewType: u32 {
        const ONE_DIM = daxa_sys::VkImageViewType_VK_IMAGE_VIEW_TYPE_1D;
        const TWO_DIM = daxa_sys::VkImageViewType_VK_IMAGE_VIEW_TYPE_2D;
        const THREE_DIM = daxa_sys::VkImageViewType_VK_IMAGE_VIEW_TYPE_3D;
        const CUBE = daxa_sys::VkImageViewType_VK_IMAGE_VIEW_TYPE_CUBE;
        const ONE_DIM_ARRAY = daxa_sys::VkImageViewType_VK_IMAGE_VIEW_TYPE_1D_ARRAY;
        const TWO_DIM_ARRAY = daxa_sys::VkImageViewType_VK_IMAGE_VIEW_TYPE_2D_ARRAY;
        const CUBE_ARRAY = daxa_sys::VkImageViewType_VK_IMAGE_VIEW_TYPE_CUBE_ARRAY;
    }
}

bitflags! {
    pub struct Filter: u32 {
        const NEAREST = daxa_sys::VkFilter_VK_FILTER_NEAREST;
        const LINEAR = daxa_sys::VkFilter_VK_FILTER_LINEAR;
        const CUBIC_EXT = daxa_sys::VkFilter_VK_FILTER_CUBIC_EXT;
        const CUBIC_IMG = daxa_sys::VkFilter_VK_FILTER_CUBIC_IMG;
    }
}

bitflags! {
    pub struct ImageCreateFlags: u32 {
        const SPARSE_BINDING = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_SPARSE_BINDING_BIT;
        const SPARSE_RESIDENCY = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT;
        const SPARSE_ALIASED = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_SPARSE_ALIASED_BIT;
        const MUTABLE_FORMAT = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT;
        const CUBE_COMPATIBLE = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT;
        const ALIAS = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_ALIAS_BIT;
        const SPLIT_INSTANCE_BIND_REGIONS = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT;
        const TWO_DIM_ARRAY_COMPATIBLE_BIT = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_SPARSE_BINDING_BIT;
        const BLOCK_TEXEL_VIEW_COMPATIBLE = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT;
        const EXTENDED_USAGE = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_EXTENDED_USAGE_BIT;
        const PROTECTED = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_PROTECTED_BIT;
        const DISJOINT = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_DISJOINT_BIT;
        const CORNER_SAMPLED = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV;
        const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT;
        const SUBSAMPLED = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT;
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT;
        const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT;
        const TWO_DIM_VIEW_COMPATIBLE = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_2D_VIEW_COMPATIBLE_BIT_EXT;
        const FRAGMENT_DENSITY_MAP_OFFSET = daxa_sys::VkImageCreateFlagBits_VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM;
    }
}
bitflags! {
    pub struct ImageUsageFlags: u32 {
        const TRANSFER_SRC = daxa_sys::VkImageUsageFlagBits_VK_IMAGE_USAGE_TRANSFER_SRC_BIT;
        const TRANSFER_DST = daxa_sys::VkImageUsageFlagBits_VK_IMAGE_USAGE_TRANSFER_DST_BIT;
        const SAMPLED = daxa_sys::VkImageUsageFlagBits_VK_IMAGE_USAGE_SAMPLED_BIT;
        const STORAGE = daxa_sys::VkImageUsageFlagBits_VK_IMAGE_USAGE_STORAGE_BIT;
        const COLOR_ATTACHMENT = daxa_sys::VkImageUsageFlagBits_VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;
        const DEPTH_STENCIL_ATTACHMENT = daxa_sys::VkImageUsageFlagBits_VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT;
        const TRANSIENT_ATTACHMENT = daxa_sys::VkImageUsageFlagBits_VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT;
        const VIDEO_DECODE_DST = daxa_sys::VkImageUsageFlagBits_VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR;
        const VIDEO_DECODE_SRC = daxa_sys::VkImageUsageFlagBits_VK_IMAGE_USAGE_VIDEO_DECODE_SRC_BIT_KHR;
        const VIDEO_DECODE_DPB = daxa_sys::VkImageUsageFlagBits_VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR;
        const FRAGMENT_DENSITY_MAP = daxa_sys::VkImageUsageFlagBits_VK_IMAGE_USAGE_SAMPLED_BIT;
        const FRAGMENT_SHADING_RATE_ATTACHMENT = daxa_sys::VkImageUsageFlagBits_VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT;
        const HOST_TRANSFER = daxa_sys::VkImageUsageFlagBits_VK_IMAGE_USAGE_HOST_TRANSFER_BIT_EXT;
        const INVOCATION_MASK = daxa_sys::VkImageUsageFlagBits_VK_IMAGE_USAGE_INVOCATION_MASK_BIT_HUAWEI;
        const SAMPLE_WEIGHT = daxa_sys::VkImageUsageFlagBits_VK_IMAGE_USAGE_SAMPLE_WEIGHT_BIT_QCOM;
        const SAMPLE_BLOCK_MATCH = daxa_sys::VkImageUsageFlagBits_VK_IMAGE_USAGE_SAMPLE_BLOCK_MATCH_BIT_QCOM;
    }
}

#[repr(u32)]
pub enum SamplerAddressMode {
    Repeat = daxa_sys::VkSamplerAddressMode_VK_SAMPLER_ADDRESS_MODE_REPEAT,
    MirroredRepeat = daxa_sys::VkSamplerAddressMode_VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT,
    ClampToEdge = daxa_sys::VkSamplerAddressMode_VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
    ClampToBorder = daxa_sys::VkSamplerAddressMode_VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER,
    MirroredClampToBorder =
        daxa_sys::VkSamplerAddressMode_VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE,
}

#[repr(u32)]
pub enum ReductionMode {
    WeightedAverage = daxa_sys::VkSamplerReductionMode_VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE,
    Min = daxa_sys::VkSamplerReductionMode_VK_SAMPLER_REDUCTION_MODE_MIN,
    Max = daxa_sys::VkSamplerReductionMode_VK_SAMPLER_REDUCTION_MODE_MAX,
}

#[repr(u32)]
pub enum BorderColor {
    FloatTransparentBlack = daxa_sys::VkBorderColor_VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK,
    IntTransparentBlack = daxa_sys::VkBorderColor_VK_BORDER_COLOR_INT_TRANSPARENT_BLACK,
    FloatOpaqueBlack = daxa_sys::VkBorderColor_VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK,
    IntOpaqueBlack = daxa_sys::VkBorderColor_VK_BORDER_COLOR_INT_OPAQUE_BLACK,
    FloatOpaqueWhite = daxa_sys::VkBorderColor_VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE,
    IntOpaqueWhite = daxa_sys::VkBorderColor_VK_BORDER_COLOR_INT_OPAQUE_WHITE,
}

pub enum Extent {
    OneDim(u32),
    TwoDim(u32, u32),
    ThreeDim(u32, u32, u32),
}

pub struct CommandSubmitInfo<'a> {
    pub wait_stages: PipelineStageFlags,
    pub cmd_recorders: &'a [CommandRecorder],
    pub wait_binary_semaphores: &'a [BinarySemaphore],
    pub signal_binary_semaphores: &'a [BinarySemaphore],
    pub wait_timeline_semaphores: &'a [BinarySemaphore],
    pub signal_timeline_semaphores: &'a [BinarySemaphore],
}

pub struct PresentInfo<'a> {
    pub wait_binary_semaphores: &'a [BinarySemaphore],
    pub swapchain: Swapchain,
}

macro_rules! id {
    ($name:ident) => {
        paste::item! {
            pub type [< $name Id >] = daxa_sys:: [< daxa_ $name Id >];
        }
    };
}
id!(Image);
id!(ImageView);
id!(Sampler);
id!(Buffer);

pub type BufferDeviceAddress = daxa_sys::daxa_BufferDeviceAddress;

#[repr(C)]
pub struct BufferInfo {
    pub size: usize,
    pub allocate_info: MemoryFlags,
    pub name: SmallString,
}

#[repr(C)]
pub struct ImageInfo {
    pub flags: ImageCreateFlags,
    pub extent: Extent,
    pub format: Format,
    pub mip_level_count: u32,
    pub array_layer_count: u32,
    pub sample_count: u32,
    pub usage: ImageUsageFlags,
    pub allocate_info: MemoryFlags,
    pub name: SmallString,
}

#[repr(C)]
pub struct ImageViewInfo {
    pub ty: ImageViewType,
    pub format: Format,
    pub image: ImageId,
    pub slice: ImageMipArraySlice,
    pub name: SmallString,
}

#[repr(C)]
pub struct SamplerInfo {
    pub magnification_filter: Filter,
    pub minification_filter: Filter,
    pub mipmap_filter: Filter,
    pub reduction_mode: ReductionMode,
    pub address_mode_u: SamplerAddressMode,
    pub address_mode_v: SamplerAddressMode,
    pub address_mode_w: SamplerAddressMode,
    pub mip_lod_bias: f32,
    pub enable_anisotropy: bool,
    pub max_anisotropy: f32,
    pub compare_op: CompareOp,
    pub min_lod: f32,
    pub max_lod: f32,
    pub border_color: BorderColor,
    pub enable_unnormalized_coordinates: bool,
    pub name: SmallString,
}

macro_rules! handle {
    ($name:ident) => {
        paste::item! {
            #[repr(C)]
            pub struct $name(pub(crate) daxa_sys:: [< daxa_ $name >]);

            impl $name {
                //TODO do we need this?
                pub fn id(&self) -> daxa_sys:: [< daxa_ $name >] {
                    self.0
                }
            }
        }
    };
    ($name:ident, $postfix:ident) => {
        paste::item! {
            #[repr(C)]
            pub struct $name(pub(crate) [< $name $postfix >]);

            impl $name {
                pub fn id(&self) -> [ < $name $postfix >] {
                    self.0
                }
            }
        }
    };
}

handle!(MemoryBlock);
handle!(Image, Id);
handle!(ImageView, Id);
handle!(Buffer, Id);
handle!(Sampler, Id);
handle!(RasterPipeline);
handle!(ComputePipeline);
handle!(Swapchain);
handle!(CommandRecorder);
handle!(BinarySemaphore);
handle!(TimelineSemaphore);
handle!(Event);
handle!(TimelineQueryPool);

#[repr(u32)]
pub enum CompareOp {
    Never = daxa_sys::VkCompareOp_VK_COMPARE_OP_NEVER,
    Less = daxa_sys::VkCompareOp_VK_COMPARE_OP_LESS,
    Equal = daxa_sys::VkCompareOp_VK_COMPARE_OP_EQUAL,
    LessOrEqual = daxa_sys::VkCompareOp_VK_COMPARE_OP_LESS_OR_EQUAL,
    Greater = daxa_sys::VkCompareOp_VK_COMPARE_OP_GREATER,
    NotEqual = daxa_sys::VkCompareOp_VK_COMPARE_OP_NOT_EQUAL,
    GreaterOrEqual = daxa_sys::VkCompareOp_VK_COMPARE_OP_GREATER_OR_EQUAL,
    Always = daxa_sys::VkCompareOp_VK_COMPARE_OP_ALWAYS,
}

pub type DeviceSize = u64;

bitflags! {
    #[derive(Default)]
    pub struct MemoryFlags: u32 {
        const DEDICATED_MEMORY = daxa_sys::DAXA_MEMORY_FLAG_DEDICATED_MEMORY;
        const CAN_ALIAS = daxa_sys::DAXA_MEMORY_FLAG_CAN_ALIAS;
        const SEQUENTIAL_WRITE = daxa_sys::DAXA_MEMORY_FLAG_HOST_ACCESS_SEQUENTIAL_WRITE;
        const HOST_ACCESS_RANDOM = daxa_sys::DAXA_MEMORY_FLAG_HOST_ACCESS_RANDOM;
        const MIN_MEMORY = daxa_sys::DAXA_MEMORY_FLAG_STRATEGY_MIN_MEMORY;
        const MIN_TIME = daxa_sys::DAXA_MEMORY_FLAG_STRATEGY_MIN_TIME;
    }
}

pub struct MemoryRequirements {
    size: DeviceSize,
    alignment: DeviceSize,
    memory_type_bits: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MemoryBlockInfo {
    requirements: daxa_sys::VkMemoryRequirements,
    flags: MemoryFlags,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ManualAllocInfo {
    memory_block: MemoryBlockInfo,
    offset: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union AllocInfo {
    auto_alloc_info: MemoryFlags,
    manual_alloc_info: ManualAllocInfo,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MemoryAllocateInfo {
    index: u64,
    info: AllocInfo,
}

#[repr(C)]
#[derive(Clone)]
pub struct AllocateInfo {
    query_count: u32,
    name: SmallString,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Option<T> {
    data: T,
    has_value: bool,
}

impl<T: 'static> From<std::option::Option<T>> for Option<T> {
    fn from(value: std::option::Option<T>) -> Self {
        let Some(data) = value else {
            return Option {
                data: unsafe { mem::MaybeUninit::uninit().assume_init() },
                has_value: false,
            };
        };

        Option {
            data,
            has_value: true,
        }
    }
}

impl<T: 'static> Into<std::option::Option<T>> for Option<T> {
    fn into(self) -> std::option::Option<T> {
        let Option {
            has_value: true, ..
        } = &self
        else {
            return None;
        };
        Some(self.data)
    }
}

#[derive(Clone, Copy)]
pub struct FixedString<const Capacity: usize> {
    data: [std::os::raw::c_char; Capacity],
    len: u8,
}

impl<const Capacity: usize> FixedString<Capacity> {
    pub unsafe fn from_ptr(ptr: *const os::raw::c_char, len: u8) -> Self {
        let mut data = [Default::default(); Capacity];
        unsafe { std::ptr::copy(ptr, &mut data as *mut _, len as _) };
        Self { data, len }
    }
    pub unsafe fn from_ptr_null_terminated(ptr: *const os::raw::c_char) -> Self {
        let mut len = 0u8;
        while *ptr.add(len as _) != 0 {
            len += 1;
        }
        Self::from_ptr(ptr, len)
    }
}

impl<'a, const Capacity: usize> From<&'a str> for FixedString<Capacity> {
    fn from(string: &'a str) -> Self {
        assert!(string.bytes().len() < Capacity);
        unsafe { Self::from_ptr(string.as_bytes().as_ptr() as *const _, string.bytes().len() as u8) }
    }
}

pub type SmallString = FixedString<39>;

#[repr(u32)]
pub enum Format {
    Undefined = daxa_sys::VkFormat_VK_FORMAT_UNDEFINED,
    R4g4UnormPack8 = daxa_sys::VkFormat_VK_FORMAT_R4G4_UNORM_PACK8,
    R4g4b4a4UnormPack16 = daxa_sys::VkFormat_VK_FORMAT_R4G4B4A4_UNORM_PACK16,
    B4g4r4a4UnormPack16 = daxa_sys::VkFormat_VK_FORMAT_B4G4R4A4_UNORM_PACK16,
    R5g6b5UnormPack16 = daxa_sys::VkFormat_VK_FORMAT_R5G6B5_UNORM_PACK16,
    B5g6r5UnormPack16 = daxa_sys::VkFormat_VK_FORMAT_B5G6R5_UNORM_PACK16,
    R5g5b5a1UnormPack16 = daxa_sys::VkFormat_VK_FORMAT_R5G5B5A1_UNORM_PACK16,
    B5g5r5a1UnormPack16 = daxa_sys::VkFormat_VK_FORMAT_B5G5R5A1_UNORM_PACK16,
    A1r5g5b5UnormPack16 = daxa_sys::VkFormat_VK_FORMAT_A1R5G5B5_UNORM_PACK16,
    R8Unorm = daxa_sys::VkFormat_VK_FORMAT_R8_UNORM,
    R8Snorm = daxa_sys::VkFormat_VK_FORMAT_R8_SNORM,
    R8Uscaled = daxa_sys::VkFormat_VK_FORMAT_R8_USCALED,
    R8Sscaled = daxa_sys::VkFormat_VK_FORMAT_R8_SSCALED,
    R8Uint = daxa_sys::VkFormat_VK_FORMAT_R8_UINT,
    R8Sint = daxa_sys::VkFormat_VK_FORMAT_R8_SINT,
    R8Srgb = daxa_sys::VkFormat_VK_FORMAT_R8_SRGB,
    R8g8Unorm = daxa_sys::VkFormat_VK_FORMAT_R8G8_UNORM,
    R8g8Snorm = daxa_sys::VkFormat_VK_FORMAT_R8G8_SNORM,
    R8g8Uscaled = daxa_sys::VkFormat_VK_FORMAT_R8G8_USCALED,
    R8g8Sscaled = daxa_sys::VkFormat_VK_FORMAT_R8G8_SSCALED,
    R8g8Uint = daxa_sys::VkFormat_VK_FORMAT_R8G8_UINT,
    R8g8Sint = daxa_sys::VkFormat_VK_FORMAT_R8G8_SINT,
    R8g8Srgb = daxa_sys::VkFormat_VK_FORMAT_R8G8_SRGB,
    R8g8b8Unorm = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_UNORM,
    R8g8b8Snorm = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_SNORM,
    R8g8b8Uscaled = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_USCALED,
    R8g8b8Sscaled = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_SSCALED,
    R8g8b8Uint = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_UINT,
    R8g8b8Sint = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_SINT,
    R8g8b8Srgb = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_SRGB,
    B8g8r8Unorm = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_UNORM,
    B8g8r8Snorm = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_SNORM,
    B8g8r8Uscaled = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_USCALED,
    B8g8r8Sscaled = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_SSCALED,
    B8g8r8Uint = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_UINT,
    B8g8r8Sint = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_SINT,
    B8g8r8Srgb = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_SRGB,
    R8g8b8a8Unorm = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_UNORM,
    R8g8b8a8Snorm = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_SNORM,
    R8g8b8a8Uscaled = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_USCALED,
    R8g8b8a8Sscaled = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_SSCALED,
    R8g8b8a8Uint = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_UINT,
    R8g8b8a8Sint = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_SINT,
    R8g8b8a8Srgb = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_SRGB,
    B8g8r8a8Unorm = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_UNORM,
    B8g8r8a8Snorm = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_SNORM,
    B8g8r8a8Uscaled = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_USCALED,
    B8g8r8a8Sscaled = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_SSCALED,
    B8g8r8a8Uint = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_UINT,
    B8g8r8a8Sint = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_SINT,
    B8g8r8a8Srgb = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_SRGB,
    A8b8g8r8UnormPack32 = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_UNORM_PACK32,
    A8b8g8r8SnormPack32 = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_SNORM_PACK32,
    A8b8g8r8UscaledPack32 = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_USCALED_PACK32,
    A8b8g8r8SscaledPack32 = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_SSCALED_PACK32,
    A8b8g8r8UintPack32 = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_UINT_PACK32,
    A8b8g8r8SintPack32 = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_SINT_PACK32,
    A8b8g8r8SrgbPack32 = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_SRGB_PACK32,
    A2r10g10b10UnormPack32 = daxa_sys::VkFormat_VK_FORMAT_A2R10G10B10_UNORM_PACK32,
    A2r10g10b10SnormPack32 = daxa_sys::VkFormat_VK_FORMAT_A2R10G10B10_SNORM_PACK32,
    A2r10g10b10UscaledPack32 = daxa_sys::VkFormat_VK_FORMAT_A2R10G10B10_USCALED_PACK32,
    A2r10g10b10SscaledPack32 = daxa_sys::VkFormat_VK_FORMAT_A2R10G10B10_SSCALED_PACK32,
    A2r10g10b10UintPack32 = daxa_sys::VkFormat_VK_FORMAT_A2R10G10B10_UINT_PACK32,
    A2r10g10b10SintPack32 = daxa_sys::VkFormat_VK_FORMAT_A2R10G10B10_SINT_PACK32,
    A2b10g10r10UnormPack32 = daxa_sys::VkFormat_VK_FORMAT_A2B10G10R10_UNORM_PACK32,
    A2b10g10r10SnormPack32 = daxa_sys::VkFormat_VK_FORMAT_A2B10G10R10_SNORM_PACK32,
    A2b10g10r10UscaledPack32 = daxa_sys::VkFormat_VK_FORMAT_A2B10G10R10_USCALED_PACK32,
    A2b10g10r10SscaledPack32 = daxa_sys::VkFormat_VK_FORMAT_A2B10G10R10_SSCALED_PACK32,
    A2b10g10r10UintPack32 = daxa_sys::VkFormat_VK_FORMAT_A2B10G10R10_UINT_PACK32,
    A2b10g10r10SintPack32 = daxa_sys::VkFormat_VK_FORMAT_A2B10G10R10_SINT_PACK32,
    R16Unorm = daxa_sys::VkFormat_VK_FORMAT_R16_UNORM,
    R16Snorm = daxa_sys::VkFormat_VK_FORMAT_R16_SNORM,
    R16Uscaled = daxa_sys::VkFormat_VK_FORMAT_R16_USCALED,
    R16Sscaled = daxa_sys::VkFormat_VK_FORMAT_R16_SSCALED,
    R16Uint = daxa_sys::VkFormat_VK_FORMAT_R16_UINT,
    R16Sint = daxa_sys::VkFormat_VK_FORMAT_R16_SINT,
    R16Sfloat = daxa_sys::VkFormat_VK_FORMAT_R16_SFLOAT,
    R16g16Unorm = daxa_sys::VkFormat_VK_FORMAT_R16G16_UNORM,
    R16g16Snorm = daxa_sys::VkFormat_VK_FORMAT_R16G16_SNORM,
    R16g16Uscaled = daxa_sys::VkFormat_VK_FORMAT_R16G16_USCALED,
    R16g16Sscaled = daxa_sys::VkFormat_VK_FORMAT_R16G16_SSCALED,
    R16g16Uint = daxa_sys::VkFormat_VK_FORMAT_R16G16_UINT,
    R16g16Sint = daxa_sys::VkFormat_VK_FORMAT_R16G16_SINT,
    R16g16Sfloat = daxa_sys::VkFormat_VK_FORMAT_R16G16_SFLOAT,
    R16g16b16Unorm = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_UNORM,
    R16g16b16Snorm = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_SNORM,
    R16g16b16Uscaled = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_USCALED,
    R16g16b16Sscaled = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_SSCALED,
    R16g16b16Uint = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_UINT,
    R16g16b16Sint = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_SINT,
    R16g16b16Sfloat = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_SFLOAT,
    R16g16b16a16Unorm = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_UNORM,
    R16g16b16a16Snorm = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_SNORM,
    R16g16b16a16Uscaled = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_USCALED,
    R16g16b16a16Sscaled = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_SSCALED,
    R16g16b16a16Uint = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_UINT,
    R16g16b16a16Sint = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_SINT,
    R16g16b16a16Sfloat = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_SFLOAT,
    R32Uint = daxa_sys::VkFormat_VK_FORMAT_R32_UINT,
    R32Sint = daxa_sys::VkFormat_VK_FORMAT_R32_SINT,
    R32Sfloat = daxa_sys::VkFormat_VK_FORMAT_R32_SFLOAT,
    R32g32Uint = daxa_sys::VkFormat_VK_FORMAT_R32G32_UINT,
    R32g32Sint = daxa_sys::VkFormat_VK_FORMAT_R32G32_SINT,
    R32g32Sfloat = daxa_sys::VkFormat_VK_FORMAT_R32G32_SFLOAT,
    R32g32b32Uint = daxa_sys::VkFormat_VK_FORMAT_R32G32B32_UINT,
    R32g32b32Sint = daxa_sys::VkFormat_VK_FORMAT_R32G32B32_SINT,
    R32g32b32Sfloat = daxa_sys::VkFormat_VK_FORMAT_R32G32B32_SFLOAT,
    R32g32b32a32Uint = daxa_sys::VkFormat_VK_FORMAT_R32G32B32A32_UINT,
    R32g32b32a32Sint = daxa_sys::VkFormat_VK_FORMAT_R32G32B32A32_SINT,
    R32g32b32a32Sfloat = daxa_sys::VkFormat_VK_FORMAT_R32G32B32A32_SFLOAT,
    R64Uint = daxa_sys::VkFormat_VK_FORMAT_R64_UINT,
    R64Sint = daxa_sys::VkFormat_VK_FORMAT_R64_SINT,
    R64Sfloat = daxa_sys::VkFormat_VK_FORMAT_R64_SFLOAT,
    R64g64Uint = daxa_sys::VkFormat_VK_FORMAT_R64G64_UINT,
    R64g64Sint = daxa_sys::VkFormat_VK_FORMAT_R64G64_SINT,
    R64g64Sfloat = daxa_sys::VkFormat_VK_FORMAT_R64G64_SFLOAT,
    R64g64b64Uint = daxa_sys::VkFormat_VK_FORMAT_R64G64B64_UINT,
    R64g64b64Sint = daxa_sys::VkFormat_VK_FORMAT_R64G64B64_SINT,
    R64g64b64Sfloat = daxa_sys::VkFormat_VK_FORMAT_R64G64B64_SFLOAT,
    R64g64b64a64Uint = daxa_sys::VkFormat_VK_FORMAT_R64G64B64A64_UINT,
    R64g64b64a64Sint = daxa_sys::VkFormat_VK_FORMAT_R64G64B64A64_SINT,
    R64g64b64a64Sfloat = daxa_sys::VkFormat_VK_FORMAT_R64G64B64A64_SFLOAT,
    B10g11r11UfloatPack32 = daxa_sys::VkFormat_VK_FORMAT_B10G11R11_UFLOAT_PACK32,
    E5b9g9r9UfloatPack32 = daxa_sys::VkFormat_VK_FORMAT_E5B9G9R9_UFLOAT_PACK32,
    D16Unorm = daxa_sys::VkFormat_VK_FORMAT_D16_UNORM,
    X8D24UnormPack32 = daxa_sys::VkFormat_VK_FORMAT_X8_D24_UNORM_PACK32,
    D32Sfloat = daxa_sys::VkFormat_VK_FORMAT_D32_SFLOAT,
    S8Uint = daxa_sys::VkFormat_VK_FORMAT_S8_UINT,
    D16UnormS8Uint = daxa_sys::VkFormat_VK_FORMAT_D16_UNORM_S8_UINT,
    D24UnormS8Uint = daxa_sys::VkFormat_VK_FORMAT_D24_UNORM_S8_UINT,
    D32SfloatS8Uint = daxa_sys::VkFormat_VK_FORMAT_D32_SFLOAT_S8_UINT,
    Bc1RgbUnormBlock = daxa_sys::VkFormat_VK_FORMAT_BC1_RGB_UNORM_BLOCK,
    Bc1RgbSrgbBlock = daxa_sys::VkFormat_VK_FORMAT_BC1_RGB_SRGB_BLOCK,
    Bc1RgbaUnormBlock = daxa_sys::VkFormat_VK_FORMAT_BC1_RGBA_UNORM_BLOCK,
    Bc1RgbaSrgbBlock = daxa_sys::VkFormat_VK_FORMAT_BC1_RGBA_SRGB_BLOCK,
    Bc2UnormBlock = daxa_sys::VkFormat_VK_FORMAT_BC2_UNORM_BLOCK,
    Bc2SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_BC2_SRGB_BLOCK,
    Bc3UnormBlock = daxa_sys::VkFormat_VK_FORMAT_BC3_UNORM_BLOCK,
    Bc3SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_BC3_SRGB_BLOCK,
    Bc4UnormBlock = daxa_sys::VkFormat_VK_FORMAT_BC4_UNORM_BLOCK,
    Bc4SnormBlock = daxa_sys::VkFormat_VK_FORMAT_BC4_SNORM_BLOCK,
    Bc5UnormBlock = daxa_sys::VkFormat_VK_FORMAT_BC5_UNORM_BLOCK,
    Bc5SnormBlock = daxa_sys::VkFormat_VK_FORMAT_BC5_SNORM_BLOCK,
    Bc6hUfloatBlock = daxa_sys::VkFormat_VK_FORMAT_BC6H_UFLOAT_BLOCK,
    Bc6hSfloatBlock = daxa_sys::VkFormat_VK_FORMAT_BC6H_SFLOAT_BLOCK,
    Bc7UnormBlock = daxa_sys::VkFormat_VK_FORMAT_BC7_UNORM_BLOCK,
    Bc7SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_BC7_SRGB_BLOCK,
    Etc2R8g8b8UnormBlock = daxa_sys::VkFormat_VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK,
    Etc2R8g8b8SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK,
    Etc2R8g8b8a1UnormBlock = daxa_sys::VkFormat_VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK,
    Etc2R8g8b8a1SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK,
    Etc2R8g8b8a8UnormBlock = daxa_sys::VkFormat_VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK,
    Etc2R8g8b8a8SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK,
    EacR11UnormBlock = daxa_sys::VkFormat_VK_FORMAT_EAC_R11_UNORM_BLOCK,
    EacR11SnormBlock = daxa_sys::VkFormat_VK_FORMAT_EAC_R11_SNORM_BLOCK,
    EacR11g11UnormBlock = daxa_sys::VkFormat_VK_FORMAT_EAC_R11G11_UNORM_BLOCK,
    EacR11g11SnormBlock = daxa_sys::VkFormat_VK_FORMAT_EAC_R11G11_SNORM_BLOCK,
    Astc4x4UnormBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_4x4_UNORM_BLOCK,
    Astc4x4SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_4x4_SRGB_BLOCK,
    Astc5x4UnormBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x4_UNORM_BLOCK,
    Astc5x4SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x4_SRGB_BLOCK,
    Astc5x5UnormBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x5_UNORM_BLOCK,
    Astc5x5SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x5_SRGB_BLOCK,
    Astc6x5UnormBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x5_UNORM_BLOCK,
    Astc6x5SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x5_SRGB_BLOCK,
    Astc6x6UnormBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x6_UNORM_BLOCK,
    Astc6x6SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x6_SRGB_BLOCK,
    Astc8x5UnormBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x5_UNORM_BLOCK,
    Astc8x5SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x5_SRGB_BLOCK,
    Astc8x6UnormBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x6_UNORM_BLOCK,
    Astc8x6SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x6_SRGB_BLOCK,
    Astc8x8UnormBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x8_UNORM_BLOCK,
    Astc8x8SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x8_SRGB_BLOCK,
    Astc10x5UnormBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x5_UNORM_BLOCK,
    Astc10x5SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x5_SRGB_BLOCK,
    Astc10x6UnormBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x6_UNORM_BLOCK,
    Astc10x6SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x6_SRGB_BLOCK,
    Astc10x8UnormBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x8_UNORM_BLOCK,
    Astc10x8SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x8_SRGB_BLOCK,
    Astc10x10UnormBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x10_UNORM_BLOCK,
    Astc10x10SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x10_SRGB_BLOCK,
    Astc12x10UnormBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x10_UNORM_BLOCK,
    Astc12x10SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x10_SRGB_BLOCK,
    Astc12x12UnormBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x12_UNORM_BLOCK,
    Astc12x12SrgbBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x12_SRGB_BLOCK,
    G8b8g8r8422Unorm = daxa_sys::VkFormat_VK_FORMAT_G8B8G8R8_422_UNORM,
    B8g8r8g8422Unorm = daxa_sys::VkFormat_VK_FORMAT_B8G8R8G8_422_UNORM,
    G8B8R83plane420Unorm = daxa_sys::VkFormat_VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM,
    G8B8r82plane420Unorm = daxa_sys::VkFormat_VK_FORMAT_G8_B8R8_2PLANE_420_UNORM,
    G8B8R83plane422Unorm = daxa_sys::VkFormat_VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM,
    G8B8r82plane422Unorm = daxa_sys::VkFormat_VK_FORMAT_G8_B8R8_2PLANE_422_UNORM,
    G8B8R83plane444Unorm = daxa_sys::VkFormat_VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM,
    R10x6UnormPack16 = daxa_sys::VkFormat_VK_FORMAT_R10X6_UNORM_PACK16,
    R10x6g10x6Unorm2pack16 = daxa_sys::VkFormat_VK_FORMAT_R10X6G10X6_UNORM_2PACK16,
    R10x6g10x6b10x6a10x6Unorm4pack16 =
        daxa_sys::VkFormat_VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16,
    G10x6b10x6g10x6r10x6422Unorm4pack16 =
        daxa_sys::VkFormat_VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16,
    B10x6g10x6r10x6g10x6422Unorm4pack16 =
        daxa_sys::VkFormat_VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16,
    G10x6B10x6R10x63plane420Unorm3pack16 =
        daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16,
    G10x6B10x6r10x62plane420Unorm3pack16 =
        daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16,
    G10x6B10x6R10x63plane422Unorm3pack16 =
        daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16,
    G10x6B10x6r10x62plane422Unorm3pack16 =
        daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16,
    G10x6B10x6R10x63plane444Unorm3pack16 =
        daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16,
    R12x4UnormPack16 = daxa_sys::VkFormat_VK_FORMAT_R12X4_UNORM_PACK16,
    R12x4g12x4Unorm2pack16 = daxa_sys::VkFormat_VK_FORMAT_R12X4G12X4_UNORM_2PACK16,
    R12x4g12x4b12x4a12x4Unorm4pack16 =
        daxa_sys::VkFormat_VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16,
    G12x4b12x4g12x4r12x4422Unorm4pack16 =
        daxa_sys::VkFormat_VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16,
    B12x4g12x4r12x4g12x4422Unorm4pack16 =
        daxa_sys::VkFormat_VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16,
    G12x4B12x4R12x43plane420Unorm3pack16 =
        daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16,
    G12x4B12x4r12x42plane420Unorm3pack16 =
        daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16,
    G12x4B12x4R12x43plane422Unorm3pack16 =
        daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16,
    G12x4B12x4r12x42plane422Unorm3pack16 =
        daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16,
    G12x4B12x4R12x43plane444Unorm3pack16 =
        daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16,
    G16b16g16r16422Unorm = daxa_sys::VkFormat_VK_FORMAT_G16B16G16R16_422_UNORM,
    B16g16r16g16422Unorm = daxa_sys::VkFormat_VK_FORMAT_B16G16R16G16_422_UNORM,
    G16B16R163plane420Unorm = daxa_sys::VkFormat_VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM,
    G16B16r162plane420Unorm = daxa_sys::VkFormat_VK_FORMAT_G16_B16R16_2PLANE_420_UNORM,
    G16B16R163plane422Unorm = daxa_sys::VkFormat_VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM,
    G16B16r162plane422Unorm = daxa_sys::VkFormat_VK_FORMAT_G16_B16R16_2PLANE_422_UNORM,
    G16B16R163plane444Unorm = daxa_sys::VkFormat_VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM,
    G8B8r82plane444Unorm = daxa_sys::VkFormat_VK_FORMAT_G8_B8R8_2PLANE_444_UNORM,
    G10x6B10x6r10x62plane444Unorm3pack16 =
        daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16,
    G12x4B12x4r12x42plane444Unorm3pack16 =
        daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16,
    G16B16r162plane444Unorm = daxa_sys::VkFormat_VK_FORMAT_G16_B16R16_2PLANE_444_UNORM,
    A4r4g4b4UnormPack16 = daxa_sys::VkFormat_VK_FORMAT_A4R4G4B4_UNORM_PACK16,
    A4b4g4r4UnormPack16 = daxa_sys::VkFormat_VK_FORMAT_A4B4G4R4_UNORM_PACK16,
    Astc4x4SfloatBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK,
    Astc5x4SfloatBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK,
    Astc5x5SfloatBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK,
    Astc6x5SfloatBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK,
    Astc6x6SfloatBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK,
    Astc8x5SfloatBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK,
    Astc8x6SfloatBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK,
    Astc8x8SfloatBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK,
    Astc10x5SfloatBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK,
    Astc10x6SfloatBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK,
    Astc10x8SfloatBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK,
    Astc10x10SfloatBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK,
    Astc12x10SfloatBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK,
    Astc12x12SfloatBlock = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK,
    Pvrtc12bppUnormBlockImg = daxa_sys::VkFormat_VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG,
    Pvrtc14bppUnormBlockImg = daxa_sys::VkFormat_VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG,
    Pvrtc22bppUnormBlockImg = daxa_sys::VkFormat_VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG,
    Pvrtc24bppUnormBlockImg = daxa_sys::VkFormat_VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG,
    Pvrtc12bppSrgbBlockImg = daxa_sys::VkFormat_VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG,
    Pvrtc14bppSrgbBlockImg = daxa_sys::VkFormat_VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG,
    Pvrtc22bppSrgbBlockImg = daxa_sys::VkFormat_VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG,
    Pvrtc24bppSrgbBlockImg = daxa_sys::VkFormat_VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG,
    R16g16S105Nv = daxa_sys::VkFormat_VK_FORMAT_R16G16_S10_5_NV,
    A1b5g5r5UnormPack16Khr = daxa_sys::VkFormat_VK_FORMAT_A1B5G5R5_UNORM_PACK16_KHR,
    A8UnormKhr = daxa_sys::VkFormat_VK_FORMAT_A8_UNORM_KHR,
}

pub const VK_UUID_SIZE: usize = 16;

pub type VkPhysicalDeviceLimits = daxa_sys::VkPhysicalDeviceLimits;
pub type VkPhysicalDeviceSparseProperties = daxa_sys::VkPhysicalDeviceSparseProperties;

pub struct VkPhysicalDeviceProperties {
    api_version: u32,
    driver_version: u32,
    vendor_id: u32,
    device_id: u32,
    device_type: DeviceType,
    device_name: SmallString,
    pipeline_cache_uuid: [u8; VK_UUID_SIZE],
    limits: VkPhysicalDeviceLimits,
    sparse_properties: VkPhysicalDeviceSparseProperties,
}
