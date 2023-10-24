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
    pub struct PipelineStageFlags: i32 {
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
    pub struct ImageViewType: i32 {
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
    pub struct Filter: i32 {
        const NEAREST = daxa_sys::VkFilter_VK_FILTER_NEAREST;
        const LINEAR = daxa_sys::VkFilter_VK_FILTER_LINEAR;
        const CUBIC_EXT = daxa_sys::VkFilter_VK_FILTER_CUBIC_EXT;
        const CUBIC_IMG = daxa_sys::VkFilter_VK_FILTER_CUBIC_IMG;
    }
}

bitflags! {
    pub struct ImageCreateFlags: i32 {
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
    pub struct ImageUsageFlags: i32 {
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
enum ReductionMode {
    WeightedAverage = daxa_sys::VkSamplerReductionMode_VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE,
    Min = daxa_sys::VkSamplerReductionMode_VK_SAMPLER_REDUCTION_MODE_MIN,
    Max = daxa_sys::VkSamplerReductionMode_VK_SAMPLER_REDUCTION_MODE_MAX,
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

pub type BufferId = daxa_sys::daxa_BufferId;
pub type ImageId = daxa_sys::daxa_ImageId;
pub type ImageViewId = daxa_sys::daxa_ImageViewId;
pub type SamplerId = daxa_sys::daxa_SamplerId;

pub type BufferDeviceAddress = u64;

pub struct BufferInfo {
    pub size: usize,
    pub allocate_info: MemoryFlags,
    pub name: SmallString,
}

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

pub struct ImageViewInfo {
    pub ty: ImageViewType,
    pub format: Format,
    pub image: ImageId,
    pub slice: ImageMipArraySlice,
    pub name: SmallString,
}

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

#[derive(Clone)]
pub struct Buffer {
    pub(crate) device: Device,
    pub(crate) handle: BufferId,
}

impl Buffer {
    pub fn id(&self) -> BufferId {
        unsafe { self.handle }
    }
}

#[derive(Clone)]
struct Image {
    pub(crate) device: Device,
    pub(crate) handle: ImageId,
}

impl Image {
    pub fn id(&self) -> ImageId {
        self.handle
    }
}

#[derive(Clone)]
struct ImageView {
    device: Device,
    handle: ImageViewId,
}

impl ImageView {
    fn id(&self) -> ImageViewId {
        self.handle
    }
}

#[derive(Clone)]
pub struct Sampler {
    pub(crate) device: Device,
    pub(crate) handle: SamplerId,
}

impl Sampler {
    fn id(&self) -> SamplerId {
        self.handle
    }
}

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

pub type MemoryBlock = daxa_sys::daxa_MemoryBlock;

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
            }
        };

        Option {
            data,
            has_value: true,
        }
    }
}

impl<T: 'static> Into<std::option::Option<T>> for Option<T> {
    fn into(self) -> std::option::Option<T> {
        let Option { has_value: true, .. } = &self else {
            return None;
        };
        Some(self.data)
    }
}

pub struct FixedString<const Capacity: usize> {
    data: [std::os::raw::c_char; Capacity],
    len: u8
}

impl<const Capacity: usize> FixedString {
    pub unsafe fn from_ptr(ptr: *const os::raw::c_char, len: usize) -> Self {
        let mut data = [Default::default(); Capacity];
        unsafe { std::ptr::copy(string.as_bytes(), &mut data, len) };
        Self {
            data,
            len,
        }
    }
    pub unsafe fn from_ptr_null_terminated(ptr: *const os::raw::c_char) -> Self {
        let mut len = 0;
        while *ptr.add(len) != 0 {
            len += 1;
        }
        Self::from_ptr(ptr, len)
    }
    
}

impl<const Capacity: usize> From<&'a str> for FixedString  {
    fn from(string: &'a str) -> Self {
        assert!(string.bytes().len() < Capacity);
        unsafe { Self::from_ptr(string.as_bytes(), string.bytes().len()) }
    }
}

pub type SmallString = FixedString<39>;

#[repr(u32)]
pub enum Format {
    UNDEFINED = daxa_sys::VkFormat_VK_FORMAT_UNDEFINED,
    R4G4_UNORM_PACK8 = daxa_sys::VkFormat_VK_FORMAT_R4G4_UNORM_PACK8,
    R4G4B4A4_UNORM_PACK16 = daxa_sys::VkFormat_VK_FORMAT_R4G4B4A4_UNORM_PACK16,
    B4G4R4A4_UNORM_PACK16 = daxa_sys::VkFormat_VK_FORMAT_B4G4R4A4_UNORM_PACK16,
    R5G6B5_UNORM_PACK16 = daxa_sys::VkFormat_VK_FORMAT_R5G6B5_UNORM_PACK16,
    B5G6R5_UNORM_PACK16 = daxa_sys::VkFormat_VK_FORMAT_B5G6R5_UNORM_PACK16,
    R5G5B5A1_UNORM_PACK16 = daxa_sys::VkFormat_VK_FORMAT_R5G5B5A1_UNORM_PACK16,
    B5G5R5A1_UNORM_PACK16 = daxa_sys::VkFormat_VK_FORMAT_B5G5R5A1_UNORM_PACK16,
    A1R5G5B5_UNORM_PACK16 = daxa_sys::VkFormat_VK_FORMAT_A1R5G5B5_UNORM_PACK16,
    R8_UNORM = daxa_sys::VkFormat_VK_FORMAT_R8_UNORM,
    R8_SNORM = daxa_sys::VkFormat_VK_FORMAT_R8_SNORM,
    R8_USCALED = daxa_sys::VkFormat_VK_FORMAT_R8_USCALED,
    R8_SSCALED = daxa_sys::VkFormat_VK_FORMAT_R8_SSCALED,
    R8_UINT = daxa_sys::VkFormat_VK_FORMAT_R8_UINT,
    R8_SINT = daxa_sys::VkFormat_VK_FORMAT_R8_SINT,
    R8_SRGB = daxa_sys::VkFormat_VK_FORMAT_R8_SRGB,
    R8G8_UNORM = daxa_sys::VkFormat_VK_FORMAT_R8G8_UNORM,
    R8G8_SNORM = daxa_sys::VkFormat_VK_FORMAT_R8G8_SNORM,
    R8G8_USCALED = daxa_sys::VkFormat_VK_FORMAT_R8G8_USCALED,
    R8G8_SSCALED = daxa_sys::VkFormat_VK_FORMAT_R8G8_SSCALED,
    R8G8_UINT = daxa_sys::VkFormat_VK_FORMAT_R8G8_UINT,
    R8G8_SINT = daxa_sys::VkFormat_VK_FORMAT_R8G8_SINT,
    R8G8_SRGB = daxa_sys::VkFormat_VK_FORMAT_R8G8_SRGB,
    R8G8B8_UNORM = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_UNORM,
    R8G8B8_SNORM = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_SNORM,
    R8G8B8_USCALED = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_USCALED,
    R8G8B8_SSCALED = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_SSCALED,
    R8G8B8_UINT = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_UINT,
    R8G8B8_SINT = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_SINT,
    R8G8B8_SRGB = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_SRGB,
    B8G8R8_UNORM = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_UNORM,
    B8G8R8_SNORM = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_SNORM,
    B8G8R8_USCALED = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_USCALED,
    B8G8R8_SSCALED = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_SSCALED,
    B8G8R8_UINT = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_UINT,
    B8G8R8_SINT = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_SINT,
    B8G8R8_SRGB = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_SRGB,
    R8G8B8A8_UNORM = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_UNORM,
    R8G8B8A8_SNORM = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_SNORM,
    R8G8B8A8_USCALED = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_USCALED,
    R8G8B8A8_SSCALED = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_SSCALED,
    R8G8B8A8_UINT = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_UINT,
    R8G8B8A8_SINT = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_SINT,
    R8G8B8A8_SRGB = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_SRGB,
    B8G8R8A8_UNORM = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_UNORM,
    B8G8R8A8_SNORM = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_SNORM,
    B8G8R8A8_USCALED = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_USCALED,
    B8G8R8A8_SSCALED = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_SSCALED,
    B8G8R8A8_UINT = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_UINT,
    B8G8R8A8_SINT = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_SINT,
    B8G8R8A8_SRGB = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_SRGB,
    A8B8G8R8_UNORM_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_UNORM_PACK32,
    A8B8G8R8_SNORM_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_SNORM_PACK32,
    A8B8G8R8_USCALED_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_USCALED_PACK32,
    A8B8G8R8_SSCALED_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_SSCALED_PACK32,
    A8B8G8R8_UINT_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_UINT_PACK32,
    A8B8G8R8_SINT_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_SINT_PACK32,
    A8B8G8R8_SRGB_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_SRGB_PACK32,
    A2R10G10B10_UNORM_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A2R10G10B10_UNORM_PACK32,
    A2R10G10B10_SNORM_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A2R10G10B10_SNORM_PACK32,
    A2R10G10B10_USCALED_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A2R10G10B10_USCALED_PACK32,
    A2R10G10B10_SSCALED_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A2R10G10B10_SSCALED_PACK32,
    A2R10G10B10_UINT_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A2R10G10B10_UINT_PACK32,
    A2R10G10B10_SINT_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A2R10G10B10_SINT_PACK32,
    A2B10G10R10_UNORM_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A2B10G10R10_UNORM_PACK32,
    A2B10G10R10_SNORM_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A2B10G10R10_SNORM_PACK32,
    A2B10G10R10_USCALED_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A2B10G10R10_USCALED_PACK32,
    A2B10G10R10_SSCALED_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A2B10G10R10_SSCALED_PACK32,
    A2B10G10R10_UINT_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A2B10G10R10_UINT_PACK32,
    A2B10G10R10_SINT_PACK32 = daxa_sys::VkFormat_VK_FORMAT_A2B10G10R10_SINT_PACK32,
    R16_UNORM = daxa_sys::VkFormat_VK_FORMAT_R16_UNORM,
    R16_SNORM = daxa_sys::VkFormat_VK_FORMAT_R16_SNORM,
    R16_USCALED = daxa_sys::VkFormat_VK_FORMAT_R16_USCALED,
    R16_SSCALED = daxa_sys::VkFormat_VK_FORMAT_R16_SSCALED,
    R16_UINT = daxa_sys::VkFormat_VK_FORMAT_R16_UINT,
    R16_SINT = daxa_sys::VkFormat_VK_FORMAT_R16_SINT,
    R16_SFLOAT = daxa_sys::VkFormat_VK_FORMAT_R16_SFLOAT,
    R16G16_UNORM = daxa_sys::VkFormat_VK_FORMAT_R16G16_UNORM,
    R16G16_SNORM = daxa_sys::VkFormat_VK_FORMAT_R16G16_SNORM,
    R16G16_USCALED = daxa_sys::VkFormat_VK_FORMAT_R16G16_USCALED,
    R16G16_SSCALED = daxa_sys::VkFormat_VK_FORMAT_R16G16_SSCALED,
    R16G16_UINT = daxa_sys::VkFormat_VK_FORMAT_R16G16_UINT,
    R16G16_SINT = daxa_sys::VkFormat_VK_FORMAT_R16G16_SINT,
    R16G16_SFLOAT = daxa_sys::VkFormat_VK_FORMAT_R16G16_SFLOAT,
    R16G16B16_UNORM = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_UNORM,
    R16G16B16_SNORM = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_SNORM,
    R16G16B16_USCALED = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_USCALED,
    R16G16B16_SSCALED = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_SSCALED,
    R16G16B16_UINT = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_UINT,
    R16G16B16_SINT = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_SINT,
    R16G16B16_SFLOAT = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_SFLOAT,
    R16G16B16A16_UNORM = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_UNORM,
    R16G16B16A16_SNORM = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_SNORM,
    R16G16B16A16_USCALED = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_USCALED,
    R16G16B16A16_SSCALED = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_SSCALED,
    R16G16B16A16_UINT = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_UINT,
    R16G16B16A16_SINT = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_SINT,
    R16G16B16A16_SFLOAT = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_SFLOAT,
    R32_UINT = daxa_sys::VkFormat_VK_FORMAT_R32_UINT,
    R32_SINT = daxa_sys::VkFormat_VK_FORMAT_R32_SINT,
    R32_SFLOAT = daxa_sys::VkFormat_VK_FORMAT_R32_SFLOAT,
    R32G32_UINT = daxa_sys::VkFormat_VK_FORMAT_R32G32_UINT,
    R32G32_SINT = daxa_sys::VkFormat_VK_FORMAT_R32G32_SINT,
    R32G32_SFLOAT = daxa_sys::VkFormat_VK_FORMAT_R32G32_SFLOAT,
    R32G32B32_UINT = daxa_sys::VkFormat_VK_FORMAT_R32G32B32_UINT,
    R32G32B32_SINT = daxa_sys::VkFormat_VK_FORMAT_R32G32B32_SINT,
    R32G32B32_SFLOAT = daxa_sys::VkFormat_VK_FORMAT_R32G32B32_SFLOAT,
    R32G32B32A32_UINT = daxa_sys::VkFormat_VK_FORMAT_R32G32B32A32_UINT,
    R32G32B32A32_SINT = daxa_sys::VkFormat_VK_FORMAT_R32G32B32A32_SINT,
    R32G32B32A32_SFLOAT = daxa_sys::VkFormat_VK_FORMAT_R32G32B32A32_SFLOAT,
    R64_UINT = daxa_sys::VkFormat_VK_FORMAT_R64_UINT,
    R64_SINT = daxa_sys::VkFormat_VK_FORMAT_R64_SINT,
    R64_SFLOAT = daxa_sys::VkFormat_VK_FORMAT_R64_SFLOAT,
    R64G64_UINT = daxa_sys::VkFormat_VK_FORMAT_R64G64_UINT,
    R64G64_SINT = daxa_sys::VkFormat_VK_FORMAT_R64G64_SINT,
    R64G64_SFLOAT = daxa_sys::VkFormat_VK_FORMAT_R64G64_SFLOAT,
    R64G64B64_UINT = daxa_sys::VkFormat_VK_FORMAT_R64G64B64_UINT,
    R64G64B64_SINT = daxa_sys::VkFormat_VK_FORMAT_R64G64B64_SINT,
    R64G64B64_SFLOAT = daxa_sys::VkFormat_VK_FORMAT_R64G64B64_SFLOAT,
    R64G64B64A64_UINT = daxa_sys::VkFormat_VK_FORMAT_R64G64B64A64_UINT,
    R64G64B64A64_SINT = daxa_sys::VkFormat_VK_FORMAT_R64G64B64A64_SINT,
    R64G64B64A64_SFLOAT = daxa_sys::VkFormat_VK_FORMAT_R64G64B64A64_SFLOAT,
    B10G11R11_UFLOAT_PACK32 = daxa_sys::VkFormat_VK_FORMAT_B10G11R11_UFLOAT_PACK32,
    E5B9G9R9_UFLOAT_PACK32 = daxa_sys::VkFormat_VK_FORMAT_E5B9G9R9_UFLOAT_PACK32,
    D16_UNORM = daxa_sys::VkFormat_VK_FORMAT_D16_UNORM,
    X8_D24_UNORM_PACK32 = daxa_sys::VkFormat_VK_FORMAT_X8_D24_UNORM_PACK32,
    D32_SFLOAT = daxa_sys::VkFormat_VK_FORMAT_D32_SFLOAT,
    S8_UINT = daxa_sys::VkFormat_VK_FORMAT_S8_UINT,
    D16_UNORM_S8_UINT = daxa_sys::VkFormat_VK_FORMAT_D16_UNORM_S8_UINT,
    D24_UNORM_S8_UINT = daxa_sys::VkFormat_VK_FORMAT_D24_UNORM_S8_UINT,
    D32_SFLOAT_S8_UINT = daxa_sys::VkFormat_VK_FORMAT_D32_SFLOAT_S8_UINT,
    BC1_RGB_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_BC1_RGB_UNORM_BLOCK,
    BC1_RGB_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_BC1_RGB_SRGB_BLOCK,
    BC1_RGBA_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_BC1_RGBA_UNORM_BLOCK,
    BC1_RGBA_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_BC1_RGBA_SRGB_BLOCK,
    BC2_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_BC2_UNORM_BLOCK,
    BC2_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_BC2_SRGB_BLOCK,
    BC3_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_BC3_UNORM_BLOCK,
    BC3_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_BC3_SRGB_BLOCK,
    BC4_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_BC4_UNORM_BLOCK,
    BC4_SNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_BC4_SNORM_BLOCK,
    BC5_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_BC5_UNORM_BLOCK,
    BC5_SNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_BC5_SNORM_BLOCK,
    BC6H_UFLOAT_BLOCK = daxa_sys::VkFormat_VK_FORMAT_BC6H_UFLOAT_BLOCK,
    BC6H_SFLOAT_BLOCK = daxa_sys::VkFormat_VK_FORMAT_BC6H_SFLOAT_BLOCK,
    BC7_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_BC7_UNORM_BLOCK,
    BC7_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_BC7_SRGB_BLOCK,
    ETC2_R8G8B8_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK,
    ETC2_R8G8B8_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK,
    ETC2_R8G8B8A1_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK,
    ETC2_R8G8B8A1_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK,
    ETC2_R8G8B8A8_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK,
    ETC2_R8G8B8A8_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK,
    EAC_R11_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_EAC_R11_UNORM_BLOCK,
    EAC_R11_SNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_EAC_R11_SNORM_BLOCK,
    EAC_R11G11_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_EAC_R11G11_UNORM_BLOCK,
    EAC_R11G11_SNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_EAC_R11G11_SNORM_BLOCK,
    ASTC_4x4_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_4x4_UNORM_BLOCK,
    ASTC_4x4_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_4x4_SRGB_BLOCK,
    ASTC_5x4_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x4_UNORM_BLOCK,
    ASTC_5x4_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x4_SRGB_BLOCK,
    ASTC_5x5_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x5_UNORM_BLOCK,
    ASTC_5x5_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x5_SRGB_BLOCK,
    ASTC_6x5_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x5_UNORM_BLOCK,
    ASTC_6x5_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x5_SRGB_BLOCK,
    ASTC_6x6_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x6_UNORM_BLOCK,
    ASTC_6x6_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x6_SRGB_BLOCK,
    ASTC_8x5_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x5_UNORM_BLOCK,
    ASTC_8x5_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x5_SRGB_BLOCK,
    ASTC_8x6_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x6_UNORM_BLOCK,
    ASTC_8x6_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x6_SRGB_BLOCK,
    ASTC_8x8_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x8_UNORM_BLOCK,
    ASTC_8x8_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x8_SRGB_BLOCK,
    ASTC_10x5_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x5_UNORM_BLOCK,
    ASTC_10x5_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x5_SRGB_BLOCK,
    ASTC_10x6_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x6_UNORM_BLOCK,
    ASTC_10x6_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x6_SRGB_BLOCK,
    ASTC_10x8_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x8_UNORM_BLOCK,
    ASTC_10x8_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x8_SRGB_BLOCK,
    ASTC_10x10_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x10_UNORM_BLOCK,
    ASTC_10x10_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x10_SRGB_BLOCK,
    ASTC_12x10_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x10_UNORM_BLOCK,
    ASTC_12x10_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x10_SRGB_BLOCK,
    ASTC_12x12_UNORM_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x12_UNORM_BLOCK,
    ASTC_12x12_SRGB_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x12_SRGB_BLOCK,
    G8B8G8R8_422_UNORM = daxa_sys::VkFormat_VK_FORMAT_G8B8G8R8_422_UNORM,
    B8G8R8G8_422_UNORM = daxa_sys::VkFormat_VK_FORMAT_B8G8R8G8_422_UNORM,
    G8_B8_R8_3PLANE_420_UNORM = daxa_sys::VkFormat_VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM,
    G8_B8R8_2PLANE_420_UNORM = daxa_sys::VkFormat_VK_FORMAT_G8_B8R8_2PLANE_420_UNORM,
    G8_B8_R8_3PLANE_422_UNORM = daxa_sys::VkFormat_VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM,
    G8_B8R8_2PLANE_422_UNORM = daxa_sys::VkFormat_VK_FORMAT_G8_B8R8_2PLANE_422_UNORM,
    G8_B8_R8_3PLANE_444_UNORM = daxa_sys::VkFormat_VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM,
    R10X6_UNORM_PACK16 = daxa_sys::VkFormat_VK_FORMAT_R10X6_UNORM_PACK16,
    R10X6G10X6_UNORM_2PACK16 = daxa_sys::VkFormat_VK_FORMAT_R10X6G10X6_UNORM_2PACK16,
    R10X6G10X6B10X6A10X6_UNORM_4PACK16 =
        daxa_sys::VkFormat_VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16,
    G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 =
        daxa_sys::VkFormat_VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16,
    B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 =
        daxa_sys::VkFormat_VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16,
    G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 =
        daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16,
    G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 =
        daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16,
    G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 =
        daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16,
    G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 =
        daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16,
    G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 =
        daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16,
    R12X4_UNORM_PACK16 = daxa_sys::VkFormat_VK_FORMAT_R12X4_UNORM_PACK16,
    R12X4G12X4_UNORM_2PACK16 = daxa_sys::VkFormat_VK_FORMAT_R12X4G12X4_UNORM_2PACK16,
    R12X4G12X4B12X4A12X4_UNORM_4PACK16 =
        daxa_sys::VkFormat_VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16,
    G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 =
        daxa_sys::VkFormat_VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16,
    B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 =
        daxa_sys::VkFormat_VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16,
    G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 =
        daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16,
    G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 =
        daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16,
    G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 =
        daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16,
    G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 =
        daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16,
    G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 =
        daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16,
    G16B16G16R16_422_UNORM = daxa_sys::VkFormat_VK_FORMAT_G16B16G16R16_422_UNORM,
    B16G16R16G16_422_UNORM = daxa_sys::VkFormat_VK_FORMAT_B16G16R16G16_422_UNORM,
    G16_B16_R16_3PLANE_420_UNORM = daxa_sys::VkFormat_VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM,
    G16_B16R16_2PLANE_420_UNORM = daxa_sys::VkFormat_VK_FORMAT_G16_B16R16_2PLANE_420_UNORM,
    G16_B16_R16_3PLANE_422_UNORM = daxa_sys::VkFormat_VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM,
    G16_B16R16_2PLANE_422_UNORM = daxa_sys::VkFormat_VK_FORMAT_G16_B16R16_2PLANE_422_UNORM,
    G16_B16_R16_3PLANE_444_UNORM = daxa_sys::VkFormat_VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM,
    G8_B8R8_2PLANE_444_UNORM = daxa_sys::VkFormat_VK_FORMAT_G8_B8R8_2PLANE_444_UNORM,
    G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16 =
        daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16,
    G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16 =
        daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16,
    G16_B16R16_2PLANE_444_UNORM = daxa_sys::VkFormat_VK_FORMAT_G16_B16R16_2PLANE_444_UNORM,
    A4R4G4B4_UNORM_PACK16 = daxa_sys::VkFormat_VK_FORMAT_A4R4G4B4_UNORM_PACK16,
    A4B4G4R4_UNORM_PACK16 = daxa_sys::VkFormat_VK_FORMAT_A4B4G4R4_UNORM_PACK16,
    ASTC_4x4_SFLOAT_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK,
    ASTC_5x4_SFLOAT_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK,
    ASTC_5x5_SFLOAT_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK,
    ASTC_6x5_SFLOAT_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK,
    ASTC_6x6_SFLOAT_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK,
    ASTC_8x5_SFLOAT_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK,
    ASTC_8x6_SFLOAT_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK,
    ASTC_8x8_SFLOAT_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK,
    ASTC_10x5_SFLOAT_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK,
    ASTC_10x6_SFLOAT_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK,
    ASTC_10x8_SFLOAT_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK,
    ASTC_10x10_SFLOAT_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK,
    ASTC_12x10_SFLOAT_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK,
    ASTC_12x12_SFLOAT_BLOCK = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK,
    PVRTC1_2BPP_UNORM_BLOCK_IMG = daxa_sys::VkFormat_VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG,
    PVRTC1_4BPP_UNORM_BLOCK_IMG = daxa_sys::VkFormat_VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG,
    PVRTC2_2BPP_UNORM_BLOCK_IMG = daxa_sys::VkFormat_VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG,
    PVRTC2_4BPP_UNORM_BLOCK_IMG = daxa_sys::VkFormat_VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG,
    PVRTC1_2BPP_SRGB_BLOCK_IMG = daxa_sys::VkFormat_VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG,
    PVRTC1_4BPP_SRGB_BLOCK_IMG = daxa_sys::VkFormat_VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG,
    PVRTC2_2BPP_SRGB_BLOCK_IMG = daxa_sys::VkFormat_VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG,
    PVRTC2_4BPP_SRGB_BLOCK_IMG = daxa_sys::VkFormat_VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG,
    R16G16_S10_5_NV = daxa_sys::VkFormat_VK_FORMAT_R16G16_S10_5_NV,
    A1B5G5R5_UNORM_PACK16_KHR = daxa_sys::VkFormat_VK_FORMAT_A1B5G5R5_UNORM_PACK16_KHR,
    A8_UNORM_KHR = daxa_sys::VkFormat_VK_FORMAT_A8_UNORM_KHR,
    ASTC_4x4_SFLOAT_BLOCK_EXT = daxa_sys::VkFormat_VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK_EXT,
    ASTC_5x4_SFLOAT_BLOCK_EXT = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK_EXT,
    ASTC_5x5_SFLOAT_BLOCK_EXT = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK_EXT,
    ASTC_6x5_SFLOAT_BLOCK_EXT = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK_EXT,
    ASTC_6x6_SFLOAT_BLOCK_EXT = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK_EXT,
    ASTC_8x5_SFLOAT_BLOCK_EXT = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK_EXT,
    ASTC_8x6_SFLOAT_BLOCK_EXT = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK_EXT,
    ASTC_8x8_SFLOAT_BLOCK_EXT = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK_EXT,
    ASTC_10x5_SFLOAT_BLOCK_EXT = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK_EXT,
    ASTC_10x6_SFLOAT_BLOCK_EXT = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK_EXT,
    ASTC_10x8_SFLOAT_BLOCK_EXT = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK_EXT,
    ASTC_10x10_SFLOAT_BLOCK_EXT = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK_EXT,
    ASTC_12x10_SFLOAT_BLOCK_EXT = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK_EXT,
    ASTC_12x12_SFLOAT_BLOCK_EXT = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK_EXT,
    G8B8G8R8_422_UNORM_KHR = daxa_sys::VkFormat_VK_FORMAT_G8B8G8R8_422_UNORM_KHR,
    B8G8R8G8_422_UNORM_KHR = daxa_sys::VkFormat_VK_FORMAT_B8G8R8G8_422_UNORM_KHR,
    G8_B8_R8_3PLANE_420_UNORM_KHR = daxa_sys::VkFormat_VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM_KHR,
    G8_B8R8_2PLANE_420_UNORM_KHR = daxa_sys::VkFormat_VK_FORMAT_G8_B8R8_2PLANE_420_UNORM_KHR,
    G8_B8_R8_3PLANE_422_UNORM_KHR = daxa_sys::VkFormat_VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM_KHR,
    G8_B8R8_2PLANE_422_UNORM_KHR = daxa_sys::VkFormat_VK_FORMAT_G8_B8R8_2PLANE_422_UNORM_KHR,
    G8_B8_R8_3PLANE_444_UNORM_KHR = daxa_sys::VkFormat_VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM_KHR,
    R10X6_UNORM_PACK16_KHR = daxa_sys::VkFormat_VK_FORMAT_R10X6_UNORM_PACK16_KHR,
    R10X6G10X6_UNORM_2PACK16_KHR = daxa_sys::VkFormat_VK_FORMAT_R10X6G10X6_UNORM_2PACK16_KHR,
    R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR =
        daxa_sys::VkFormat_VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR,
    G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR =
        daxa_sys::VkFormat_VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR,
    B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR =
        daxa_sys::VkFormat_VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR,
    G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR =
        daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR,
    G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR =
        daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR,
    G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR =
        daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR,
    G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR =
        daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR,
    G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR =
        daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR,
    R12X4_UNORM_PACK16_KHR = daxa_sys::VkFormat_VK_FORMAT_R12X4_UNORM_PACK16_KHR,
    R12X4G12X4_UNORM_2PACK16_KHR = daxa_sys::VkFormat_VK_FORMAT_R12X4G12X4_UNORM_2PACK16_KHR,
    R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR =
        daxa_sys::VkFormat_VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR,
    G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR =
        daxa_sys::VkFormat_VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR,
    B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR =
        daxa_sys::VkFormat_VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR,
    G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR =
        daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR,
    G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR =
        daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR,
    G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR =
        daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR,
    G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR =
        daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR,
    G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR =
        daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR,
    G16B16G16R16_422_UNORM_KHR = daxa_sys::VkFormat_VK_FORMAT_G16B16G16R16_422_UNORM_KHR,
    B16G16R16G16_422_UNORM_KHR = daxa_sys::VkFormat_VK_FORMAT_B16G16R16G16_422_UNORM_KHR,
    G16_B16_R16_3PLANE_420_UNORM_KHR =
        daxa_sys::VkFormat_VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM_KHR,
    G16_B16R16_2PLANE_420_UNORM_KHR = daxa_sys::VkFormat_VK_FORMAT_G16_B16R16_2PLANE_420_UNORM_KHR,
    G16_B16_R16_3PLANE_422_UNORM_KHR =
        daxa_sys::VkFormat_VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM_KHR,
    G16_B16R16_2PLANE_422_UNORM_KHR = daxa_sys::VkFormat_VK_FORMAT_G16_B16R16_2PLANE_422_UNORM_KHR,
    G16_B16_R16_3PLANE_444_UNORM_KHR =
        daxa_sys::VkFormat_VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM_KHR,
    G8_B8R8_2PLANE_444_UNORM_EXT = daxa_sys::VkFormat_VK_FORMAT_G8_B8R8_2PLANE_444_UNORM_EXT,
    G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT =
        daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT,
    G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT =
        daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT,
    G16_B16R16_2PLANE_444_UNORM_EXT = daxa_sys::VkFormat_VK_FORMAT_G16_B16R16_2PLANE_444_UNORM_EXT,
    A4R4G4B4_UNORM_PACK16_EXT = daxa_sys::VkFormat_VK_FORMAT_A4R4G4B4_UNORM_PACK16_EXT,
    A4B4G4R4_UNORM_PACK16_EXT = daxa_sys::VkFormat_VK_FORMAT_A4B4G4R4_UNORM_PACK16_EXT,
}

pub const VK_UUID_SIZE: usize = 16;

pub type VkPhysicalDeviceLimits = daxa_sys::VkPhysicalDeviceLimits;
pub type VkPhysicalDeviceSparseProperties = daxa_sys::VkPhysicalDeviceSparseProperties;

pub struct VkPhysicalDeviceProperties<'a> {
    api_version: u32,
    driver_version: u32,
    vendor_id: u32,
    device_id: u32,
    device_type: DeviceType,
    device_name: StringView<'a>,
    pipeline_cache_uuid: [u8; VK_UUID_SIZE],
    limits: VkPhysicalDeviceLimits,
    sparse_properties: VkPhysicalDeviceSparseProperties,
}
